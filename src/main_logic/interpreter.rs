use std::collections::HashMap;
use super::parser::Command;
use rand::*;

#[derive(Debug, PartialEq)]
pub enum Signal<'a> {
    Continue,
    Jump {label: &'a str},
    None,
}

#[allow(dead_code)]
pub struct Interpreter<'a> {
    env: HashMap<&'a str, i64>
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self { env: HashMap::new() }
    }

    pub fn get_marks(&mut self, commands: &[Command<'a>]) {
        let mut marks: HashMap<&'a str, usize> = HashMap::new();
        for (idx, mark) in commands.iter().enumerate() {
            if let Command::Label { name } = mark {
                marks.insert(name, idx);
            }
        }
        self.execute(commands, &marks);
    }

    pub fn execute(&mut self, commands: &[Command<'a>], labels: &HashMap<&'a str, usize>) {
        let mut command_idx = 0;
        while command_idx < commands.len() {
            match self.execute_single(&commands[command_idx], labels) {
                Signal::None => {
                    return;
                }
                Signal::Jump { label } => {
                    if let Some(&new_idx) = labels.get(label) {
                        command_idx = new_idx;
                        continue;
                    }
                }
                Signal::Continue => {
                    command_idx += 1
                }
            }
        }
    }
    // Function returns Option<usize>. 
    // If Some(idx) was returned — GOTO worked and we need to jump to next index
    fn execute_single(&mut self, cmd: &Command<'a>, labels: &HashMap<&'a str, usize>) -> Signal<'a> {
        match cmd {
            Command::Label { .. } => Signal::Continue,
            
            Command::GOTO { label } => {
                if labels.contains_key(label) {
                    Signal::Jump { label }
                } else {
                    println!("Runtime Error: label '{}' not found", label);
                    Signal::Continue
                }
            }
            Command::Assign { name, value } => {
                let final_value = value.evaluate(&self.env).expect("Execute Error");
                self.env.insert(name, final_value);
                Signal::Continue
            }
            
            Command::Input { name } => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let value: i64 = input.trim().parse().unwrap_or(0);
                self.env.insert(name, value);
                Signal::Continue
            }
     
            Command::PrintStr(text) => {
                println!("{}", text);
                Signal::Continue
            }

            Command::PrintVar(name) => {
                // Если это переменная, ищем её в env
                if let Some(val) = self.env.get(name) {
                    println!("{}", val)
                } else {
                    println!("Runtime Error: variable '{}' is not defined", name)
                }
                Signal::Continue
            }
            
            Command::IF { left_value, cmp, right_value, body } => {
                let lhs = left_value.evaluate(&self.env);
                let rhs = right_value.evaluate(&self.env);

                let condition = match cmp {
                    '=' => lhs == rhs,
                    '!' => lhs != rhs,
                    '<' => lhs < rhs, 
                    '>' => lhs > rhs,
                    _ => unreachable!(),
                };

                if condition {
                    // If condition is true, we start cycle and execute programs one by one
                    for inner_cmd in body {
                        // Checking signal
                        let signal = self.execute_single(inner_cmd, labels);
                        // If signal != Continue -> Signal::None or Signal::Jump
                        if signal != Signal::Continue {
                            return signal;
                        }
                    }
                }
                // default Signal
                Signal::Continue
            }

            Command::Random { name, min, max } => {
                let mut rng = rand::thread_rng();
                let min_val = *min;
                let max_val = *max;

                let random_value: i64 = rng.gen_range(min_val..=max_val);
                
                // Записываем его в наше окружение, как обычный LET
                self.env.insert(name, random_value);
                Signal::Continue
            }

            Command::End => {
                return Signal::None;
            }
        }
    }
}