// use std::io;

fn main() {
    // println!("hello world");
    // println!("please input a number:");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("failed to read input");
    // println!("your raw input is: {:?}", input);
    // let number: i64 = input.trim().parse().expect("input is not a number");
    // println!("your input is {}", number);

    let a = (1, 2);
    println!("a: {:?}", a);
    let (a, b) = a;
    println!("a: {:?}, b: {:}", a, b);
}

