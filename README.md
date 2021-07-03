# bases

Rust implementation of jan Misali's number base algorithm (see [a base-neutral system for naming numbering systems](https://youtu.be/7OEF3JD-jYo) and https://www.seximal.net/names-of-other-bases).

## Installation and Usage:

```bash
# install with cargo
cargo install bases

# print the name of base 6 (seximal)
bases 6

# print names and abbreviations of all bases
bases
```

## Using `bases` as a library:

This crate can also be used as a library, check out [docs.rs/bases](https://docs.rs/bases) for more info.

## Examples:

```
$ bases 6
seximal
$ bases
1: unary (UNA)
2: binary (BIN)
3: trinary (TRI)
4: quaternary (QUA)
5: quinary (QUI)
6: seximal (SEX)
7: septimal (SEP)
8: octal (OCT)
9: nonary (NON)
10: decimal (DEC)
11: elevenary (ELE)
12: dozenal (DOZ)
13: baker's dozenal (BAK)
14: biseptimal (BIS)
15: triquinary (TRQ)
16: hex (HEX)
17: suboptimal (SUB)
18: triseximal (TRS)
19: untriseximal (UNT)
20: vigesimal (VIG)
21: triseptimal (TIS)
22: bielevenary (BIE)
23: unbielevenary (UNB)
24: tetraseximal (TET)
25: pentaquinary (PEN)
26: biker's dozenal (BIK)
27: trinonary (TRN)
28: tetraseptimal (TER)
29: untetraseptimal (UTT)
30: pentaseximal (PET)
...
```