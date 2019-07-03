use resign_jwt::{Algorithm, Token};
use structopt::StructOpt;
use std::str::FromStr;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    pub jwt: String,
    #[structopt(short, long)]
    pub key: String,
    #[structopt(short, long)]
    pub algorithm: Algorithm,
}

fn main() {
    let opt = Opt::from_args();
    let token = Token::from_str(&opt.jwt).unwrap();
    println!("{}", token.sign(&opt.key, opt.algorithm));
}
