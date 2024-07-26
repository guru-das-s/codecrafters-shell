#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, process::exit};

enum CmdHandler {
    Exit(fn(i32)),
    Echo(fn(&[&str])),
}

fn handle_exit(exit_code: i32) {
    exit(exit_code);
}

fn handle_echo(tokens: &[&str]) {
    println!("{}", tokens.join(" "));
}

fn is_valid(builtins: &HashMap<String, CmdHandler>, input: &str) -> bool {
    return builtins.contains_key(input);
}

fn process_input(builtins: &HashMap<String, CmdHandler>, input: &String) {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() == 0 {
        return;
    }

    if !is_valid(&builtins, &tokens[0]) {
        println!("{}: command not found", input.trim());
        return;
    }

    let cmd = builtins.get(&tokens[0] as &str);
    if let Some(cmd) = cmd {
        match cmd {
            CmdHandler::Exit(f) => {
                if let Ok(code) = tokens[1].parse::<i32>() {
                    f(code);
                }
            }
            CmdHandler::Echo(f) => f(&tokens[1..]),
        }
    }
}

fn initialize_shell_builtins(builtins: &mut HashMap<String, CmdHandler>) {
    builtins.insert("exit".to_string(), CmdHandler::Exit(handle_exit));
    builtins.insert("echo".to_string(), CmdHandler::Echo(handle_echo));
}

fn main() {
    let mut builtins: HashMap<String, CmdHandler> = HashMap::new();

    initialize_shell_builtins(&mut builtins);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        process_input(&builtins, &input);
    }
}
