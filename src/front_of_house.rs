
// MODULE - FRONT_OF_HOUSE
// Creating a Module file to separate functionality
// Note that since we named this file as front_of_house, we can write the "hosting" block 
// of code directly we do not need to embed "hosting" inside the front_of_house block
// anymore

// You can make a module public with pub keyword
// Making module public doesn't automatically make the functions inside
// automatically public
//Making module public only lets code in its ancestor module refer to it
pub mod hosting {
    //default functions are private in modules. Child can access parent's function
    //even if they are private. Parent cannot access Child's functions which
    // are private
    // functions are part of the module tree structures in Rust
    // Path in RUST crate -> modules -> functions, variables etc
    
    pub fn add_to_waitlist() {}
    //child components can access private parent componenets
    // parent cannot access private child components
    // child components are defined in the context of parent so they have access to
    // the parent variables
    fn seat_at_table() {}
}

pub mod serving {
    pub fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}