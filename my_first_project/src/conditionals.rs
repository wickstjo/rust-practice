fn main() {
    
    let first_age = 8;
    
    // BASIC IF STATEMENT
    if (first_age >= 1) && (first_age <= 18) {
        println!("IMPORTANT BIRTHDAY");
    } else {
        println!("INSIGNIFICANT");
    }

    let second_age = 21;

    // SWITCH MATCHING
    match second_age {

        // 1-18, INCLUSIVE
        1..=18 => println!("IMPORTANT"),

        // 21 OR 50
        21 | 50 => println!("SIGNIFICANT"),

        // FALLBACK WILDCARD
        _ => println!("NOT IMPORTANT"),
    };

    let third_age = 15;

    // TERNARY
    let can_vote = if third_age >= 18 { true } else { false };
    println!("IS ABLE TO VOTE: {}", can_vote);
}