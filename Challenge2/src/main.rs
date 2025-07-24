#![allow(unused)]

use std::io;

fn main() { 
    
    // creat variables 
    let mut x = String::new();
    let mut y = String::new();
    
    //get numbers from user input
    println!("enter the first number : ");
    io::stdin().read_line(&mut x).expect("Faild to read input");

    println!("enter the second number:");
    io::stdin().read_line(&mut y).expect("Faild to read input");
    
    // string to int and float
    let x_int: i32 = x.trim_end().parse().expect("x is not number");
    let y_int:i32 = y.trim_end().parse().expect("y is not number");
    let x_float= x_int as f32 ;
    let y_float = y_int as f32;

    //print result
    println!("{} + {} = {}",x_int,y_int,x_int+y_int );
    println!("{} - {} = {}",x_int,y_int,x_int-y_int );
    println!("{} * {} = {}",x_int,y_int,x_int*y_int );
    println!("{} / {} = {}",x_int,y_int,x_float / y_float );
}