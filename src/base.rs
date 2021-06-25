use std::{collections::HashMap, convert, fmt};

#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum Root {
    Binary = 2,
    Trinary = 3,
    Quaternary = 4,
    Quinary = 5,
    Seximal = 6,
    Septimal = 7,
    Octal = 8,
    Nonary = 9,
    Decimal = 10,
    Elevenary = 11,
    Dozenal = 12,
    BakersDozenal = 13,
    Hex = 16,
    Suboptimal = 17,
    Vigesimal = 20,
    Niftimal = 36,
    Centesimal = 100,
}

impl Root {
    fn from_number(number: i64) -> Option<Self> {
        Some(match number {
            2 => Self::Binary,
            3 => Self::Trinary,
            4 => Self::Quaternary,
            5 => Self::Quinary,
            6 => Self::Seximal,
            7 => Self::Septimal,
            8 => Self::Octal,
            9 => Self::Nonary,
            10 => Self::Decimal,
            11 => Self::Elevenary,
            12 => Self::Dozenal,
            13 => Self::BakersDozenal,
            16 => Self::Hex,
            17 => Self::Suboptimal,
            20 => Self::Vigesimal,
            36 => Self::Niftimal,
            100 => Self::Centesimal,
            _ => return None,
        })
    }

    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "binary" => Self::Binary,
            "trinary" => Self::Trinary,
            "quaternary" => Self::Quaternary,
            "quinary" => Self::Quinary,
            "seximal" => Self::Seximal,
            "septimal" => Self::Septimal,
            "octal" => Self::Octal,
            "nonary" => Self::Nonary,
            "decimal" => Self::Decimal,
            "elevenary" => Self::Elevenary,
            "dozenal" => Self::Dozenal,
            "baker's dozenal" => Self::BakersDozenal,
            "hex" => Self::Hex,
            "suboptimal" => Self::Suboptimal,
            "vigesimal" => Self::Vigesimal,
            "niftimal" => Self::Niftimal,
            "centesimal" => Self::Centesimal,
            _ => return None,
        })
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Trinary => "trinary",
            Self::Quaternary => "quaternary",
            Self::Quinary => "quinary",
            Self::Seximal => "seximal",
            Self::Septimal => "septimal",
            Self::Octal => "octal",
            Self::Nonary => "nonary",
            Self::Decimal => "decimal",
            Self::Elevenary => "elevenary",
            Self::Dozenal => "dozenal",
            Self::BakersDozenal => "baker's dozenal",
            Self::Hex => "hex",
            Self::Suboptimal => "suboptimal",
            Self::Vigesimal => "vigesimal",
            Self::Niftimal => "niftimal",
            Self::Centesimal => "centesimal",
        }
    }

    fn prefix_name(&self) -> &'static str {
        match self {
            Self::Binary => "bi",
            Self::Trinary => "tri",
            Self::Quaternary => "tetra",
            Self::Quinary => "penta",
            Self::Seximal => "hexa",
            Self::Septimal => "hepta",
            Self::Octal => "octo",
            Self::Nonary => "enna",
            Self::Decimal => "deca",
            Self::Elevenary => "leva",
            Self::Dozenal => "doza",
            Self::BakersDozenal => "baker",
            Self::Hex => "tesser",
            Self::Suboptimal => "mal",
            Self::Vigesimal => "icosi",
            Self::Niftimal => "feta",
            Self::Centesimal => "hecto",
        }
    }

    fn suffix_name(&self) -> &'static str {
        match self {
            Self::Decimal => "gesimal",
            Self::BakersDozenal => "ker's dozenal",
            _ => self.name(),
        }
    }

    fn to_number(&self) -> u8 {
        *self as u8
    }
}

fn num_roots_in_name(n: i64, prefix: bool, cache: &mut Cache) -> usize {
    if n < 0 { panic!() }
    if n == 1 { return 1 }
    if let Some(_) = Root::from_number(n) {
        1
    } else {
        let (a, b) = closest_factors(n, cache);
        if a == 1 {
            if prefix {
                2 + num_roots_in_name(n - 1, true, cache)
            } else {
                1 + num_roots_in_name(n - 1, false, cache)
            }
        } else {
            num_roots_in_name(a, true, cache) + num_roots_in_name(b, prefix, cache)
        }
    }
}

fn sqrt(n: i64) -> i64 {
    if n < 2 { return n }
    let mut a = 1255;
    let mut b = n / a;
    a = (a + b) / 2;
    b = n / a;
    a = (a + b) / 2;
    b = n / a;
    a = (a + b) / 2;
    b = n / a;
    a = (a + b) / 2;
    a + 1
}

fn abbr_in_use(abbr: &str, cache: &mut Cache) -> bool {
    for a in &cache.abbreviations {
        if a == abbr {
            return true;
        }
    }
    false
}

fn get_abbr(name: &str, k: usize, res: &mut String) {
    for (_, ch) in name.char_indices().filter(|(i, _)| k & (1 << i) != 0) {
        res.push(ch);
    }
}

pub(crate) fn find_abbreviation(n: i64, cache: &mut Cache) -> &str {
    assert!(n >= 1);
    let n: usize = convert::TryFrom::try_from(n).unwrap();
    for num in cache.abbreviations.len()..=n {
        let num: i64 = convert::TryFrom::try_from(num).unwrap();
        let mut name = BaseName(Base::new(num, cache), true).to_string();
        name.make_ascii_uppercase();
        name = name
            .char_indices()
            .filter(|(_, c)| { *c != ' ' && *c != '\'' })
            .filter(|(i, c)| *i < 3 || !"AEIOU".contains(*c))
            .map(|(_, c)| c.to_ascii_uppercase())
            .collect();
        let first_char = name.chars().next().unwrap();
        let mut abbr = String::from(first_char);
        let name = name.split_at(1).1;
        let mut first = true;
        'outer: for abbr_len in 3.. {
            for k in 0.. {
                if usize::count_ones(k) != abbr_len - 1 {
                    continue
                }
                if !first && k & (1 << name.len() as u32 - 1) != 0 {
                    break;
                }
                get_abbr(&name, k, &mut abbr);
                first = false;
                if !abbr_in_use(&abbr, cache) {
                    break 'outer;
                }
                abbr.clear();
                abbr.push(first_char);
            }
        }
        cache.abbreviations.push(abbr);
    }
    return cache.abbreviations[n].as_str();
}

// input: >= 2
// output: (1.., 2..)
fn closest_factors(n: i64, cache: &mut Cache) -> (i64, i64) {
    if n < 2 { panic!() }
    if let Some(res) = cache.factors.get(&n) {
        return *res;
    }
    let mut res = (1, n);
    let mut root_count = usize::MAX;
    let loop_max = sqrt(n);
    for smaller_factor in (2..loop_max.min(n)).rev() {
        if n % smaller_factor != 0 { continue }
        let larger_factor = n / smaller_factor;
        let (smaller_factor, larger_factor) = if larger_factor < smaller_factor { 
            (larger_factor, smaller_factor)
        } else {
            (smaller_factor, larger_factor)
        };
        let this_root_count = num_roots_in_name(smaller_factor, false, cache) + num_roots_in_name(larger_factor, false, cache);
        // if n == 646 {
        //     eprintln!("{} = {} * {} ({} roots)", n, smaller_factor, larger_factor, this_root_count);
        // }
        if this_root_count > root_count { continue }
        if this_root_count < root_count {
            root_count = this_root_count;
            res = (smaller_factor, larger_factor);
        }
        if larger_factor - smaller_factor < res.1 - res.0 {
            res = (smaller_factor, larger_factor)
        }
    }
    cache.factors.insert(n, res);
    res
}

/// Used to cache intermediate calculations
#[derive(Default)]
pub struct Cache {
    factors: HashMap<i64, (i64, i64)>,
    abbreviations: Vec<String>,
}

pub(crate) enum Base {
    Nullary,
    Unary,
    Root(Root),
    FactorPair(Box<Base>, Box<Base>),
    Prime(Box<Base>), // un- prefix
    Nega(Box<Base>),
    Vot(Box<Base>, Box<Base>),
    CustomLessThanSix(String),
    Imal(String), // greater than six, one syllable
    Al(String), // greater than six, more than one syllable
}

fn is_vowel_or_y(ch: char) -> bool {
    matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

impl Base {
    pub(crate) fn new_frac(num: i64, den: i64, cache: &mut Cache) -> Self {
        if den == 1 {
            Self::new(num, cache)
        } else {
            Self::Vot(Box::new(Self::new(num, cache)), Box::new(Self::new(den, cache)))
        }
    }

    pub(crate) fn new(n: i64, cache: &mut Cache) -> Self {
        if n < 0 { return Self::Nega(Box::new(Self::new(-n, cache))) }
        if n == 0 { return Self::Nullary };
        if n == 1 { return Self::Unary };
        if let Some(root) = Root::from_number(n) {
            return Self::Root(root)
        }
        let (a, b) = closest_factors(n, cache);
        if a == 1 {
            // prime base
            return Self::Prime(Box::new(Base::new(b - 1, cache)));
        }
        let a = Base::new(a, cache);
        let b = Base::new(b, cache);
        Self::FactorPair(Box::new(a), Box::new(b))
    }

    pub(crate) fn new_custom(s: &str, greater_than_six: bool, one_syllable: bool) -> Self {
        if !greater_than_six {
            Self::CustomLessThanSix(s.to_string())
        } else if one_syllable {
            Self::Imal(s.to_string())
        } else {
            Self::Al(s.to_string())
        }
    }

    pub(crate) fn try_parse(s: &str) -> Option<Self> {
        Some(Self::Root(Root::parse(s)?))
    }

    pub(crate) fn to_number(&self) -> i64 {
        match self {
            Self::Nullary => 0,
            Self::Unary => 1,
            Self::Root(r) => r.to_number().into(),
            Self::FactorPair(a, b) => a.to_number() * i64::from(b.to_number()),
            Self::Prime(one_below) => i64::from(one_below.to_number()) + 1,
            Self::Nega(n) => -n.to_number(),
            Self::Vot(num, den) => {
                if den.to_number() == 1 {
                    num.to_number()
                } else {
                    panic!("non-integer base")
                }
            }
            Self::CustomLessThanSix(_) => panic!("unknown number"),
            Self::Imal(_) => panic!("unknown number"),
            Self::Al(_) => panic!("unknown number"),
        }
    }

    fn prefix_name(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Root(r) => write!(f, "{}", r.prefix_name()),
            Self::Prime(one_below) => {
                write!(f, "hen")?;
                one_below.prefix_name(f)?;
                write!(f, "sna")
            }
            Self::FactorPair(a, b) => {
                a.prefix_name(f)?;
                b.prefix_name(f)
            }
            _ => panic!(),
        }
    }

    fn suffix_name(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Root(r) => write!(f, "{}", r.suffix_name()),
            _ => self.format_name(f)
        }
    }

    pub(crate) fn format_name(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Nullary => write!(f, "{}", "nullary"),
            Self::Unary => write!(f, "{}", "unary"),
            Self::Root(r) => write!(f, "{}", r.name()),
            Self::FactorPair(a, b) => {
                a.prefix_name(f)?;
                b.suffix_name(f)
            }
            Self::Prime(one_below) => {
                write!(f, "un")?;
                one_below.format_name(f)
            }
            Self::Nega(n) => {
                write!(f, "nega")?;
                n.format_name(f)
            }
            Self::Vot(a, b) => {
                if a.to_number() != 1 {
                    a.prefix_name(f)?;
                }
                write!(f, "vot")?;
                b.format_name(f)
            }
            Self::CustomLessThanSix(s) => {
                write!(f, "{}", s)?;
                if s.ends_with(is_vowel_or_y) {
                    write!(f, "nary")
                } else {
                    write!(f, "ary")
                }
            }
            Self::Imal(s) => {
                write!(f, "{}imal", s)
            }
            Self::Al(s) => {
                write!(f, "{}al", s)
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum EndVowel {
    None,
    I,
    A,
    O,
}

fn fixup_vowels(s: &str) -> String {
    let mut res = String::new();
    let mut prev = None;
    for ch in s.chars() {
        match (prev, ch) {
            (Some('i'), 'i' | 'u') => {
                res.push('i');
                prev = None;
            }
            (Some('a' | 'o'), 'o' | 'e' | 'i' | 'u') => {
                res.push(ch);
                prev = None;
            }
            (Some(p), _) => {
                res.push(p);
                prev = Some(ch);
            }
            (None, _) => prev = Some(ch),
        }
    }
    if let Some(p) = prev {
        res.push(p);
    }
    res
}

struct InternalName<'a>(&'a Base);

impl fmt::Display for InternalName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.format_name(f)
    }
}

/// Represents the name of a number base (bool: fixup)
pub struct BaseName(pub(crate) Base, pub(crate) bool);

impl fmt::Display for BaseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = InternalName(&self.0).to_string();
        if self.1 {
            write!(f, "{}", fixup_vowels(&s))
        } else {
            write!(f, "{}", &s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    struct PrefixName(Base);
    impl fmt::Display for PrefixName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.prefix_name(f)
        }
    }

    #[track_caller]
    fn check_name(n: i64, s: &str, cache: &mut Cache) {
        assert_eq!(BaseName(Base::new(n, cache), true).to_string(), s);
    }

    #[track_caller]
    fn check_prefix(n: i64, s: &str, cache: &mut Cache) {
        assert_eq!(PrefixName(Base::new(n, cache)).to_string(), s);
    }

    #[test]
    fn nested_prefix_names() {
        let mut cache = Cache::default();
        check_name(19, "untriseximal", &mut cache);
        check_name(23, "unbielevenary", &mut cache);
        check_name(29, "untetraseptimal", &mut cache);
        check_name(31, "unpentaseximal", &mut cache);

        check_prefix(19, "hentrihexasna", &mut cache);
        check_prefix(23, "henbilevasna", &mut cache);
        check_prefix(29, "hentetraheptasna", &mut cache);
        check_prefix(31, "henpentahexasna", &mut cache);

        check_name(646, "hentrihexasnabisuboptimal", &mut cache);
    }

    #[test]
    fn more_than_two_factors() {
        let mut cache = Cache::default();
        check_name(98, "heptabiseptimal", &mut cache);
        check_name(600, "hexacentesimal", &mut cache);
    }

    #[test]
    fn vowel_changes() {
        let mut cache = Cache::default();
        check_name(20 * 64, "icosioctoctal", &mut cache);
        check_name(22, "bielevenary", &mut cache);
        check_name(20 * 20 * 401, "icosicosinicosivigesimal", &mut cache);
        check_name(38, "bintriseximal", &mut cache);

        check_name(32, "tetroctal", &mut cache);
        check_name(44, "tetrelevenary", &mut cache);
        check_name(140 * 140, "hepticosiheptavigesimal", &mut cache);
        check_name(4 * 19, "tetruntriseximal", &mut cache);

        check_name(64, "octoctal", &mut cache);
        check_name(88, "octelevenary", &mut cache);
        check_name(2000 * 2000, "icosihecticosicentesimal", &mut cache);
        check_name(8 * 19, "octuntriseximal", &mut cache);
    }

    #[test]
    fn special_names() {
        let mut cache = Cache::default();
        check_name(1, "unary", &mut cache);
        check_name(0, "nullary", &mut cache);
        check_name(-2, "negabinary", &mut cache);
        check_name(-10, "negadecimal", &mut cache);
    }

    #[test]
    fn long_names() {
        let mut cache = Cache::default();
        check_name(841, "hentetraheptasnuntetraseptimal", &mut cache);
        check_name(6254, "henbihentetraheptasnasnabintetraker's dozenal", &mut cache);
        check_name(5758, "binbinbinbinbinbinoctelevenary", &mut cache);
    }

    #[test]
    fn test_num_roots() {
        let mut cache = Cache::default();
        assert_eq!(num_roots_in_name(76, false, &mut cache), 4);
        assert_eq!(num_roots_in_name(95, false, &mut cache), 4);
        assert_eq!(num_roots_in_name(20, false, &mut cache), 1);

        check_name(361, "hentrihexasnuntriseximal", &mut cache);

        // hen-tri-hexa-sn-un-tri-seximal
        assert_eq!(num_roots_in_name(361, false, &mut cache), 7);

        assert_eq!(num_roots_in_name(17, false, &mut cache), 1);
        assert_eq!(num_roots_in_name(19, false, &mut cache), 3);
        assert_eq!(num_roots_in_name(34, false, &mut cache), 2);
        assert_eq!(num_roots_in_name(38, false, &mut cache), 4);

        check_name(7220, "tetrahentrihexasnapentuntriseximal", &mut cache);
    }

    #[test]
    fn test_get_abbr() {
        fn get_abbr_test(s: &str, k: usize) -> String {
            let mut res = String::new();
            get_abbr(s, k, &mut res);
            res
        }
        assert_eq!(get_abbr_test("abcdefg", 0), "");
        assert_eq!(get_abbr_test("abcdefg", 1), "a");
        assert_eq!(get_abbr_test("abcdefg", 2), "b");
        assert_eq!(get_abbr_test("abcdefg", 3), "ab");
        assert_eq!(get_abbr_test("abcdefg", 4), "c");
        assert_eq!(get_abbr_test("abcdefg", 5), "ac");
        assert_eq!(get_abbr_test("abcdefg", 6), "bc");
        assert_eq!(get_abbr_test("abcdefg", 7), "abc");
        assert_eq!(get_abbr_test("abcdefg", 8), "d");
    }

    #[test]
    fn find_abbr() {
        let mut cache = Cache::default();
        assert_eq!(find_abbreviation(1, &mut cache), "UNA");
        assert_eq!(find_abbreviation(16, &mut cache), "HEX");
        assert_eq!(find_abbreviation(40, &mut cache), "PEC");
        assert_eq!(find_abbreviation(100, &mut cache), "CEN");
        assert_eq!(find_abbreviation(200, &mut cache), "DEV");
        assert_eq!(find_abbreviation(300, &mut cache), "TCN");
        assert_eq!(find_abbreviation(400, &mut cache), "ICO");
        assert_eq!(find_abbreviation(500, &mut cache), "PCN");
        assert_eq!(find_abbreviation(585, &mut cache), "BAKR");
        assert_eq!(find_abbreviation(841, &mut cache), "HSS");
        assert_eq!(find_abbreviation(969, &mut cache), "HBM");
        assert_eq!(find_abbreviation(1000, &mut cache), "DCS");
        //assert_eq!(find_abbreviation(5758, &mut cache), "BBBC");
        //assert_eq!(find_abbreviation(6254, &mut cache), "HHTK");
    }
}
