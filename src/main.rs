use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "gh-ranking")]
struct Opt {
    #[structopt(name = "ACTION")]
    action: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("Action: {}", &opt.action);
}
