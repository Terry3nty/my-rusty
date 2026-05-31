//strings are used to store text.
//there are two types of strings in Rust 
// &str and String

//CREATE A STRING
//you can create a String from a string literal using the to_string() method or String::from()

fn main (){
    let text1 = "Hello World".to_string();

    let text2 = String::from("Hello worlds");

    println!("{} \n {}", text1, text2);

    //change a string we use push_str()

    let mut greeting = String::from("Hello");
    greeting.push_str("World");

    //to add just one character we use push()

    let mut word = String::from("hi");
    word.push('!');

    println!("{} \n {}", greeting, word);


    //Concatenate Strings
    //to combine strings we use the format! macro:

    let s1 = String::from("Hellos");
    let s2 = String::from("World");
    let s3 = String::from("What a beautifull day ");

    let result = format!("{} {} {}", s1 ,s2, s3);
    let result2 = s1 + "" + &s2 + "" + &s3;

    println! ("{} \n {}", result, result2);

    //string length, we use .len()

    let squish = String::from("John");
    println! ("{}", squish.len());
}