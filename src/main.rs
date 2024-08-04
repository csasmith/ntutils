/// TODO: how to get better CLI help messages?
///
/// TODO: testing!
///
/// TODO: should probably have third option to generate
/// prime of given size along with factorization and generator
/// using algorithm given in Shoup.
/// 
/// TODO: allow factorization as optional parameter to 
/// get_generator
/// 
/// TODO: Bezout coefficients

use anyhow::{Ok, Result};
use clap::{Args, Parser, Subcommand};
use num::BigUint;
use std::str::FromStr;
use num_prime::nt_funcs::factorize;
use ntutils::*;


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
        // compute gcd of {a} and {b}
        Commands::Gcd(gcd_args) => {
            let a = BigUint::from_str(&gcd_args.a)?;
            let b = BigUint::from_str(&gcd_args.b)?;
            let d = gcd(a.clone(),b.clone());
            println!("gcd({:?}, {:?}) = {:?}", a, b, d);
            Ok(())
        },
        // compute Euler's totient of {n}, given optional factorization {factors} of n
        Commands::Phi(phi_args) => {
            let n = BigUint::from_str(&phi_args.n)?;
            let factors = match &phi_args.factors {
                Some(cli_factors) => Some(parse_cli_factorization(cli_factors)?),
                None => None
            };
            let phi = eulers_phi(n.clone(), factors);
            println!("phi({:?}) = {:?}", n, phi);
            Ok(())
        },
        // get prime factorization of {n}
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
        // get generator for Z_p^*, p a prime {modulus}
        Commands::GetGenerator(get_generator_args) => {
            println!("args: {:?}", get_generator_args);
            // check genargs.modulus is prime
            let p = BigUint::from_str(&get_generator_args.modulus)?;
            let generator = get_generator(p)?;
            println!("generator: {:?}", generator);
            Ok(())
        },
        // test if {candidate_gen} is a generator for Z_p^*, p a prime {modulus}
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

