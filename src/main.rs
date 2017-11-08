extern crate rand;

use rand::Rng;

/// サンプルデータを表示する
pub fn show_numbers(numbers: &Vec<i32>) {
    println!("サンプルデータ");
    println!("{:?}", numbers);
}

/// シンプルに重複しないデータの作成
pub fn distinct_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let mut distinct_numbers: Vec<i32> = Vec::new();
    for v in &numbers {
        let mut flag:bool = true;
        for v2 in &distinct_numbers {
            if v == v2 {
                flag = false;
                break;
            }
        }
        if flag {
            distinct_numbers.push(v.abs());
        }
    }
    return distinct_numbers;
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..500 {
        let x: i32 = rand::thread_rng().gen_range(1, 101);
        numbers.push(x);
    }
    show_numbers(&numbers);
    let distinct_numbers: Vec<i32> = distinct_numbers(numbers);
    println!("{:?}", distinct_numbers);
}

