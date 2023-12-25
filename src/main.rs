mod conf;
mod engine;
mod logger;
mod result;
use crate::conf::conf::Conf;
use crate::engine::engine::Worker;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author = "bayuncao",
    version = "1.0.0",
    about = "About rdlp",
    long_about = "Long about rdlp"
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Doctor(Doctor),
    Test(Test),
    #[command(flatten)]
    Run(Run),
}

#[derive(Subcommand)]
enum Run {
    Server(Server),
    Local(Local),
}

#[derive(Args)]
struct Doctor {
    // Add fields here if needed
}

#[derive(Args)]
struct Test {
    #[arg(
        short = 'f',
        long = "file",
        default_value = "development.toml",
        help = "Validate the given configuration file (default is <development.toml>)"
    )]
    file: String,
}

#[derive(Args)]
struct Server {
    #[arg(
        short = 'p',
        long = "port",
        default_value_t = 6000,
        help = "rdlp will open a local port service http://localhost:<:port> to listen for post requests."
    )]
    port: u16,
}

#[derive(Args)]
struct Local {
    #[arg(
        short = 'd',
        long = "directory",
        default_value = "data",
        help = "When running in local mode, read the path where the original text data file is stored."
    )]
    directory: String,
    #[arg(
        short = 's',
        long = "suffix",
        default_value = ".txt",
        help = "When running in local mode, read the suffix of the original text data file(These files need to be in the directory of the -d/--directory parameter)."
    )]
    suffix: String,
}




#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Doctor(_) => {}
        Commands::Test(test) => {
            let file = &test.file;
            match validate_config(file) {
                Ok(_) => println!("Configuration file is valid."),
                Err(e) => println!("Configuration file is invalid: {}", e),
            }
        }
        Commands::Run(run) => match run {
            Run::Server(server) => {
                let port = server.port;
            }
            Run::Local(local) => {
                let directory = &local.directory;
                let suffix = &local.suffix;

                let worker = Worker::new();
                let data: Vec<String> = vec!["text sample 1".to_string(), "text sample 2".to_string()];
                // let rules = vec![Conf::Rules::Item];
                // worker.async_detect(data, rules).await.unwrap();
            }
        },
    }
}




pub fn validate_config(file: &str) -> Result<(), &'static str> {
    let conf_string = std::fs::read_to_string(file).map_err(|_| "Failed to read configuration file")?;
    let conf: Conf = toml::from_str(&conf_string).map_err(|_| "Failed to parse configuration file")?;

    // Check each configuration item
    // This is a placeholder, replace it with your actual validation logic
    for item in &conf.rules.item {
        let _ = item;
    }

    Ok(())
}