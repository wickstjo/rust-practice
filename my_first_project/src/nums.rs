use rand::Rng;

fn main() {

    // UNSIGNED & SIGNED
    println!("MAX U8: {}", u8::MAX);
    println!("MAX I8: {}", i8::MAX);

    // MAX VALUES OF LARGERS
    println!("MAX U16: {}", u16::MAX);
    println!("MAX U32: {}", u32::MAX);
    println!("MAX U64: {}", u64::MAX);

    // F32 PRECISION
    let floats_one: f32 = 1.111111111111111;
    println!("FLOAT 32 PRECISION: {}", floats_one + 0.111111111111111);
    
    // F64 PRECISION
    let floats_two: f64 = 1.111111111111111;
    println!("FLOAT 64 PRECISION: {}", floats_two + 0.111111111111111);

    // RANDOM NUMBER -- 1-100
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("RANDOM NUMBER: {}", random_num);
}