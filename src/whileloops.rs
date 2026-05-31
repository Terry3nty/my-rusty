fn main(){
    let mut count = 1;

    while count <=5{
        println!("count: {}", count);
        count +=1
    }

    let mut num = 1;

    while num <= 10{
        if num == 6 {
            break;
        }
        println!("the stuff stopped {}", num);
        num +=1;
    }

    let mut vrim = 1;

    while vrim <= 10 {
        if vrim == 6 {
            vrim += 1;
            continue;
        }

        println!("Number {}", vrim);
        vrim += 1;
    }
}