use inquire::Text;
use rand::Rng;

pub fn inp_num() -> u8 {
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

pub fn gen_num(num: u8) -> Option<u8> {
    let mut rng = rand::rng();
    let secret_sub = rng.random_range(0..=2);
    num.checked_sub(secret_sub)
}
