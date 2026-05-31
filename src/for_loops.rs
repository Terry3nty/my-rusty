fn main() {
    for i in 1..=6{
        println!("i is {}", i);
    }
    // if you want to include the last number , use ..=

    for i in 1..=10{
        if i==3{
            continue;
        }

        if i==5{
            break;
        }
        println!("i is {}", i)
    }
}