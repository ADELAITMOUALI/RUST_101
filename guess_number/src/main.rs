use std::io;
use rand::Rng;
fn main() {
    let the_number = rand::rng().random_range(1..101);
    println!("{}",the_number);

    let mut user_input = String::new();
    println!("Guess a number between 1 and 100");
    io::stdin().read_line(&mut user_input)
    .expect("Filed to read input");

    let input_to_int:i32 =user_input
    .trim_end()
    .parse()
    .expect("Filed to turn to int");


    if input_to_int == the_number {
        println!("wow {} is the right number ",input_to_int);
        println!("the random num is {}",the_number);
        
    }else {
        println!("No the number is {} ",the_number);
    };


}