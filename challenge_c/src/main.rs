use std::io;

fn calculator(x:i32,y:i32,operator:&str){
    match operator {
        "+" => add(x,y),
        "-" => subtract(x,y),
        "*" => multiply(x,y),
        "/" => divide(x,y),
        _ => println!("Invalid operator"),
    }
}
//math function
fn add(x:i32,y:i32) {
    println!("{} + {} = {}",x,y,x+y);
}
fn subtract(x:i32,y:i32) {
    println!("{} - {} = {}",x,y,x-y);
    }
fn multiply(x:i32,y:i32) {
    println!("{} * {} = {}",x,y,x*y);
    }
fn divide(x:i32,y:i32) {
    let x_to_float:f32 = x as f32;
    let y_to_float:f32 = y as f32;
    println!("{} / {} = {}",x_to_float,y_to_float,x_to_float/y_to_float);
}    

fn start() {
    // Todo list 
    //creat a calculator that takes 3 inputs (x, y, operator)
    //creat fonction for +, -, *, /
    //use if/else or match to determine which operator to use
    
println!("enter your first number:");
let mut x = String::new();
io::stdin().read_line(&mut x).expect("failed to read line");
let x: i32 = x.trim().parse().expect("please type a number");

println!("enter your second number:");
let mut y = String::new();
io::stdin().read_line(&mut y).expect("failed to read line");
let y: i32 = y.trim().parse().expect("please type a number");

println!("enter your operator (+, -, *, /):");
let mut operator = String::new();
io::stdin().read_line(&mut operator).expect("failed to read line");
let operator = operator.trim();

calculator(x,y,operator);
println!("------------------")
}
//loop to keep the calculator running
fn main() {
    loop {
        start();
        }
}
