use basic_lexer::settings::{
    run
};

use basic_lexer::main_logic::syntaxd::Dictionaries;

use std::env;
use std::fs;
use std::process;

fn main() {
    // 1. Собираем аргументы, которые передали в терминале
    let args: Vec<String> = env::args().collect();

    // 2. Проверяем, передал ли пользователь имя файла
    if args.len() < 2 {
        println!("Ошибка: Вы не указали файл с программой!");
        println!("Использование: cargo run -- <имя_файла>");
        process::exit(1);
    }
    
    // Имя файла — это первый аргумент после `cargo run --`
    // Например: cargo run -- C:\Folder\Folder\Folder\src\FILES\game.bas
    let file_path = &args[1];

    // 3. Читаем весь файл в одну строку
    let program = match fs::read_to_string(file_path) {
        Ok(content) => content
            .replace("\r", "")
            .replace("\u{fe0f}", ""),
        Err(err) => {
            println!("Ошибка чтения: {}", err);
            process::exit(1);
        }
    };

    println!("Запуск программы {}...", file_path);
    println!("--------------------------------");

    run(&program, Dictionaries::Crab);
}
