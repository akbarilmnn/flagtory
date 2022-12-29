
use crate::traits::Flag;

/// The structure for the Flag;
pub struct Element<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub value: Box<dyn Flag>
}

impl<'a> Element<'a> {
    pub fn new(name: &'a str, description: &'a str, value: impl Flag + 'static) -> Self {
        Self {
            name,
            description,
            value: Box::new(value)
        }
    }
    
    pub fn get<T: 'static>(&mut self) -> *mut T {
        self.value.as_any().downcast_mut::<T>().unwrap() as *mut T
    }
}
