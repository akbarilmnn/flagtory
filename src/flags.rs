
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
        // the reason why i need to slicee it because the first argument is the path to the
        // executable meanwhile the rest is what the user is going to type.
        let provided_arguments = &std::env::args().collect::<Vec<String>>()[1..];
    
    
        // iterate through all the provided_arguments 
        for (index,argument) in provided_arguments.iter().enumerate() {
            let arg = &**argument;
            // check if this is not a flag 
            if arg.starts_with("--") == false && arg.starts_with("-") == false {
                continue;
            } 
    
            if let Some(valid_name) = arg.strip_prefix("--") {
                
                // this is basically checking if the flag in the end of the user provided argument
                // has no value like the -b flag for example`-w 88 -h 88 -b`
                // this b flag is assumed to be a boolean flag same as name if the order is like
                // this 
                // -w 88 -b -h 88
                if index + 1 == provided_arguments.len() {
                    if let Some(flag) = self.flags.borrow_mut().iter_mut().find(|x| x.name == valid_name.split("-").collect::<Vec<&str>>().join(""))  {
                        *flag.value.as_any().downcast_mut::<bool>().unwrap() = !(*flag.value.as_any().downcast_mut::<bool>().unwrap());
                    }
                } else { 
                    // this is basically turn the user defined flag comparable to the real flag 
                    // e.g: defined as `allow net`
                    // e.g: user argument as `--allow-net`
                    // if it succeeded to strip it prefix it will turn into allow-net then we must
                    // split it by the dash to [allow,net] and then join it by using a space to
                    // allow net.
                    if let Some(flag) = self.flags.borrow_mut().iter_mut().find(|x| x.name == valid_name.split("-").collect::<Vec<&str>>().join(""))  {
                        // check if the flags value is another flag (this means the flag is assumed to be  a boolean flag)
                        let user_provided_value = &*provided_arguments[index+1];
                        if user_provided_value.starts_with("--") == false && user_provided_value.starts_with("-") == false {
                            flag.value.into_flag(user_provided_value);
                        } else {
                                *flag.value.as_any().downcast_mut::<bool>().unwrap() = !(*flag.value.as_any().downcast_mut::<bool>().unwrap());
                        }
                    }
                }

            } 

            if let Some(valid_name) = arg.strip_prefix("-") {
                if index+1 == provided_arguments.len() {
                    if let Some(flag) = self.flags.borrow_mut().iter_mut().find(|x| x.name == valid_name) {
                        *flag.value.as_any().downcast_mut::<bool>().unwrap() = !(*flag.value.as_any().downcast_mut::<bool>().unwrap());
                    }
                } else {
                    if let Some(flag) = self.flags.borrow_mut().iter_mut().find(|x| x.name == valid_name) {
                        let user_provided_value = &*provided_arguments[index+1];
                        if user_provided_value.starts_with("--") == false && user_provided_value.starts_with("-") == false {
                            flag.value.into_flag(user_provided_value);
                        } else {
                                *flag.value.as_any().downcast_mut::<bool>().unwrap() = !(*flag.value.as_any().downcast_mut::<bool>().unwrap());
                        }
                    }
                }
            }


        }

    }     

}


