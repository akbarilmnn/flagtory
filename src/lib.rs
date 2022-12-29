
//! a simple library to make command line flags.
//! This library provides the user to make their own command line flags easily.
//! though there are some limitations on how to create a flag that is parsable by the
//! [`flags::Flags::parse`] method.
//! 
//! - **if the flag name has only 1 character, the user must use a one dash flag**:
//!     e.g: -w, -p, -v.
//! - **if the flag name has more than 1 character, the user must use a double dash flag**
//!     e.g: --bin, --lib, --http.
//! - **if the flag name has more than 1 character and has a space delimiter, the delimeter must be changed into a dash**
//!     e.g: --allow-net.
//!
//!     flagtory will assume that if you register a flag using the [`flags::Flags::add`] method and
//!     the name of the flag is 1 characters long it will match to the user provided flags that has
//!     one dash flag, same as the double dash flag it the name of the flag is more than a
//!     character.

mod flags;
mod traits;
mod element;

pub use traits::Flag;
pub use element::Element;
pub use flags::Flags;
