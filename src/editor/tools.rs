use super::Editor;
use std::env;
use std::process::Command;

pub struct Terminal {
    pub is_toggled: bool,
    pub output: String,
    pub input: String,
    pub shell: String,
    pub com: Command,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            is_toggled: false,
            output: String::new(),
            input: String::new(),
            shell: env::var("SHELL").unwrap(),
            com: Command::new(env::var("SHELL").unwrap()),
        }
    }
    pub fn startup(ed: &mut Editor) {
        let mut fin = "\n> ";
        ed.terminal.output.push_str(fin.clone());
        ed.terminal.input.push_str(fin.clone());
    }
}
impl Default for Terminal {
    fn default() -> Self {
        Terminal {
            is_toggled: false,
            output: String::new(),
            input: String::new(),
            shell: env::var("SHELL").unwrap(),
            com: Command::new(env::var("SHELL").unwrap()),
        }
    }
}

pub fn run_file(ed: &mut Editor) {}

pub fn run_project(ed: &mut Editor) {}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

pub fn terminal(ed: &mut Editor) {
    // let mut s1 = "ola tudo bem";
    // let s2 = String::from("ola tu");
    // let output = crop_letters(s1, s2.capacity());
    // println!("cropped: {}", output);
    let inp = ed.terminal.input.as_str();
    let comand = crop_letters(inp, ed.terminal.output.capacity());
    ed.terminal.com = Command::new(format!("{} -c {}", ed.terminal.shell, comand));
    let _ = ed.terminal.com.spawn();
    // ed.terminal.com.arg("-c");
    // ed.terminal.com.arg(comand);
    match ed.terminal.com.status() {
        Ok(_) => {
            let output = String::from_utf8(ed.terminal.com.output().unwrap().stdout).unwrap();
            println!("{}", output);
            ed.terminal
                .input
                .push_str(format!("\n{}\n>", output).as_str());
            ed.terminal.output = ed.terminal.input.clone();
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    // ed.terminal
    //     .input
}
