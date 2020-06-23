use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tsunammi")]
pub struct Params {
    #[structopt(short = "d", long = "Test target (domain)")]
    domain: String,
    #[structopt(short = "t", long = "Timeout (second)")]
    timeout: u16,
    #[structopt(short = "c", long = "Concurrency number",  default_value = "15000")]
    concurrency:u16,
}

