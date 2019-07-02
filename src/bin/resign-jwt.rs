use structopt::StructOpt;
use resign_jwt::Algorithm;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    jwt: String,
    #[structopt(short, long)]
    key: String,
    #[structopt(short, long)]
    algorithm: Algorithm,
}


fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
