mod internals;
use internals::scanner::*;
use internals::utils::*;
use std::io;
use std::io::{BufRead, stdout, Write};
use std::fs::read_to_string;
use std::env::args;
use std::process::exit;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: ")
    }else if args.len() == 2 {
        println!("running ...");
        let _ = run_file(String::from(args[1].to_string()));
    }else {
        run_shell()
    }
}

fn run_file(filename: String) -> io::Result<()> {
    let buf = read_to_string(filename)?;
    match run(buf) {
        Ok(_) => {},
        Err(e) => {
            e.report();
            exit(65)
        }
    }
    Ok(())
}

fn run_shell() {
    let stdin = io::stdin();
    print!("pn> ");
    let _ = stdout().flush().expect("Failed to flush stdout");
    for line in stdin.lock().lines() {
        if let Ok(li) = line {
            // if li.is_empty() {
            //     break;
            // }
            match run(li) {
                Ok(_) => {},
                Err(e)=> {
                    e.report()
                }
            }
        } else {
            break;
        }
        print!("pn> ");
        let _ = stdout().flush().expect("Failed to flush stdout");

    }
}

fn run(source: String) -> Result<(), PneumaError>{
    let mut scanner = Scanner::new(source);
    let _status = scanner.generate_tokens()?;
    println!("{:#?}", &scanner);
    // for t in &scanner.tokens {
    //     println!("{:#?}", t);
    // }
    Ok(())
}
