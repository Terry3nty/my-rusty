//Borrowing and References
//sometimes you wanna use a value without taking ownership of it 
//Rust let's you do this using reference - this is called borrowing

//What is Reference ?
//A reference lets you look at a vlaue without owning it. create a reference using the & symbol
fn main(){
    let a =String::from("Hello world");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);


    let mut name = "John".to_string();
    let name_ref = &mut name;
    name_ref.push_str("Doe");

    println!("{}", name_ref);
}
/*
Why Borrowing is Important
Borrowing helps you reuse values safely, without giving them away.

It lets you use values without taking ownership
It avoids cloning, which can be slow for large data
It makes your programs safer and faster
*/