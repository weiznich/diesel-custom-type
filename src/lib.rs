//! Small utility crate to allow the usage of custom types with [diesel](http://diesel.rs)
//!
//! This crate allows to add all needed trait implementations with a few lines of code
//!
//!```
//!#[macro_use]
//!extern crate diesel;
//!#[macro_use]
//!extern crate diesel_custom_type;
//!
//!use diesel_custom_type::CustomSqlType;
//!use diesel::types::SmallInt;
//!use std::error::Error;
//!
//!#[derive(Clone, Copy)]
//!#[repr(i16)]
//!enum Color {
//!    Red = 1,
//!    Green = 2,
//!    Blue = 3,
//!}
//!
//!// Specify how the custom type should be converted
//!impl CustomSqlType for Color {
//!    type DataBaseType = SmallInt;
//!    type RawType = i16;
//!
//!    fn to_database_type(&self) -> i16 {
//!        *self as i16
//!    }
//!
//!    fn from_database_type(v: &i16) -> Result<Self, Box<Error + Send + Sync>> {
//!        match *v {
//!            1 => Ok(Color::Red),
//!            2 => Ok(Color::Green),
//!            3 => Ok(Color::Blue),
//!            v => panic!("Unknown value {} for Color found", v),
//!        }
//!    } 
//!}
//!
//!// Add all needed implements for diesel
//!register_custom_type!(Color);
//!
//!
//!// Use the type like every other type provided by diesel
//!table!{
//!    users{
//!        id -> Integer,
//!        name -> Text,
//!        hair_color -> Nullable<SmallInt>,
//!    }
//!}
//!
//!struct User {
//!    name: String,
//!    hair_color: Option<Color>,
//!}
//!
//!Queryable! {
//!    struct User {
//!        name: String,
//!        hair_color: Option<Color>,
//!    }
//!}
//!
//!
//!struct NewUser<'a> {
//!    name: &'a str,
//!    hair_color: Option<Color>,
//!}
//!
//!Insertable! {
//!    (users)
//!    struct NewUser<'a> {
//!        name: &'a str,
//!        hair_color: Option<Color>,
//!    }
//!}
//!
//!
//!# fn main(){}
//!
//!```
extern crate diesel;

use std::error::Error;

/// Trait indicating how to convert a custom type into a diesel known SQL-type
pub trait CustomSqlType: Sized {
    /// [Diesel type](http://docs.diesel.rs/diesel/types/index.html)
    type DataBaseType;
    /// Raw rust type corresponding to the diesel type
    type RawType;

    /// How to convert the custom type into the database type
    fn to_database_type(&self) -> Self::RawType;

    /// How to convert the database type into the custom type
    fn from_database_type(&Self::RawType) -> Result<Self, Box<Error + Send + Sync>>;
}

/// Macro to generate all needed trait implementations for diesel.
/// The macro assumes that `CustomSqlType` is implemented for your target type
#[macro_export]
macro_rules! register_custom_type {
    ( $Target:ty  ) => {


        impl <DB> ::diesel::types::ToSql<<$Target as CustomSqlType>::DataBaseType, DB> for $Target
        where $Target: CustomSqlType,
              DB: ::diesel::backend::Backend+ ::diesel::types::HasSqlType<<$Target as CustomSqlType>::DataBaseType>,
              <$Target as CustomSqlType>::RawType: ::diesel::types::ToSql<<$Target as CustomSqlType>::DataBaseType, DB>
        {
            fn to_sql<W: ::std::io::Write>(&self, out: &mut W) -> ::std::result::Result<::diesel::types::IsNull, Box<Error + Send + Sync>>{
                <$Target as CustomSqlType>::RawType::to_sql(&Self::to_database_type(self),out)
            }
        }

        impl<DB> ::diesel::types::FromSql<<$Target as CustomSqlType>::DataBaseType, DB> for $Target
            where $Target: CustomSqlType,
                  DB: ::diesel::backend::Backend+ ::diesel::types::HasSqlType<<$Target as CustomSqlType>::DataBaseType>,
                  <$Target as CustomSqlType>::RawType: ::diesel::types::FromSql<<$Target as CustomSqlType>::DataBaseType, DB>
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> ::std::result::Result<Self, Box<Error + Send + Sync>>{
                match <$Target as CustomSqlType>::RawType::from_sql(bytes) {
                    Ok(a) => Self::from_database_type(&a),
                    Err(e) => Err(e),
                }
            }
        }

        impl<DB> ::diesel::types::FromSqlRow<<$Target as CustomSqlType>::DataBaseType, DB> for $Target
        where DB: ::diesel::backend::Backend + ::diesel::types::HasSqlType<<$Target as CustomSqlType>::DataBaseType>,
              $Target: ::diesel::types::FromSql<<$Target as CustomSqlType>::DataBaseType, DB>
        {
            fn build_from_row<R: ::diesel::row::Row<DB>>(row: &mut R) -> ::std::result::Result<Self, Box<Error + Send + Sync>> {
                <$Target as ::diesel::types::FromSql<<$Target as CustomSqlType>::DataBaseType, DB>>::from_sql(row.take())
            }
        }


        impl ::diesel::expression::AsExpression<<$Target as CustomSqlType>::DataBaseType> for $Target {
            type Expression = ::diesel::expression::bound::Bound<<$Target as CustomSqlType>::DataBaseType, $Target>;

            fn as_expression(self) -> Self::Expression {
                ::diesel::expression::bound::Bound::new(self)
            }
        }

        impl<'a> ::diesel::expression::AsExpression<<$Target as CustomSqlType>::DataBaseType> for &'a $Target {
            type Expression = ::diesel::expression::bound::Bound<<$Target as CustomSqlType>::DataBaseType, &'a $Target>;

            fn as_expression(self) -> Self::Expression {
                ::diesel::expression::bound::Bound::new(self)
            }
        }

    };
}


