//OWNERSHIP
//Rust uses "ownership" to manage memory in a safe way.
//Every value in Rust has an owner. The owner is usually a variable.

// Ownership rules
// -> Each value has one owner
// -> When the owner goes out of scope, the value is deleted
// -> You can only have one owner at a time, unless you borrow it

//Basic examples
fn main(){
    let a = String::from("Hello");
    let b = a ;

    println!("{}", b); //b now owns the value

    /* when we assign a to b , the ownership moves, This means only b can use the value now,
    because a is no longer valid.
    but simple types like numbers, characters and booleans are copied, not moved.
    This means you can still use the original valirabel after assigning it to another:
    */

    let c = 5;
    let d = c;
    println!("c = {}", c );
    println!("d = {}", d );

    //here c is copied into d, not moved , so you can still use b

    //CLONE
    //For other types, like String, if you really want to keep the original value and also assign it to another variable, you can use the .clone() method, which makes a copy of the data:

    let e = String::from("hello");
    let f = e.clone();//now both have same value

    println!(" e= {}", e);
    println!("f = {}", f);
}