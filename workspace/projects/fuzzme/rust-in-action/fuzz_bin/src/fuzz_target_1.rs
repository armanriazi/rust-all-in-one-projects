#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct Input {
    s: String,
}

fuzz_target!(|input: Input| {
    use fuzz_lib::parse_integer;

    parse_integer(&input.s);
});
