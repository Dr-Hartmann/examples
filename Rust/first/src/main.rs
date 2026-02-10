use leetcode::{
    add_two_numbers, is_palindrome_math, is_palindrome_string, length_of_longest_substring,
    list_to_i32, two_sum_brute_force, two_sum_hash_table,
};
use my_trash::{
    chrn_test, collect, ferris_say_test, find_word, gen_num, inp_num, last_char_of_first_line,
    mtch, or_test, read_username_from_file, reference_cycles, reference_cycles_2, slice, strt,
};
use std::io::{self, Read};

fn main() {
    println!("It`s work, isn`t?");

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
            4 => strt(),
            5 => println!("{:?}", find_word("Za warudo!", 3)),
            6 => slice(),
            // 7 => enm_test(),
            8 => mtch(),
            9 => collect(),
            10 => panic!("crash and burn trash"),
            11 => {
                let v = vec![1, 2, 3];
                v[99];
            }
            12 => println!("{:?}", last_char_of_first_line("мама мыла раму")),
            13 => println!("{:?}", read_username_from_file()),
            14 => reference_cycles(),
            15 => reference_cycles_2(),
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
                println!("Answer: {}", list_to_i32(Some(out)));
            }
            32 => {
                let len = length_of_longest_substring("abcdabcdebbacfgjku");
                print!("{len}");
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
