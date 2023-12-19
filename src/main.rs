// allow unused variables (using this removes warnings of unused variables)
#![allow(unused)]

// adding libraries
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    /* 
    println!("What is your name?");
    // creating a mutable variable, a variable that can be changed. immutable variables cannot be changed.
    // the String::new() creates an empty string
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    // reading input from keyboard. the "&" in front of mut means that its referencing the mutable variable called name
    io::stdin().read_line(&mut name)
        // read_line() returns a "result" which is a ENUM and it returns two values that are "Ok" or "Err"
        .expect("Didn't receive input");
    // puting {} inside a println!() means you can sign a variable there and print it
    // you put the variables after the "". trim_end() makes it so we don't break a line.
    println!("Hello {}! {}", name.trim_end(), greeting);

    // constants variables are always named in uppercase to separate them from other variables
    // big values like one million, you separate them using underline (_)
    // u32 means that it's an unsigned 32 bit integer 
    const ONE_MIL: u32 = 1_000_000;
    // f32 means that it's a 32 bit float
    const PI: f32 = 3.141592;
    let age: &str = "47";
    // you can create variables with the same name but with different data types and it's called shadowing
    // trim() takes out all the whitespace if there is any
    let mut age: u32 = age.trim().parse()
        // to handle errors, we use expect(msg)
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
    */
}
