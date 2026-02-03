#[cfg(test)]
mod tests;

mod chrn;
mod enm;
mod inpt;
mod myutils;
use crate::myutils::filereader::read_username_from_file;
use crate::myutils::or;
use myutils::{collect, ferris_say_test, find_word, mtch, slice, strct};
use std::io::{self, Read};

pub fn start() {
    'outer: loop {
        let num: u8 = inpt::inp_num();

        match inpt::gen_num(num) {
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
            1 => chrn::test(),
            2 => ferris_say_test(),
            3 => or::test(),
            4 => strct(),
            5 => println!("{:?}", find_word("Za warudo!", 3)),
            6 => slice(),
            7 => enm::enm(),
            8 => mtch(),
            9 => collect(),
            10 => panic!("crash and burn trash"),
            11 => {
                let v = vec![1, 2, 3];
                v[99];
            }
            12 => {}
            13 => println!("{:?}", read_username_from_file()),

            n => println!("непонятно: {n}"),
        }
    }

    println!("Программа 'trash' выполнена. Нажмите Enter для выхода...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

// pub fn
