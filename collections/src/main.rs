//collections
//collections allow you to store multiple values in a single data structure.
//collections are allocated on the heap and can grow and shrink in size at runtime.
use unicode_segmentation::UnicodeSegmentation;

/*
fn main() {
    let a = [1, 2, 3]; //array
    let mut v: Vec<i32> = Vec::new(); //vector
    v.push(1);
    v.push(2);
    v.push(3);
    {
        let v2 = vec![1, 2, 3]; //vector with initial values
        //vector will be dropped when out of scope just like any other data structure on heap
    }
    //v2 is dropped
    let mut v3 = vec![1, 2, 3, 4, 5]; //vector with initial values using macro
    let third = &v3[2]; //accessing vector element using indexing //this will panic if index is out of bounds //immutable reference borrow
    //v3.push(6); //this would cause a compile time error because we have an immutable reference to v3, so we cannot borrow it as mutable, as new elements may cause reallocationc
    println!("The third element is {}", third);
    match v3.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    for i in &v3 {
        println!("{}", i);
    }
    for i in &mut v3 {
        *i += 50;
    }
    for i in &v3 {
        println!("{}", i);
    }
    //storing enums in vectors
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Float(10.12),
    ];
    //you have to use a match expression to figure out which variant of the enum it is
    match &row[0] {
        SpreadsheetCell::Int(i) => println!("Int:{}", i),
        _ => {
            println!("Not an Int");
        }
    }
}
*/
/*
fn main(){
    //Strings
    //Strings are complicated in Rust as this a low level programming language
    //Strings are stored as a collection of UTF-8 encoded bytes
    //defining strings
    let s1=String::new(); //defining an empty string
    let s2="initial contents";//string slices
    let s3=s2.to_string();//converting string slices to string
    let s4=String::from("Hello");//converting string slices to string using from function(this does s2 and s3 sathmai)
    //strings can be written in all languages
    let mut s=String::from("Hello");
    //string can grow and shrink in size
    s.push_str("bar"); //this takes string slices as we dont want to take ownership
    s.push('!');//characters can also be added in the end of string but only single quotes (Syntax)
    //Hellobar!
    let s1=String::from("Hello");
    let s2=String::from(" World!");
    //let s3:String=s1+&s2; //moving ownership of s1 to s3 and taking all the characters of s2 and appending them at the end of s3
    //this saves a little bit of memory than copying string
    //as now ownership of s1 is moved, we cant access it now
    //println!("{}",s1);

    let s3=format!("{}{}",s1,s2);//format doesnt take ownership of strings so s1 and s2 can still be accessed
    //indexing of string
    let hello:String=String::from("Hello");
    let example=String::from("नमस्ते");
    //let h=hello[0]; //this will give error as strings cannot be indexed
    //this is because strings are stored as a collection of UTF-8 encoded bytes
    //नमस्ते is stored as
    //[224,164,168,224,164,174,224,165,135,224,164,164,224,165,135]
    //so if we try to access the first byte it will give us 224 which is not a valid character
    //so we can use bytes method to access the bytes of the string
    println!("Bytes of नमस्ते:");
    for b in example.bytes(){
        println!("{}",b);
    }

    //scalar values
    //a string is a collection of bytes so accessing a particular character is not that simple
    //['न','म','स','्','त','े']
    println!("Characters of नमस्ते:");
    for c in example.chars(){
        println!("{}",c);
    }

    //Grapheme clusters
    //a grapheme cluster is a collection of one or more code points that represent a single
    //character
    //["न","म","स","त"]
    println!("Grapheme clusters of नमस्ते:");
    for g in example.graphemes(true){
        println!("{}",g);
    }
    //true indicates that we want to use the extended grapheme clusters
    let slice=&example[0..3]; //slicing the string using byte indices //has to be exact size of the number of bytes each character takes like न takes 3 bytes so we have to slice from 0 to 3
    println!("Sliced string: {}",slice);
} */

use std::collections::HashMap;

fn main() {
    //hashmaps
    //hashmaps allow you to store a collection of key-value pairs which can be of different types
    //it uses a hashing function to determine where to store the key-value pair in memory
    //keys to be team name
    //values to be score
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10); //we are not passing reference of blue and yellow so they are moved here and cannot be used later
    //passing of ownership is done to avoid cloning of data which can be expensive in terms of memory and performance
    scores.insert(yellow, 50);
    let blue_score = scores.get("Blue"); //get method returns an Option<&V>
    match blue_score {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("Blue team not found"),
    }
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores2: HashMap<String, i32> = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25); //this will overwrite the previous value
    scores2.entry(String::from("Yellow")).or_insert(20); //if key is not present insert the value
    scores2.entry(String::from("Blue")).or_insert(50); //if key is not present insert the value
    for (key, value) in &scores2 {
        println!("{}: {}", key, value);
    }
    //map.entry returns an enum representing the value at that key which may be either Vacant or Occupied
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    //["hello", "world", "wonderful","world"]
    //1st iteration:hello, new key is created with value 0, count is incremented to 1 (count is a mutatble reference to the value i.e hello)
    //2nd iteration:world, new key is created with value 0, count is incremented to 1
    //3rd iteration:wonderful, new key is created with value 0, count is incremented to 1
    //4th iteration:world, key is already present with value 1, count is incremented to 2 (mutable reference to the value i.e world) so changing the mutatble reference changes the value in the hashmap
    //each word created a new mutable reference to its value in the hashmap
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //if the word is not present insert 0
        *count += 1; //increment the count
    }
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
