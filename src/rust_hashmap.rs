//Rust Hashmap
/*
A HashMap is a collection of key/value pairs.

HashMaps are great when you want to store values and find them by a key.

To use HashMap, you must import it from Rust's standard library:
*?


/*
Create a HashMap
You can create a new, empty HashMap and add items to it:
*/
*/
use std::collections::HashMap;
fn main(){
    let mut capital_cities = HashMap::new();

    capital_cities.insert("England", "London");
    capital_cities.insert("Germany", "Berlin");
    capital_cities.insert("Norway", "Oslo");

    println!("{:?}", capital_cities);

    //Access Values: you can use .get()
    if let Some(city) = capital_cities.get("England"){
        println!("The capital of england is {}.", city);
    }else{
        println!("England is not in the map");
    }

    /*
    Update Values
If you insert a new value using a key that already exists, the old value is replaced with the new one:

Example
let mut capitalCities = HashMap::new();

capitalCities.insert("England", "London");
capitalCities.insert("England", "Berlin");

println!("{:?}", capitalCities);


Remove Values
To remove a key from a HashMap, use the .remove() method:

Example
let mut capitalCities = HashMap::new();

// Add keys and values (Country, City)
capitalCities.insert("England", "London");
capitalCities.insert("Germany", "Berlin");
capitalCities.insert("Norway", "Oslo");

// Remove the key "England"
capitalCities.remove("England");

println!("{:?}", capitalCities);
Loop Through a HashMap
You can use a for loop to go through all key/value pairs:

Example
let mut capitalCities = HashMap::new();

// Add keys and values (Country, City)
capitalCities.insert("England", "London");
capitalCities.insert("Germany", "Berlin");
capitalCities.insert("Norway", "Oslo");

// Loop through the HashMap
for (country, city) in &capitalCities {
  println!("The capital of {} is {}.", country, city);
}
Why Use HashMaps?
To store data by key
To quickly look up values
To group related data (like names and scores)
Note: HashMaps require keys to be unique. Inserting the same key again will overwrite the old value.
*/

}