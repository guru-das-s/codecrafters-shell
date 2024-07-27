#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, env, fs, process::exit};

enum CmdHandler {
    Exit(fn(i32)),
    Echo(fn(&[&str])),
    Type(fn(&HashMap<String, CmdHandler>, &str)),
}

fn is_valid(builtins: &HashMap<String, CmdHandler>, input: &str) -> bool {
    return builtins.contains_key(input);
}

fn in_path(input: &str) -> Option<String> {
    let path = env::var("PATH").unwrap();
    let path_dirs: Vec<&str> = path.split(":").collect();

    for dir in path_dirs {
        let full_path = format!("{}/{}", dir, input);
        if fs::metadata(&full_path).is_ok() {
            return Some(full_path);
        }
    }

    None
}

fn handle_exit(exit_code: i32) {
    exit(exit_code);
}

fn handle_echo(tokens: &[&str]) {
    println!("{}", tokens.join(" "));
}

fn handle_type(builtins: &HashMap<String, CmdHandler>, input: &str) {
    print!("{}", input);
    if is_valid(&builtins, input) {
        print!(" is a shell builtin\n");
    } else if let Some(fq_path) = in_path(input) {
        print!(" is {}\n", fq_path);
    } else {
        print!(": not found\n");
    }
}
fn process_input(builtins: &HashMap<String, CmdHandler>, input: &String) {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() == 0 {
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
            CmdHandler::Type(f) => f(builtins, &tokens[1]),
        }
    } else {
        println!("{}: command not found", input.trim());
    }
}

fn initialize_shell_builtins(builtins: &mut HashMap<String, CmdHandler>) {
    builtins.insert("exit".to_string(), CmdHandler::Exit(handle_exit));
    builtins.insert("echo".to_string(), CmdHandler::Echo(handle_echo));
    builtins.insert("type".to_string(), CmdHandler::Type(handle_type));
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
