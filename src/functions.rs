//A function is a block of code that only runs when you call it.
// Functions are used to organize your code, avoid repeating yourself , and make your program easier to understand
fn main(){

    fn just_hi(){
        println!("hello there");
    }

    just_hi();

    fn say_hello() -> &'static str {
        "hello from a function"
    }

    let vrim = say_hello();

    println!("how to call a function, it's this way : {}", vrim);

    //Functions with parameters
    fn greet(name:&str){
        println!("Hello, {}", name);
    }

    greet("John");

    //Functions with return values
    fn add(a: i32, b:i32) -> i32{
        return a+b;
    }

    let sum = add(3,4);
    println!("{}",sum);
}