//Rust Vectors 
//A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.

fn main(){
    let mut fruits = vec!["apple", "banana", "orange"];

    //Access Vector Elements
    // you can access values in a vector using index number (just like arrays)

    println!("First fruit : {}", fruits[0]);

    //Change vector values
    //to change a value in the vector, refer to the index number and assign a new value.
    //remember to make the vector mutable(using mut keyword)

    fruits[0] = "grape";
    println!("New fruit: {}", fruits[0]);

    //Add and remove and element from a vector
    // add with .push() and remove with .pop()

    fruits.push("cherry");
    println!("{:?}", fruits);

    fruits.pop();
    println!("{:?}", fruits);

    //add or remove at specific index
    // use .insert to add and .remove() to remove

    fruits.insert(0, "kiwi");
    println!("{:?}", fruits);

    fruits.insert(1, "watermelon");
    println!("{:?}", fruits);

    //to remove
    fruits.remove(0);
    println!("{:?}", fruits);


    //vector length 
    //you can find out how many element are there in a vector using .len() method

    println!("There are {} fruits", fruits.len());

    //loop through a vector 
    // just like arrays, you can use a for loop to go through all the calues in a vector:

    for fruit in &fruits{
        println! ("I like {}", fruit);
    }

}