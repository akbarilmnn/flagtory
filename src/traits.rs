
use std::any::Any;
use std::str::FromStr;

/// The trait that needs to be implemented in order to be a flag
/// anything that implements FromStr and has a static lifetime implements this trait.
pub trait Flag {
    fn into_flag(&mut self, s: &str);
    fn as_any(&mut self) -> &mut dyn Any;
}

impl<T> Flag for T 
    where T: FromStr + 'static
{
    fn into_flag(&mut self, s: &str) {
        if let Ok(parsable_value) = s.parse::<T>() {
            *self = parsable_value;
        } else {
            // error message 
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

