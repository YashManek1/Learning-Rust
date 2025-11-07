// //command to create a lib with cargo new instead of main
// //cargo new --lib project-name

// //modules are specified by mod keyword
// //mod name_of_module {}
// //modules can contain other modules inside them
// //also can contain structs,enum,constants,traits and so on
// mod front_of_the_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }
// //by default child module and everything inside it is private and cannot be accessed by parent module in Rust
// //child module can see anything defined in their parent module

// //if you want to reference an itema in your module tree
// //you would need to specify the path to that function
// pub fn eat_at_restaurant() {
//     //Absolute path
//     crate::front_of_the_house::hosting::add_to_waitlist();
//     //relative path (start from the current module)
//     front_of_the_house::hosting::add_to_waitlist();
// }

// fn server_order(){}

// mod back_of_house{
//     fn fix_incorrect_order(){
//         cook_order();
//         super::server_order(); //super allows us to reference the parent module which in this case is crate
//     }

//     fn cook_order(){}
// }

//Privacy rules in Rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Toast");
    meal.toast = String::from("Wheat");
}

mod back_of_house2 {
    pub enum Appetizer {
        //by default if enum is marked public all its variants are marked public as well (this doesnt happen in structs as seen above, the seasonal_fruits is private)
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let order1 = back_of_house2::Appetizer::Soup;
    let order2 = back_of_house2::Appetizer::Salad;
}

/* use rand::Rng;
use rand::CryptoRng;
use rand::ErrorKind::Transient; */

use rand::{CryptoRng, ErrorKind::Transient, Rng};

/* use std::io;
use std::io::Write; */

use std::io::{self,Write};

use std::io::*; //brings everything into scope (everything public under std::io)

mod front_of_the_house;

//use keyword
//use allows you to bring a path into scope
pub use crate::front_of_the_house::hosting; //absolute path //using use keyword makes it so that external files can also use hosting module (Export)
//use self::front_of_the_house::hosting; //relative path

pub fn eat_at_restaurant3() {
    /*  front_of_the_house::hosting::add_to_waitlist();
    front_of_the_house::hosting::add_to_waitlist();
    front_of_the_house::hosting::add_to_waitlist(); */
    let secret_number = rand::thread_rng().gen_range(1, 101);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

//in rust the general way to bring functions into scope is to bring the function's parent module
//this specifies that the function is not a local function but part of another module
//while bringing structs,enums or other items into scope its idiomatic to specify the full path
//exception of this is that suppose you are bringing two structs from two diff items into scope and they have the same name
//in this case bring parent module into scope as the name wont collide in this way

//example
/* use std::fmt;
use std::io; */
//brought parent module into scope
/* fn function1()->fmt::Result{
    //...
}

fn function2()->io::Result<()>{
    //...
} */

//using as keyword
//another way is to rename one of the Result types when bringing it into scope
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    //..
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

//we can declare modules using mod keyword
//name of the file should be same if trying to bring module into scope
//module name and file name should be same and then semi-colon(syntax)
