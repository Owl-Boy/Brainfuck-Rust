mod interpreter;

use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let opt = args.pop();
    interpret();
    match opt {
        None => help(),
        Some(i) => {
            if i == "-c".to_string() { 
                compile()
            } else if i == "-i".to_string() {
                interpret()
            } else {
                help()
            }
        }
    }
}

fn help() {
}

fn compile() {
}

fn interpret() {
    // let machine = interpreter::Memory::new("++++++++++++++++++++++++++++++++++++++++++++++++++++++.".to_string());
    let machine = interpreter::Memory::new("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string());
    println!("{:?}", machine.map(|m| m.run()));
}
