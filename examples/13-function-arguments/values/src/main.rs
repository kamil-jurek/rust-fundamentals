use std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // There are no variadic arguments in Rust
    // let numbers = [1, 2, 3, 4, 5];
    let mut numbers: Vec<i32> = Vec::new();

    let mut input = String::new();
    loop {
        input.clear();
        println!("Please enter a number (or 'done' to finish):");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        if input == "done" {
            break;
        }
        let number: i32 = input.parse().expect("Not a number!");
        numbers.push(number);
    }

    let result = sum(&numbers);
    println!("The numbers are: {:?}", numbers);
    println!("The sum is {}", result);
}
