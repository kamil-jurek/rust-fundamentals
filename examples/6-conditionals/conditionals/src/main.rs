// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    //let maybe_number: Option<Option<()>> = None;
    // let maybe_number = Some(42);
    let maybe_number = Some(Some(42));

    if let Some(number) = maybe_number {
        if let Some(42) = number {
            println!("The number is 42");
        } else if let Some(neseted_number) = number {
            println!("The nested_number is {:?}", neseted_number);
        } else {
            println!("There is no nested_number");
        }
    } else {
        println!("There is no number");
    }
}
