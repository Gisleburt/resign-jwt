use structopt::StructOpt;


#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    jwt: String,
    #[structopt(short, long)]
    key: String,
    #[structopt(short, long)]
    algorithm: String,
}


fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
