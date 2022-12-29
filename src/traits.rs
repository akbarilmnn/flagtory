
use std::any::Any;
use std::str::FromStr;


pub trait Flag {
    fn into_flag(&mut self, s: &str);
    fn as_any(&mut self) -> &mut dyn Any;
}

// the reason why iam not doing an implementation of the trait for each 
// type that is allowed, is just for convenience
impl<T> Flag for T 
    where T: FromStr + 'static
{
    fn into_flag(&mut self, s: &str) {
        if let Ok(parsable_value) = s.parse::<T>() {
            *self = parsable_value;
        } else {
            // alternative error message.
            // panic!("Cannot parse this type from a String that has the value of `{}`!\nPlease Check your flag type again to be parsable.", s);
            eprintln!("Cannot parse type `{}` from String that has the value `{}`", std::any::type_name::<T>(), s);
            eprintln!("Process exited with status 1 -> Failed");

            // exit a running process
            std::process::exit(1);
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
