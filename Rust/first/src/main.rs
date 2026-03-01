use leetcode::{
    add_two_numbers, is_palindrome_math, is_palindrome_string, length_of_longest_substring,
    list_to_i32, two_sum_brute_force, two_sum_hash_table,
};
use my_trash::{
    Day, Train, chrn_test, ferris_say_test, gen_num, inp_num, or_test, read_username_from_file,
    reference_cycles_1, reference_cycles_2,
};
use std::io::{self, Read};

use backend::{self, axum, poem};

fn main() {
    println!("It`s work, isn`t?");
    print!("Здесь нет");

    'outer: loop {
        let num: u8 = inp_num("Какое задание выполнить?");

        match gen_num(num) {
            Some(result) => {
                let num = result;
                println!("Новое значение задания: {}", num);
            }
            None => {
                println!("Ошибка! (передано '{num}')");
                break 'outer;
            }
        }

        println!("Выбрано: {}", num);
        match num {
            1 => chrn_test(),
            2 => ferris_say_test(),
            3 => or_test(),
            4 => {
                let mut v = vec![Train::new("moscow", Day::Today, [0; 5])];
                v[0].t[0] = -0b0111_1111;
                v.push(Train {
                    location: String::from("Bejing"),
                    date: Day::Yesterday,
                    ..v[0]
                });
                v[1].set_tomorrow();
                v.into_iter().for_each(|f| f.hello());
            }
            5 => {
                let get_word = |s: &'static str, i: usize| s.split_whitespace().nth(i);
                println!("{:?}", get_word("Za warudo!", 2))
            }
            // * 6 => slice(),
            // ? 7 => enm_test(),
            // TODO: 8 => mtch(),
            // ! 9 => collect(),
            10 => panic!("crash and burn trash"),
            11 => {
                let v = vec![1, 2, 3];
                v[99];
            }
            12 => println!(
                "{:?}",
                "мама мыла раму"
                    .lines()
                    .next()
                    .and_then(|line| line.chars().last())
            ),
            13 => println!("{:?}", read_username_from_file()),
            14 => reference_cycles_1(),
            15 => reference_cycles_2(),
            16 => backend::start(),
            17 => poem::start(),
            18 => axum::start(),
            30 => {
                let start = std::time::Instant::now();
                let _ = two_sum_hash_table((1..=10_000).collect(), 19_999);
                eprintln!("hash_table = {:#?}", start.elapsed());

                let start = std::time::Instant::now();
                let _ = two_sum_brute_force((1..=10_000).collect(), 19_999);
                eprintln!("hash_table = {:#?}", start.elapsed());
            }
            31 => {
                let (l1, l2) = (17, 129);
                let res = &l1 + &l2;
                let out = add_two_numbers(17, 129);
                println!("{l1} + {l2} = {res}");
                println!("Answer: {:?}", out);
                println!("Answer: {}", list_to_i32(Some(out)));
            }
            32 => {
                let len = length_of_longest_substring("abcdabcdebbacfgjku");
                println!("{len}");
            }
            33 => {
                let x = 122;
                let out = if is_palindrome_math(x) && is_palindrome_string(x) {
                    "является палиндромом"
                } else {
                    "не является палиндромом"
                };
                println!("{x} {}", out);
            }
            n => println!("непонятно: {n}"),
        }
    }

    println!("Программа 'trash' выполнена. Нажмите Enter для выхода...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
