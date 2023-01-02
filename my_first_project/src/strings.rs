fn main() {

    // STRINGS
    let my_string = "foobar";
    println!("STRINGS USE DOUBLE QUOTES: {}", my_string);
    
    // CHARACTERS
    let my_char = 'A';
    println!("CHARS USE SINGLE QUOTES: {}", my_char);
    
    // MODIFY STRING
    let mut mutable = String::new();
    mutable.push('B');
    mutable.push('I');
    mutable.push('Z');
    mutable.push_str(" BAZ");
    println!("MUTABLE STRING: {}", mutable);
    
    // REPLACE PARTS
    let replaced = mutable.replace("B", "G");
    println!("REPLACED STRING: {}", replaced);
}