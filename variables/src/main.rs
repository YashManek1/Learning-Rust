fn main() {
    //VARIABLES
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six"; //Shadowing example
    println!("The value of x is: {}", x);
    const SUBS_COUNT: u32 = 100_000;
    println!("Subscribers count constant: {}", SUBS_COUNT);

    //DATA TYPES
    //1. Integers
    //2. Floating-Point Numbers
    //3. Boolean
    //4. Characters

    //Integers
    let a = 98_222; //i32(Default) //Decimal
    let b = 0xff; //Hex
    let c = 0o77; //Octal
    let d = 0b1111_0000; //Binary
    let e = b'A'; //Byte (u8 only)
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    //Floating-Point Numbers
    let f = 2.0; //f64(Default)
    let g: f32 = 3.0; //f32
    println!("f: {}, g: {}", f, g);

    //addition
    let sum = 5 + 10;
    //subtraction
    let difference = 95.5 - 4.3;
    //multiplication
    let product = 4 * 30;
    //division
    let quotient = 56.7 / 32.2;
    //remainder
    let remainder = 43 % 5;
    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}",
        sum, difference, product, quotient, remainder
    );

    //Boolean
    let t = true;
    let f: bool = false; //with explicit type annotation
    println!("t: {}, f: {}", t, f);

    //Character
    let z = 'z';
    let heart_eyed_cat = '😻';
    println!("z: {}, heart_eyed_cat: {}", z, heart_eyed_cat);

    //Compound Types
    //1. Tuples
    let tup = ("YT", 20);
    //Destructuring
    let (channel, sub_count) = tup;
    println!("Channel: {}, Sub Count: {}", channel, sub_count);
    //Dot notation
    let sub_count = tup.1;
    println!("Sub Count: {}", sub_count);

    //2. Arrays (Fixed length)
    let error_codes = [200, 404, 400];
    let not_found = error_codes[1];
    println!("Not found code: {}", not_found);

    let byte = [0; 8]; //creates an array of length 8 with all values 0
    println!("Byte array: {:?}", byte);

    //Functions
    //keyword is fn
    let sum = my_function(4, 5);
    println!("Sum: {}", sum);

    //Control flow
    //1. If-else
    let number = 5;
    if number > 10 {
        println!("Number greater than 10");
    } else if number < 5 {
        println!("Number less than 5");
    } else {
        println!("Number equals to 5");
    }

    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("X: {}", x);

    //Loops
    let mut count = 0;
    //1. Loop (loop keyword)
    let result = loop {
        //runs until break statement
        println!("Again!");
        count += 1;
        if count == 5 {
            break count; //adding count after break returns count
        }
    };
    println!("Result: {}", result);

    //2. While loop (will execute until a certain condition)
    let mut number = 3;
    while number != 0 {
        println!("Number still not zero: {}", number);
        number -= 1;
    }

    //3. For loop (useful when looping through a collection)
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        //.iter() gives an iterator for the array
        println!("The value is {}", element);
    }

    for number in 1..4 {
        //last number is exclusive (running loop through a range)
        println!("{}!", number);
    }

    //Comments
    //line comment
    /*Block comment */
}

//Function name should be all lowercase and where there is space there should be underscore (_) snake_case
fn my_function(x: i32, y: i32) -> i32 {
    println!("X: {}, Y: {}", x, y);
    let sum = x + y;
    sum
} //In function, last statement is implicitly returned and no need for semicolon in last line of function
