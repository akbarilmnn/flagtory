
use std::any::Any;
use std::str::FromStr;


pub trait Flag {
    fn into_flag(&mut self, s: &str);
    fn as_any(&mut self) -> &mut dyn Any;
}



#[macro_export]
macro_rules! impls {
    ($($type:ident), *) => {
        $(
            impl Flag for $type {
                fn into_flag(&mut self, s: &str) {
                    if let Ok(value) = s.parse::<$type>() {
                        *self = value;
                    } else {
                        eprintln!("cannot parse it");
                    }
                }
                fn as_any(&mut self) -> &mut dyn Any {
                    self
                }
            } 
         )*
    }
}


crate::impls!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
