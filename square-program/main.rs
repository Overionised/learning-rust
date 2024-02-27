use std::io::{self, Write};
use crate::io::stdout;

fn read_i32() -> i32 {
    loop {
        let line = io::stdin().lines().next().unwrap().unwrap();
        match line.trim().parse::<i32>() {
            Ok(number) => return number,
            Err(_) => {
                println!("Error: Please enter a valid integer.");
                print!("Try again: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}

fn square_it( x : i32, y : i32, f : i32){

    for i in 0..x{
        for j in 0..y{
            if i == 0 || i == x-1 || j == 0 || j == y-1 || j+f==y/2 || i+f==x/2{
                print!("* ");
            }
            else{
                print!("  ");
            }

        }
        println!();
    }
}
fn main() {
    print!("tell me the x here : ");

    io::stdout().flush().unwrap();

    let x : i32 = read_i32();

    print!("tell me the y here : ");
    io::stdout().flush().unwrap();

    let y : i32 = read_i32();

    println!("the numbers are {} {}", x, y);


    println!("funny offset? -(can also be negative)");
    print!("if no, just write 0 : ");
    stdout().flush().expect("Failed to flush");

    let f : i32 = read_i32();

    square_it(x, y, f);
}
