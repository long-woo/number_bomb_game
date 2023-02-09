use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\n\n！！！数字炸弹！！！");

    let mut min_num = 1;
    let mut max_num = 100;
    let rand_num = rand::thread_rng().gen_range(min_num..=max_num);

    // println!("炸弹数字：{}", rand_num);
    loop {
        println!("\n请输入数字：[{}-{}]", min_num, max_num);

        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("错误");

        // 输入数字转换
        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("* 不能输入非数字！");
                continue;
            }
        };

        match num.cmp(&rand_num) {
            Ordering::Greater => {
                if num > max_num {
                    println!("* 输入的数字不能大于 {}", max_num);
                    continue;
                }

                println!("大了");
                max_num = num;
            },
            Ordering::Less => {
                if num < min_num {
                    println!("* 输入的数字不能小于 {}", min_num);
                    continue;
                }

                println!("小了");
                min_num = num;
            },
            Ordering::Equal => {
                print!("炸了");
                break;
            }
        }
    }
}
