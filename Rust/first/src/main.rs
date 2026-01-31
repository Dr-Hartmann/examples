mod chrn;
mod ferris;
mod or;
mod strct;
mod wordcut;

use std::io::{self, Read};

fn main() {
    'outer: loop {
        let num: u8 = inp_num();

        match gen_num(num) {
            Some(result) => {
                let num = result;
                println!("Новое значение задания: {}", num);
            }
            None => {
                println!("Ошибка: произошло переполнение!");
                break 'outer;
            }
        }

        println!("Выбрано: {}", num);
        match num {
            1 => chrn::test(),
            2 => ferris::test(),
            3 => or::test(),
            4 => strct::test(),
            5 => println!("{:?}", wordcut::find_word("Za warudo!", 3)),
            6 => test_slice(),
            _ => {
                println!("непонятно");
                break 'outer;
            }
        }
    }

    println!("Программа выполнена. Нажмите Enter для выхода...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

use inquire::Text;
fn inp_num() -> u8 {
    loop {
        let input = Text::new("Какое задание выполнить?")
            .with_help_message("Введите число")
            .prompt()
            .expect("Неудалось прочитать строку");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(e) => {
                println!("{e}: Введите нормальное число!");
                continue;
            }
        }
    }
}

use rand::Rng;
fn gen_num(num: u8) -> Option<u8> {
    let mut rng = rand::rng();
    let secret_sub = rng.random_range(0..=2);
    num.checked_sub(secret_sub)
}

fn test_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
