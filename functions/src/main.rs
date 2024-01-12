fn main() {
    println!("Hello, world!");

    let str = another_function(53);

    println!("{str}")
}

fn another_function(x: u32) -> &str {
    println!("The value of x is: {x}");
    return "Hello"
}
