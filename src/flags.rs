
use crate::traits::Flag;
use crate::element::Element;
use std::cell::RefCell;

/// The structure that holds all the user defined flags.
pub struct Flags<'a> {
    counter: RefCell<usize>,
    flags: RefCell<Vec<Element<'a>>>,
}

impl<'a> Flags<'a> {
    /// Initialize the Flags struct.
    pub fn new() -> Self {
        Self {
            counter: RefCell::new(0),
            flags: RefCell::new(Vec::new()),
        }
    }
    
    /// Adds a user defined flag in the flags struct and returns a mutable reference to the value
    /// that the user provided.
    /// # Examples
    /// ```
    /// // initialize the flags struct.
    /// use flagtory::Flags;
    /// let flags = Flags::new();
    ///
    /// // adds a flag named t to the flags struct. 
    /// let value_ref = flags.add("t","this is the t flag", 25);
    ///
    /// assert_eq!(*value_ref, 25);
    /// ```
    /// the value can be mutated by the user 
    /// ```
    /// use flagtory::Flags;
    /// // initialize the flags struct.
    /// let flags = Flags::new();
    ///  
    /// // add a flag named w to the flags struct.
    /// let value_ref = flags.add("w","this is the w flag", 65);
    /// 
    /// // modify the value 
    /// *value_ref = 55; 
    /// 
    /// assert_eq!(*value_ref, 55);
    ///
    /// ```
    ///
    pub fn add<T: Flag + 'static>(&self, name: &'a str, description: &'a str, value: T) -> &mut T {
        *self.counter.borrow_mut() += 1;
        
        let flag = Element::new(
            name,
            description,
            value
        );
    
        self.flags.borrow_mut().push(flag);
        
        return unsafe { &mut *self.flags.borrow_mut()[*self.counter.borrow()-1].get::<T>() };
    }
   
    /// parses the user argument provided in [`std::env::args`].
    /// Note that this method will find the flags that the user defined and change the value of the
    /// flag based on the argument provided.
    pub fn parse(&self) {
        let user_arguments = &std::env::args().collect::<Vec<String>>()[1..];    
    
        for (i,arg) in user_arguments.iter().enumerate() {
            let slice = &**arg;
    
            if slice.starts_with("--") == false && slice.starts_with("-") == false {
                continue;
            } 

            if let Some(s) = slice.strip_prefix("--") {
                
                if i+1 == user_arguments.len() {
                    if let Some(flag) = self.flags.borrow_mut().iter_mut().find(|x| x.name == s) {
                        if *flag.value.as_any().downcast_ref::<bool>().unwrap() == true  {
                            *flag.value.as_any().downcast_mut::<bool>().unwrap() = false; 
                        } else {
                            *flag.value.as_any().downcast_mut::<bool>().unwrap() = true; 
                        }
                    }
                } else {
                    if let Some(flag) = self.flags.borrow_mut().iter_mut().find(|x| x.name == s) {
                        let user_flag_value = &user_arguments[i+1];
                        flag.value.into_flag(&user_flag_value);
                    }
                }

            }

            if let Some(s) = slice.strip_prefix("-") { 
                // check if this flag has a value in it's right side  
                // e.g: -w 64 (a flag named w has a value 64) 
                // e.g: -b (a flag named b has no value, it is assumed to be a boolean flag) 
                
                // this is a boolean flag
                if i+1 == user_arguments.len() { 
                    // check the flag that has the name of the argument that the user provided 
                    if let Some(flag) = self.flags.borrow_mut().iter_mut().find(|x| x.name == s) {
                        if *flag.value.as_any().downcast_ref::<bool>().unwrap() == true  {
                            *flag.value.as_any().downcast_mut::<bool>().unwrap() = false; 
                        } else {
                            *flag.value.as_any().downcast_mut::<bool>().unwrap() = true; 
                        }
                    }
                } else {
                    // check the flag that has the name of the argument that the user provided 
                    if let Some(flag) = self.flags.borrow_mut().iter_mut().find(|x| x.name == s) {
                        let user_flag_value = &user_arguments[i+1];
                        flag.value.into_flag(user_flag_value);
                    }
                }

            }

        }

    }

}


