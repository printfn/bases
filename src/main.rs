#[allow(dead_code)]
mod base;

use base::*;

fn main() {
    let mut n = 1;
    let mut cache = Cache::default();
    loop {
        let name = BaseName(Base::new(n, &mut cache), true);
        let abbr = find_abbreviation(n, &mut cache);
        println!("{}: {} ({})", n, name, abbr);
        n += 1;
    }
}
