use anyhow::{anyhow, Error, Ok, Result};
use num::{BigUint, Integer, One};
use num_prime::{
    nt_funcs::{factorize, is_prime},
    Primality,
};
use rand::Rng;
use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

/// Parses unique factorization given in format e.g., 2^2,3^1
/// TODO: why BTreeMap again? Why not hashmap with keys as prime factors and values as exponents?
/// I suspect it's because originally I was going to allow same prime factor to show up in
/// input multiple times, but not anymore...
pub fn parse_cli_factorization(factors: &str) -> Result<BTreeMap<BigUint, usize>, Error> {
    let factor_split = factors
        .trim()
        .split(',')
        .map(|prime_power| {
            prime_power.trim().split_once("^").ok_or(anyhow!(
                "failed to find '^' character when
                    parsing factorization"
            ))
        })
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let mut prime_factors: HashSet<BigUint> = HashSet::new(); // used to detect duplicate prime factors
    let (successes, errors): (Vec<_>, Vec<_>) = factor_split
        .into_iter()
        .map(|(q, e)| (BigUint::from_str(q), usize::from_str(e)))
        .partition(|(q, e)| {
            q.is_ok()
                && e.is_ok()
                && is_prime(q.as_ref().unwrap(), None) == Primality::Yes
                && prime_factors.insert(q.as_ref().unwrap().clone()) // insert returns false if dupe
        });
    if errors.is_empty() {
        Ok(successes
            .into_iter()
            .map(|(q, e)| (q.unwrap(), e.unwrap()))
            .collect::<BTreeMap<BigUint, usize>>())
    } else {
        Err(anyhow!("Error parsing factorization: {:?}", errors))
    }
}

/// Computes gcd of a and b in naive recursive manner
/// Takes time linear in input: O(log(a) + log(b))
/// This is generally the fastest way of computing gcd
/// without getting into optimizations for special cases
/// like small numbers vs. large numbers.
/// See https://stackoverflow.com/questions/22281661/what-is-the-fastest-way-to-find-the-gcd-of-two-numbers
/// for more info.
pub fn gcd<T: Integer + Clone>(a: T, b: T) -> T {
    if b.is_zero() {
        return a;
    } else {
        return gcd(b.clone(), a % b);
    }
}

/// Computes euler's totient function for n. Takes prime factorization
/// of n as optional argument, as this is necessary for computation
/// anyway, since phi = \prod q_i^{e_i-1} * (q_i-1), where n = \prod q_i^{e_i}
pub fn eulers_phi(n: BigUint, factors: Option<BTreeMap<BigUint, usize>>) -> BigUint {
    let factors: BTreeMap<BigUint, usize> = match factors {
        Some(factors) => factors,
        None => factorize(n),
    };
    factors.into_iter().fold(BigUint::one(), |acc, (q, e)| {
        acc * q.pow((e - 1).try_into().unwrap()) * (q - BigUint::one())
    })
}

/// Compute generator for Z_p^*, where p is a prime modulus.
/// Uses algorithm from section 11.1 in Shoup's
/// "A Computational Introduction to Number Theory and "Algebra".
/// Basic idea is to randomly
/// TODO: allow factorization of p as optional parameter
/// TODO: generalize to Z_n^*
pub fn get_generator(p: BigUint) -> Result<BigUint> {
    if is_prime(&p, None) != Primality::Yes {
        return Err(anyhow!("argument is not a prime"));
    }
    // find a generator for Z_p^* using Shoup's algorithm
    // WARNING: factoring p-1 could be very slow!
    let mut rng = rand::thread_rng();
    let mut gen_factors = Vec::<BigUint>::new();
    let gp_size = p.clone() - BigUint::one();
    let factors = factorize(gp_size.clone());
    factors
        .into_iter()
        .map(|(q, e)| (q, BigUint::from(e)))
        .for_each(|(q, e)| {
            // for each prime factor and exponent...
            // a is used for randomly sampled elements of Z_p^*
            // b is set to a^{(p-1)/q_i} and checked to see if equals 1
            let mut a = BigUint::default(); // default is zero
            let mut b = BigUint::one();
            while b.is_one() {
                a = BigUint::from(rng.gen_range(BigUint::one()..p.clone()));
                // b = a.modpow(&(gp_size.clone() / q.clone().modpow(&e, &p)), &p); // BUG?
                b = a.modpow(&(gp_size.clone() / q.clone()), &p);
            }
            let exponent = gp_size.clone() / q.clone().modpow(&e, &p);
            gen_factors.push(a.modpow(&exponent, &p));
        });
    let generator = gen_factors
        .into_iter()
        .reduce(|acc, y| acc * y % p.clone())
        .unwrap();
    Ok(generator)
}

/// Test whether g is a generator for the group Z_p^*, where p is
/// a prime modulus.
/// TODO: allow factorization of p as optional parameter
/// TODO: generalize to Z_n^*
pub fn is_generator(g: BigUint, p: BigUint) -> Result<bool> {
    if is_prime(&p, None) != Primality::Yes {
        return Err(anyhow!("argument is not a prime"));
    }
    let gp_size = p.clone() - BigUint::one();
    let factors = factorize(gp_size.clone());
    Ok(factors
        .into_iter()
        .map(|(q, e)| (q, BigUint::from(e)))
        .all(|(q, e)| {
            g.modpow(&(gp_size.clone() / q.clone().modpow(&e, &p)), &p) != BigUint::one()
        }))
}
