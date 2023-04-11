use super::Editor;
use portable_pty::{CommandBuilder, NativePtySystem, PtyPair, PtySize, PtySystem};
use std::collections::VecDeque;
use std::env;
use std::process::Command;
use std::sync::mpsc::channel;

pub struct Terminal {
    pub is_toggled: bool,
    pub output: String,
    pub input: String,
    pub shell: String,
    pub pty: PtyPair,
    pub com: Command,
}

impl Terminal {
    // pub fn new() -> Self {
    //     Terminal {
    //         is_toggled: false,
    //         output: String::new(),
    //         input: String::new(),
    //         shell: env::var("SHELL").unwrap(),
    //         com: Command::new(env::var("SHELL").unwrap()),
    //     }
    // }
    pub fn startup(ed: &mut Editor) {
        let mut fin = "\n> ";
        ed.terminal.output.push_str(fin.clone());
        ed.terminal.input.push_str(fin.clone());
    }
}
impl Default for Terminal {
    fn default() -> Self {
        let pty_system = NativePtySystem::default();
        Terminal {
            is_toggled: false,
            output: String::new(),
            input: String::new(),
            pty: {
                pty_system
                    .openpty(PtySize {
                        rows: 24,
                        cols: 80,
                        pixel_width: 0,
                        pixel_height: 0,
                    })
                    .unwrap()
            },
            shell: "".to_string(),
            com: Command::new(""),
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

    // if ed.terminal.output.capacity() > ed.terminal.input.capacity() {
    //     ed.terminal.input = ed.terminal.output.clone();
    // }
    let inp = ed.terminal.input.clone();
    let inp = inp.as_str();
    if inp.ends_with("\n") {
        let _ = ed.terminal.input.trim_end();
        let comand = crop_letters(inp, ed.terminal.output.capacity());
        let mut sp: VecDeque<_> = comand.clone().trim().split(" ").collect();
        let head = sp.pop_front().unwrap();
        let mut com = CommandBuilder::new(head);
        // if !sp.is_empty() {
        //     com.args(sp);
        // }
        // let child = ed.terminal.pty.slave.spawn_command(com);
        let child = ed.terminal.pty.slave.spawn_command(com);

        // drop(ed.terminal.pty.slave.as_ref());
        // let (tx, rx) = channel();

        // let mut reader = ed.terminal.pty.master.try_clone_reader().unwrap();
        // // println!("cheguei",);

        // std::thread::spawn(move || {
        //     // Consume the output from the child
        //     let mut s = String::new();
        //     reader.read_to_string(&mut s).unwrap();
        //     println!("{}", s);
        //     tx.send(s).unwrap();
        //     std::thread::sleep(std::time::Duration::from_millis(20));
        // });
        // let output = rx.recv();
        // println!("{:?}", output);

        // let mut writer = ed.terminal.pty.master.take_writer().unwrap();

        // // for c in output {
        // //     print!("{}", c);
        // }
    }
    // ed.terminal.com = Command::new(format!("{} -c {}", ed.terminal.shell, comand));

    // ed.terminal
    //     .input
}
// TODO: Fazer terminal com PTY usar lib cross platform
//
// TODO: JAT -> se input mais curto que output input = output para evitar que user apague a shell
