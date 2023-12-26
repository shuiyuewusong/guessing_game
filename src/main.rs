use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("神秘数字是:{}", secret_number);
    let mut number = 0;
    loop {
        println!("请输入您猜测的数字");
        let mut guess = String::new();
        let result = io::stdin().read_line(&mut guess);
        result.expect("无法读取");
        println!("你猜测的数字是: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(data) => {
                println!("错误信息您输入的字符不符合数字:{}", data);
                continue;
            }
        };
        number += 1;
        println!("您当前猜测次数:{}", number);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("数字小了"),
            Ordering::Greater => println!("数字大了"),
            Ordering::Equal => {
                number_ok();
                break;
            }
        }

    }
}

fn number_ok() {
    println!("正确!");
}
