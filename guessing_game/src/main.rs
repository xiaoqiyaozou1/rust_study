use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    

    let secret_number =rand::thread_rng().gen_range(1..=100);
    loop{
        println!("请输入你猜的数字");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("你猜的数字是：{guess}");
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("小了！"),
            Ordering::Greater =>println!("大了！"),
            Ordering::Equal=>{
                println!("猜对了！");
                break;
            }

        }
    }

  
}
