// when you have many choices, using match is eaiser than writing a lots of if ..else
//match is used to selevet one of many code blocks to be executed

fn main(){
    let day = 4;

    match day {
        1 => println!("monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("thursday"),
        5 => println!("friday"),
        6 => println!("saturday"),
        7 => println!("sunday"),
        _ => println!("Invalid day."),

    

    }

     
    let result = match day {
        1 | 2 | 3 | 4 | 5 =>"Weekday",
        6 | 7 => "Weekend",
        _ => "Invalid day",
    };
    println!("{}", result)
}