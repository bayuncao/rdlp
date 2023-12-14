mod conf;
mod errors;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rdlp", about = "An example program.")]
pub struct Opt {
    /// File to read
    #[structopt(short = "f", long = "file", default_value = "development.toml")]
    pub file: String,
}

fn main() {
    let opt = Opt::from_args();
    match conf::conf::from_file(&opt.file) {
        Ok(config) => {
            println!("Config: {:?}", config);
        }
        Err(e) => {
            eprintln!("Failed to read config: {}", e);
        }
    }
    println!("Hello, world!");
}
