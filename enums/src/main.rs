/* //enums allows us to enumerate a list of variants
//enums stores variants
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       //stores no data
    Move { x: i32, y: i32 },    //anonymous struct
    Write(String),              //only stores string
    ChangeColor(i32, i32, i32), //3 int
}
//methods and associated type can be declared on enums too
impl Message {
    fn some_function() {
        println!("Get Rusty");
    }
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {}
*/

//option enum
//many languages has null values
// in null we dont have null values instead we have option enum
fn main() {
    /*     enum Option<T> {
        Some(T),  //stores some values
        None,   //stores none values
    } */
    //Option enums and variants are included in our program scope by defualt
    let some_number = Some(5);
    let some_string = Some("A string");
    let absent_number: Option<i32> = None; //optional integer set to none
    //for NONE case we have to annotate the value that is being passed in option
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // we cannot directly mix optional type with other types
    let sum = x + y.unwrap_or(0);

    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //if-let syntax
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_value{
        println!("Three");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => None, //if any other syntax (no matches of other branches) then return None
    }
}
