/*
Structs
A struct (short for "structure") is a custom data structure that lets you group related values together.

You can think of a struct like a mini-database for one thing, like a person with a name and age.

Create a Struct
You define a struct using the struct keyword and place the fields (variables) inside:
*/

fn main() {
    struct Person {
        name: String,
        age:i32,
        can_vote:bool,
    }

    let mut user = Person {
        name : String::from("John"),
        age:35,
        can_vote:true,
    };

    user.age =36;
    println!("Name : {}", user.name);
    println!("Age : {}", user.age); 
}