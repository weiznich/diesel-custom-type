# diesel_custom_type

Small utility crate to allow the usage of custom types with diesel.
This crate allows to add all needed trait implementations with a few lines of code

# This crate is deprecated

Use diesel's buildin support for custom types instead. For an example [see this testcase](https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs)

## Example
```rust
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_custom_type;

use diesel_custom_type::CustomSqlType;
use diesel::types::SmallInt;
use std::error::Error;

#[derive(Clone, Copy)]
#[repr(i16)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
}

impl CustomSqlType for Color {
    type DataBaseType = SmallInt;
    type RawType = i16;

    fn to_database_type(&self) -> i16 {
        *self as i16
    }

    fn from_database_type(v: &i16) -> Result<Self, Box<Error + Send + Sync>> {
        match *v {
            1 => Ok(Color::Red),
            2 => Ok(Color::Green),
            3 => Ok(Color::Blue),
            v => panic!("Unknown value {} for Color found", v),
        }
    } 
}

register_custom_type!(Color);


 struct User {
    name: String,
    hair_color: Option<Color>,
}

Queryable! {
    struct User {
        name: String,
        hair_color: Option<Color>,
    }
}

```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
