use basic_lexer::settings::{
    run
};

use basic_lexer::main_logic::syntaxd::Dictionaries;

fn main() {
    let program = "ПУСТЬ х = 10 * 2\nПЕЧАТЬ х";
    run(program, Dictionaries::Russian);

    let next_progamm = "LET x = 10 + 5 / 3 * 7\nPRINT x";

    run(next_progamm, Dictionaries::English);

    let emoji_programm = "✍️ y = 10 * 5 + 2 * 7\n🖨 y\n⌨️ x\n✍️ x = x + y\n🖨 x";
    run(emoji_programm, Dictionaries::Emoji);
}

