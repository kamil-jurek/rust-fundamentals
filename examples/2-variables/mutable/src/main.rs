fn main() {
    let mut message = String::from("Name: Alfredo, Height: ");
    message.clear();

    let mut height = 190;
    println!("{}", height);

    height = 189;
    message = "Alfonzo, Height: ".to_string();
    println!("{}{}", message, height);
}
