#![no_main]

use arbitrary::Arbitrary;
use fuzz_test::parse_integer;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct Input {
    s: String,
}

fuzz_target!(|input: Input| {
    parse_integer(&input.s);
});
