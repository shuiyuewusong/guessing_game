use std::io;

fn main() {
    println!("猜数!");

    let mut  guess = String::new();
    let result = io::stdin().read_line(&mut guess);
    result.expect("无法读取");
    println!("你猜测的数字是: {}", guess);
}
