use core::borrow;
use std::vec;

fn own_vec(vector: &mut Vec<i32>) -> Vec<i32> {
    let mut new_vector = Vec::new();
    
    println!("vector (before append): {:?}", vector);

    // append vector to new_vector
    new_vector.append(vector);

    new_vector.push(10);
    println!("vector (after append): {:?}", vector);
    
    println!("new_vector: {:?}", new_vector);
    
    new_vector
}

fn own_vec_2(vector: &mut Vec<i32>) {
    println!("vector (before push): {:?}", vector);
    vector.push(10);
    println!("vector (after push): {:?}", vector);
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}

fn borrow_vec(vector: &Vec<i32>) {
    println!("{:?}", vector);
}

fn borrow_string(s: &mut String) {
    println!("{}", s);
    s.push_str(" plus string");
    println!("{}", s);
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
// or another part of your program without actually transferring ownership of the variable. 
// When you borrow a variable, you're essentially saying 
// "I want to use this variable for a little while, but I promise I won't modify it."
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    // this compiles no problem!
    own_integer(my_int);
    println!("{}", my_int);

    own_string(my_string); // take ownership of my_string
    // this is using my_string which has also moved and is invalid
    // println!("{:?}", my_string); // this will not compile!

    // own_vec(&mut my_vec);
    own_vec_2( &mut my_vec);
    // but this is using my_vec which was borrowed (moved) and yet is now invalid
    println!("{:?}", my_vec); // this will now compile!

    borrow_vec(&my_vec);

    let mut my_string_2 = String::from("Hello, world, again!");
    borrow_string(&mut my_string_2);

}

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// By lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.