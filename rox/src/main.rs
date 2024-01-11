use std::{env, process, io::{self, Read}, fs::File};

fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    }
    args.next();

    if args.len() == 2 {
        run_file(args.next().expect("no script file path"));
    } else {
        run_prompt();
    }
}


fn run_file(script_path: String) {
    let mut f = File::open(script_path).expect("failed open script file");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("failed read script file");

    run(buffer);
}

fn run_prompt() {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("failed read line");
        run(buf);
    }
}

fn run(script: String) {
    println!("{}", script);
}