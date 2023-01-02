// SUM NUMBERS
fn get_sum(x: u8, y: u8) -> u8 {
    return x + y;
}

// SUM VECTOR VALUES
fn sum_vector(vector: &Vec<u8>) -> u8 {
    let mut sum = 0;
    
    for value in vector.iter() {
        sum += value;
    }

    return sum;
}

// GENERIC FUNC
fn log<T: std::fmt::Display>(input: T) {
    println!("{}", input);
}

fn main() {

    // SUM NUMBERS
    let result = get_sum(5, 4);
    println!("NUM SUM: {}", result);
    
    // SUM VECTOR VALUES
    let my_vector: Vec<u8> = vec![1,2,3,4,5];
    let my_sum: u8 = sum_vector(&my_vector);
    println!("VECTOR SUM: {}", my_sum);
    log("FOO");
    log(55);
}



1:18

https://www.youtube.com/watch?v=ygL_xcavzQ4