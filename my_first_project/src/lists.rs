fn main() {

    let my_list = [1,2,3,4,5,6];

    println!("FIRST: {}", my_list[0]);
    println!("LENGTH: {}", my_list.len());
    println!("LAST: {}", my_list[my_list.len() - 1]);

    // THIS DOESNT WORK WITH ARRAY, BUT DOES WITH VECTORS
    // my_list.push(7);

    // FOR EACH LOOP
    for value in my_list.iter() {
        println!("ARRAY VALUE: {}", value);
    }

    // IMMUTABLE NEW VECTORS
    // let vec_1: Vec<u8> = Vec::new();

    // MUTABLE VECTORS
    let mut my_vector: Vec<u8> = vec![1,2,3];

    // PUSHING DYNAMICALLY WORKS
    my_vector.push(4);
 
    for value in my_vector.iter() {
        println!("VECTOR VALUE: {}", value);
    }
}