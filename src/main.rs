
/// TODO: should probably have third option to generate
/// prime of given size along with factorization and generator
/// using algorithm given in Shoup.
/// 
/// TODO: allow factorization as optional parameter to 
/// get_generator
/// 
/// TODO: DIY gcd
/// 
/// TODO: Bezout coefficients

use anyhow::{anyhow, Error, Ok, Result};
use clap::{Args, Parser, Subcommand};
use num::{BigUint, One, Zero};
use std::{collections::BTreeMap, str::FromStr};
use num_prime::{Primality, nt_funcs::{is_prime, factorize}};
use rand::Rng;


#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gcd(GcdArgs),
    Phi(PhiArgs), // Euler's totient phi function
    Factorize(FactorizeArgs), // prime factorization
    GetGenerator(GetGeneratorArgs), // get a generator for Z_p^* -- think this can be easily generalized to non-prime moduli...
    IsGenerator(IsGeneratorArgs) // check whether number is generator for Z_p^*
}

#[derive(Args, Debug)]
struct GcdArgs {
    #[arg(short, long)]
    a: String,
    #[arg(short, long)]
    b: String
}

#[derive(Args, Debug)]
struct PhiArgs {
    #[arg(short, long)]
    n: String,
    #[arg(short, long)]
    factors: Option<String>
}

#[derive(Args, Debug)]
struct FactorizeArgs {
    #[arg(short, long)]
    n: String
}

#[derive(Args, Debug)]
struct GetGeneratorArgs {
    #[arg(short, long)]
    modulus: String // prime modulus
}

#[derive(Args, Debug)]
struct IsGeneratorArgs {
    #[arg(short, long)]
    candidate_gen: String, // candidate generator
    #[arg(short, long)]
    modulus: String // prime modulus 
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gcd(gcd_args) => {
            let a = BigUint::from_str(&gcd_args.a)?;
            let b = BigUint::from_str(&gcd_args.b)?;
            let d = gcd(a.clone(),b.clone());
            println!("gcd({:?}, {:?}) = {:?}", a, b, d);
            Ok(())
        }
        Commands::Phi(phi_args) => {
            let n = BigUint::from_str(&phi_args.n)?;
            let factors = match &phi_args.factors {
                Some(cli_factors) => Some(parse_cli_factors(cli_factors)?),
                None => None
            };
            let phi = eulers_phi(n.clone(), factors);
            println!("phi({:?}) = {:?}", n, phi);
            Ok(())
        },
        Commands::Factorize(factorize_args) => {
            let n = BigUint::from_str(&factorize_args.n)?;
            let mut factors = factorize(n.clone()).into_iter().peekable();
            let mut output = format!("{:?} = ", n);
            while let Some((q, e)) = factors.next() {
                if factors.peek().is_none() {
                    output.push_str(&format!("{:?}^{:?}", q, e))
                } else {
                    output.push_str(&format!("{:?}^{:?} \u{00b7} ", q, e));
                }
            }
            println!("{output}");
            Ok(())
        },
        Commands::GetGenerator(get_generator_args) => {
            println!("args: {:?}", get_generator_args);
            // check genargs.modulus is prime
            let p = BigUint::from_str(&get_generator_args.modulus)?;
            let generator = get_generator(p)?;
            println!("generator: {:?}", generator);
            Ok(())
        },
        Commands::IsGenerator(is_generator_args) => {
            println!("args: {:?}", is_generator_args);
            let g = BigUint::from_str(&is_generator_args.candidate_gen)?;
            let p = BigUint::from_str(&is_generator_args.modulus)?;
            if is_generator(g, p)? {
                println!("Yes");
            } else {
                println!("No");
            }
            Ok(())
        }
    }
}

fn parse_cli_factors(factors: &str) -> Result<BTreeMap<BigUint, usize>, Error> {
    let factor_split = factors
        .trim()
        .split(',')
        .map(|prime_power| prime_power
            .split_once("^")
            .ok_or(anyhow!("failed to find '^' character when parsing factorization")))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let (successes, errors): (Vec<_>, Vec<_>) = factor_split
        .into_iter()
        .map(|(q, e)| (BigUint::from_str(q), usize::from_str(e)))
        .partition(|(q,e)| q.is_ok() && e.is_ok());
    if errors.is_empty() {
        Ok(successes.into_iter()
            .map(|(q,e)| (q.unwrap(), e.unwrap()))
            .collect::<BTreeMap<BigUint, usize>>()) 
    } else {
        Err(anyhow!("Error parsing factorization: {:?}", errors))
    }
}

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    if b == BigUint::zero() {
        return a;
    } else {
        return gcd(b.clone(), a % b);
    }
}

fn eulers_phi(n: BigUint, factors: Option<BTreeMap<BigUint, usize>>) -> BigUint {
    let factors: BTreeMap<BigUint, usize> = match factors {
        Some(factors) => factors,
        None => factorize(n)
    };
    factors
        .into_iter()
        .fold(BigUint::one(), |acc, (q,e)| {
            acc * q.pow((e-1).try_into().unwrap()) * (q - BigUint::one())
    })
}


fn get_generator(p: BigUint) -> Result<BigUint> {
    if is_prime(&p, None) != Primality::Yes {
        return Err(anyhow!("argument is not a prime"));
    }
    // find a generator for Z_p^* using Shoup's algorithm 
    // WARNING: factoring p-1 could be very slow!
    let mut rng = rand::thread_rng();
    let mut gen_factors = Vec::<BigUint>::new();
    let gp_size = p.clone() - BigUint::one();
    let factors = factorize(gp_size.clone());
    factors.into_iter()
        .map(|(q, e)| (q, BigUint::from(e)))
        .for_each(|(q, e)| { // for each prime factor and exponent...
            // a is used for randomly sampled elements of Z_p^*
            // b is set to a^{(p-1)/q_i} and checked to see if equals 1
            let mut a = BigUint::default(); // default is zero
            let mut b = BigUint::one(); 
            while b.is_one() {
                a = BigUint::from(rng.gen_range(BigUint::one()..p.clone()));
                b = a.modpow(&(gp_size.clone() / q.clone().modpow(&e, &p)), &p);
            }
            let exponent = gp_size.clone() / q.clone().modpow(&e, &p);
            gen_factors.push(a.modpow(&exponent, &p));
        });
    let generator = gen_factors
        .into_iter()
        .reduce(|acc, y| acc * y % p.clone()).unwrap();
    Ok(generator)
}

fn is_generator(g: BigUint, p: BigUint) -> Result<bool> {
    if is_prime(&p, None) != Primality::Yes {
        return Err(anyhow!("argument is not a prime"));
    }
    let gp_size = p.clone() - BigUint::one();
    let factors = factorize(gp_size.clone());
    Ok(
        factors.into_iter()
            .map(|(q, e)| (q, BigUint::from(e)))
            .all(|(q, e)| 
                g.modpow(&(gp_size.clone() / q.clone().modpow(&e, &p)), &p) != BigUint::one()
            )
    )
}
