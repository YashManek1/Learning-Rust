//Ownership is rust most unique feature
//It enables Rust to make memory safe without a garbage collector

// Ownership model is way to manage memory in Rust
// Ownership model Pros
// 1. Control over memory
// 2. Error free*
// 3. Faster runtime
// 4. Smaller program size
// Ownership model Cons
// 1. Steep learning curve(fighting the borrow checker)
// 2. Slower write time

// Memory is stored in stack and heap
// Stack: Fixed size, cannot grow or shrink during runtime
// Stack also has stack frames for each function call, which stores local variables and function parameters, size is calculated during compile time
// Heap: Dynamic size, can grow and shrink during runtime
// Heap is used to store data that are dynamic in size, could be large amounts of data and we control the lifetime of data

// Heap stores the actual data, while stack stores the pointer to the data in heap
//Pushing on stack is faster than allocating on heap
//Accessing data on stack is faster than accessing data on heap

// Ownership Rules
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    //String literals are directly stored in binary and are immutable
    {
        //s is not valid here, it's not yet declared
        let s = "hello"; //s is valid from this point forward (this is a string literal)
        //do stuff with s
    } //this scope is now over, and s is no longer valid

    {
        //s is not valid here, it's not yet declared
        let mut s = String::from("hello"); //s is valid from this point forward (this is a String type, stored in heap)
        s.push_str(", world!"); //push_str() appends a literal to a String
        println!("{}", s); //this will print "hello, world!"
    }

    //How variables and data interact with ownership
    let x = 5; //x comes into scope
    let y = x; //x is copied into y, because i32 is a Copy (Copy)

    let s1 = String::from("hello");
    let s2 = s1; //s1 is moved to s2, s1 is no longer valid //Move(Not a shallow copy)
    //println!("{}", s1); //This will throw an error because s1 is no longer valid

    let s3 = String::from("hello");
    let s4 = s3.clone(); //s3 is cloned to s4, both are valid
    println!("s3 = {}, s4 = {}", s3, s4);

    //Ownership and Functions
    let s = String::from("hello"); //s comes into scope
    takes_ownership(s); //s's value MOVES into the function...
    //println!("{}", s); //... and so is no longer valid here

    let x = 5; //x comes into scope
    makes_copy(x); //x would move into the function, but i32 is Copy, so it's okay to still use x afterward
    println!("{}", x);

    let s5 = gives_ownership(); //gives_ownership moves its return value into s5
    println!("{}", s5);

    let s6 = gives_ownership(); //gives_ownership moves its return value into s6
    let s7 = String::from("hello"); //s7 comes into scope
    let s8 = takes_and_gives_back(s7); //s7 is moved into takes_and_gives_back, which also moves its return value into s8
    println!("s6 = {}, s8 = {}", s6, s8);

    //References
    let s9 = String::from("hello");
    let len = calculate_length(&s9); //We pass a reference
    //References don't take ownership of the value they refer to
    println!("The lenght of {} is {}", s9, len);

    let mut s10 = String::from("hello");
    let length = calculate_mutlength(&mut s10);
    println!("The length of {} is {}", s10, length);

    //Dangling References
    //A dangling reference is a reference that points to data that has been dropped.
    //Rust prevents dangling references at compile time.

    //Slices
    //Slices let you reference a contiguous sequence of elements in  a collection rather than the whole collection.
    let s11 = String::from("hello world");
    let hello = &s11[0..5]; //slice from index 0 to 5 (not including 5)
    let world = &s11[6..11]; //slice from index 6 to 11 (not including 11)
    let hello = &s11[..5]; //if starting at 0, we can skip it
    let world = &s11[6..]; //if going to the end, we can skip it

    //String literals are String slices
    //String references are automically coerced to string literals which are string slices
    println!("{} {}", hello, world);

    //Slices can also be used with arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; //slice from index 1 to 3
    println!("{:?}", slice);
}

fn takes_ownership(some_string: String) {
    //some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed. 

fn makes_copy(some_integer: i32) {
    //some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string //some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    //a_string comes into scope
    a_string //a_string is returned and moves out to the calling function
} // Here, a_string goes out of scope. Nothing special happens.

//Passing the references as function parameters is known as Borrowing
//References are immutable by default
//We can also have mutable references
fn calculate_length(s: &String) -> usize {
    let len = s.len();
    len
}

//mutable reference has a big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.
//You cannot have a mutable reference while you have an immutable reference to the same data.
//Scope of a reference starts when its first introduced and ends when it is last used.
fn calculate_mutlength(s: &mut String) -> usize {
    s.push_str(" world");
    let len = s.len();
    len
}

//The Rules of References
//1. At any given time, you can have either one mutable reference or any number of immutable references.
//2. References must always be valid.
