//Rust Tuples
//A tuple is a group of values of different types, stored in a single variable.

//Tuples are useful when you want to return or work with multiple values together.

//(Return a tuple from a function - spcial fucntion )
fn get_user() -> (String, i32){
    (String::from("Liam"), 25)
}


fn main(){
    //Create a tuple
    //Tuples are written using parenthesis (), with values separeted by commas:

    let person = ("John", 30, true);
    //This tuple contains a &str, an i32, and a bool.

    //Access Tuple Values
    //you can access tuple values by using a dot (.) followed by the index

    println!("name {}", person.0);
    println!("age {}", person.1);
    println!("active ? {}", person.2);

    //Unpacking a tuple
    //when we create a tuple , we normally assign values to it. This is called "packing" a tuple 
    //but in rust we are also allowed to extract the values back into variables. This is called "unpacking"

    let (name, age, active) = person;

    println!("name {}", name);
    println!("age {}", age);
    println!("active ? {}", active);

    //Return a tuple from a functin 
    let user = get_user();
    println!("User {} ({} years old)", user.0, user.1);
}