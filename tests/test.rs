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
    let good_outputs = vec![
        vec![(2, 1)],
        vec![(2, 256)],
        vec![(5, 2), (3, 1)],
        vec![(2, 1), (3, 2), (5, 3)],
    ];
    let good_outputs: Vec<BTreeMap<BigUint, usize>> = good_outputs
        .into_iter()
        .map(|case| {
            case.into_iter()
                .map(|(q, e)| (BigUint::from(q as u8), e as usize))
        })
        .map(|case| BTreeMap::from_iter(case))
        .collect();
    let mut cases = zip(good_inputs, good_outputs);
    assert!(cases.all(|(input, output)| parse_cli_factorization(input).unwrap() == output));
}

#[test]
fn test_gcd() {
    let cases = vec![
        (0, 0, 0),
        (0, 1, 1),
        (1, 0, 1),
        (1, 1, 1),
        (2, 0, 2),
        (0, 2, 2),
        (2, 1, 1),
        (1, 2, 1),
        (2, 2, 2),
        (2, 3, 1),
        (3, 2, 1),
        (2, 4, 2),
        (4, 2, 2),
        (4, 3, 1),
        (4, 4, 4),
        (5, 2, 1),
        (5, 3, 1),
        (5, 4, 1),
    ];
    assert!(cases.into_iter().all(|(a, b, d)| gcd(a, b) == d))
}
