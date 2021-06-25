#[allow(dead_code)]
mod base;

use base::*;

fn main() {
    let mut n = 1;
    let mut cache = Cache::default();
    loop {
        println!("{}: {}", n, BaseName(Base::new(n, &mut cache), true));
        n += 1;
    }
}
