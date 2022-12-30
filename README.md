

[![Crates.io](https://img.shields.io/crates/v/flagtory.svg)](https://crates.io/crates/flagtory)
[![Documentation](https://img.shields.io/docsrs/flagtory/latest)](https://docs.rs/flagtory/latest/flagtory/)
[![dependency status](https://deps.rs/repo/github/akbarilmnn/flagtory/status.svg)](https://deps.rs/repo/github/akbarilmnn/flagtory)

# Flagtory

a simple library to make command line flags.
This library provides the user to make their own command line flags easily.
This library's API model is inspired by Go's [flag](https://pkg.go.dev/flag) package which has a simple interface to the users.

## Examples

```rust
    // bring the module into scope     
    use flagtory::Flags;

    // initialize the flags struct
    let flags = Flags::new();

    // add your flags using the add method 
    // this returns a mutable reference to the value 
    let bool_ref = flags.add("f","this is my flag", true);
    
    assert_eq!(*bool_ref, true);

    // you can change the value as long it's the same type 
    *bool_ref = false;

    assert_eq!(*bool_ref, false);
```

Currently, flagtory only accept values that implements the [std::str::FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) and [std::any::Any](https://doc.rust-lang.org/std/any/trait.Any.html) trait. Which is wrapped by the [Flag](https://docs.rs/flagtory/latest/flagtory/trait.Flag.html) trait which is why `&str` is not implemented in the Flag trait, use `String` instead.
and there are some limitations around how a user can create their own flags which is parsable. See [here](https://docs.rs/flagtory/latest/flagtory/)
Please use the latest version to get the best experience.

