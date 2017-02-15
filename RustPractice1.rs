/*
    This program takes two inputs and prints their sum.
*/
use std::io;

fn main(){
    let mut str1 = String::new();
    let mut str2 = String::new();
    
    io::stdin().read_line(&mut str1).ok().expect("read error");
    io::stdin().read_line(&mut str2).ok().expect("read error");
    
    let num1 : i32  =   str1.trim().parse().ok().expect("parse error");
    let num2 : i32  =   str2.trim().parse().ok().expect("parse error");
    
    let sum : i32 = num1 + num2;
    
    println!("{}",sum);
    
}
