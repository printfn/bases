#[allow(dead_code)]
mod base;

use std::{env, error};
use base::*;

fn loop_all_numbers() -> ! {
    let mut n = 1;
    let mut cache = Cache::default();
    loop {
        let name = BaseName(Base::new(n, &mut cache), true);
        let abbr = find_abbreviation(n, &mut cache);
        println!("{}: {} ({})", n, name, abbr);
        n += 1;
    }
}

fn parse_and_display_number(n: &str) -> Result<(), Box<dyn error::Error>> {
    let n: i64 = n.parse()?;
    let mut cache = Cache::default();
    let name = BaseName(Base::new(n, &mut cache), true);
    println!("{}", name);
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        loop_all_numbers();
    } else {
        match parse_and_display_number(&args[1]) {
            Ok(()) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
