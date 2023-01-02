fn main() {

    // CAN HAVE ANY AMOUNT OF ITEMS
    let my_tuple: (u8, String) = (55, "John".to_string());

    // DIRECT REFERENCING
    println!("THE AGE: {}", my_tuple.0);
    println!("THE NAME: {}", my_tuple.1);
    
    // DESTRUCTURING
    let (age, name) = my_tuple;
    println!("THE AGE: {}", age);
    println!("THE NAME: {}", name);
}