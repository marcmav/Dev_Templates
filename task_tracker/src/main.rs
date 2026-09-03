use std::env;

fn main() {
    let args: Vec<String> = env();
    if args.len() < 2 {
        println!("wrong input");
        println!("./{} <args>", args[0]);
    }
    for arg in args.iter() {
        println!("{arg}");
    }
}
