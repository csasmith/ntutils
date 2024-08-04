extern crate ntutils;

use anyhow::anyhow;
use ntutils::*;
use num::BigUint;
use std::collections::BTreeMap;
use std::iter::zip;

#[test]
fn test_parse_cli_factorization() {
    let malformed_inputs: Vec<&str> = vec![
        "", "^", "2^", "^2", "2^^", "2^^3", "a^b", "2^3^4", "garbage", ",", "2,3", "2^3,",
        "2^3,,3^2", "4^2, 3^6", "2^3 5^2", "2^3,2^5",
    ];
    assert!(malformed_inputs
        .into_iter()
        .all(|m| parse_cli_factorization(m).is_err()));

    let good_inputs: Vec<&str> = vec!["2^1", "2^256", "5^2,3^1", "2^1,3^2, 5^3"];
    let good_outputs: Vec<BTreeMap<BigUint, usize>> = vec![
        BTreeMap::from([(BigUint::from(2u8), 1usize)]),
        BTreeMap::from([(BigUint::from(2u8), 256usize)]),
        BTreeMap::from([(BigUint::from(5u8), 2usize), (BigUint::from(3u8), 1usize)]),
        BTreeMap::from([
            (BigUint::from(2u8), 1usize),
            (BigUint::from(3u8), 2usize),
            (BigUint::from(5u8), 3usize),
        ]),
    ];
    let mut cases = zip(good_inputs, good_outputs);
    assert!(cases.all(|(input, output)| parse_cli_factorization(input).unwrap() == output));
}
