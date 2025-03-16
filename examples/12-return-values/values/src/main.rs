
fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts:  = s.split(delimiter).collect();
    // This will not compile!
    let result = parts.get(field);
    // result.to_string()
    result.expect("Problem!!! Expect not fullfield").to_string()

}

fn main() {
    let mut chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}\n", chunk);

     chunk = split_string("hello,world".to_string(), ',', 2);
    println!("Split string: {}", chunk);
}
