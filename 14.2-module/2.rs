
#![allow(unused)]
fn main() {
// in lib.rs

// FILL in the blanks and FIX the errors
// You need make something public with `pub` to provide accessiblity for outside code `fn eat_at_restaurant()`
mod front_of_house {
    /* ...snip... */
}

pub fn eat_at_restaurant() {
    // call add_to_waitlist with **absolute path**:
    crate::front_of_house::hosting::add_to_waitlist();

    // call with **relative path** 
	front_of_house::hosting::add_to_waitlist();
}
}
