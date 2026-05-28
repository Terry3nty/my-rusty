fn main(){
    if 7>5{
        println!("7 is greater than 5");
    }

    let age = 16;

    if age >= 18{
        println!("you can vote ");
    } else {
        println!("You are too young to vote");
    }

    let score = 85;

    if score >= 90{
        println!("Grade : A");
    } else if score >= 80 {
        println!("Grade : B");
    } else if score >= 70 {
        println!("Grade : C");
    } else {
        println! ("Grade : F");
    }

    let time = 20;
    let greeting = if time < 18 {
        "Fuck you, have a good day"
    } else {
        "Good evenning"
    };

    print!("\n {}", greeting);

    /*
    Note: The value from if and else must be the same type, like two pieces of text or two numbers (in the example above, both are strings).

When you mix types, like a string and an integer, you'll get an error:*/
}