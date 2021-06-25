#![allow(dead_code)]

mod base;

use base::*;

pub use base::BaseName;
pub use base::Cache;

/// Get the name of a given number base
pub fn base_name(number: i64, cache: &mut Cache) -> BaseName {
    BaseName(Base::new(number, cache), true)
}

/// Get the name of a given number base
pub fn rational_base_name(num: i64, den: i64, cache: &mut Cache) -> BaseName {
    BaseName(Base::new_frac(num, den, cache), true)
}

/// Get the name of a non-rational base, e.g. base pi, phi or tau
pub fn non_rational_base_name(name: &str, greater_than_six: bool, one_syllable: bool) -> BaseName {
    BaseName(Base::new_custom(name, greater_than_six, one_syllable), false)
}

/// Parse a given base name into a number
fn parse_base_name(name: &str) -> Option<i64> {
    Some(Base::try_parse(name)?.to_number())
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::prelude::v1::*;

    use super::*;

    #[track_caller]
    fn check_name(n: i64, s: &str, cache: &mut Cache) {
        assert_eq!(base_name(n, cache).to_string(), s);
    }

    #[test]
    #[ignore]
    fn roundtrip() {
        let mut cache = Cache::default();
        for n in 0..1000 {
            let s = base_name(n, &mut cache);
            let name = s.to_string();
            let base = parse_base_name(&name).unwrap();
            assert_eq!(base, n);
        }
    }

    #[test]
    fn factorised_names() {
        let mut cache = Cache::default();
        check_name(14, "biseptimal", &mut cache);
        check_name(15, "triquinary", &mut cache);
        check_name(18, "triseximal", &mut cache);
        check_name(21, "triseptimal", &mut cache);
        check_name(22, "bielevenary", &mut cache);
        check_name(24, "tetraseximal", &mut cache);
        check_name(60, "hexagesimal", &mut cache);
    }

    #[test]
    fn prime_base() {
        let mut cache = Cache::default();
        check_name(19, "untriseximal", &mut cache);
        check_name(23, "unbielevenary", &mut cache);
        check_name(29, "untetraseptimal", &mut cache);
        check_name(31, "unpentaseximal", &mut cache);
    }

    #[test]
    fn different_suffix() {
        let mut cache = Cache::default();
        check_name(60, "hexagesimal", &mut cache);
        check_name(26, "biker's dozenal", &mut cache);
    }

    #[test]
    fn rational_names() {
        let mut cache = Cache::default();
        assert_eq!(rational_base_name(1, 10, &mut cache).to_string(), "votdecimal");
        assert_eq!(rational_base_name(2, 3, &mut cache).to_string(), "bivottrinary");
    }

    #[test]
    fn custom_names() {
        assert_eq!(non_rational_base_name("phi", false, true).to_string(), "phinary");
        assert_eq!(non_rational_base_name("tau", true, true).to_string(), "tauimal");
    }
}
