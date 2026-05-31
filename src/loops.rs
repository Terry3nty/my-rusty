fn main() {
    let mut count = 1;


   let result = loop{
        println!("Hello world!");

        if count ==3{
            break count;
        }

        count +=1;
    };

    println!(" the loop stopped at {}", result);
}