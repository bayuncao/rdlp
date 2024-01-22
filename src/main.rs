mod conf;
mod engine;
mod logger;
mod result;

use crate::conf::conf::Conf;
use crate::engine::engine::Worker;
use crate::logger::logger::setup_logger;
use clap::{Args, Parser, Subcommand};

use glob::glob;
use tokio::fs;
use tokio::io::AsyncReadExt;

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
    Run(Run),
}


#[derive(Args)]
struct Run {
    #[command(subcommand)]
    command: RunCommands,
}

#[derive(Subcommand)]
enum RunCommands {
    Local(Local),
    Server(Server),
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
    log_output: String,
    #[arg(
    short = 'c',
    long = "config",
    default_value = "development.toml",
    help = "Path to the configuration file (default is <development.toml>)"
    )]
    config_file: String,

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
    log_output: String,
    #[arg(
    short = 'c',
    long = "config",
    default_value = "development.toml",
    help = "Path to the configuration file (default is <development.toml>)"
    )]
    config_file: String,

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Commands::Run(run) => match &run.command {


            RunCommands::Server(server) => {
                let config = load_config(&server.config_file)?;
                setup_logger(&config.logger)?;
                start_server(server.port).await?;
            }
            RunCommands::Local(local) => {
                let config = load_config(&local.config_file)?;
                setup_logger(&config.logger)?;
                let directory = &local.directory;
                let suffix = &local.suffix;

                // Initialize Worker
                let worker = Worker::new().unwrap(); // Handle error as needed

                // Async read files and apply detection
                let pattern = format!("{}/**/*{}", directory, suffix);
                for entry in glob(&pattern).expect("Failed to read glob pattern") {
                    match entry {
                        Ok(path) => {
                            match fs::read_to_string(path).await {
                                Ok(content) => {
                                    let results = worker.detect(&content);
                                    // Process and output results here
                                }
                                Err(e) => {
                                    eprintln!("Failed to read file: {}", e);
                                    // Continue to next file
                                }
                            }
                        }
                        Err(e) => eprintln!("Error reading path: {}", e),
                    }
                }
            }
        },
    }
    Ok(())
}

pub fn validate_config(file: &str) -> Result<(), &'static str> {
    let conf_string =
        std::fs::read_to_string(file).map_err(|_| "Failed to read configuration file")?;
    let conf: Conf =
        toml::from_str(&conf_string).map_err(|_| "Failed to parse configuration file")?;

    // Check each configuration item
    // This is a placeholder, replace it with your actual validation logic
    for item in &conf.rules.item {
        let _ = item;
    }

    Ok(())
}

async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Use 'actix-web' or similar library to start an HTTP server
    // Example with 'actix-web':
    use actix_web::{web, App, HttpServer, Responder};

    async fn index() -> impl Responder {
        "Hello, world!"
    }

    HttpServer::new(|| {
        App::new().route("/", web::post().to(index)) // Handle POST requests
    })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await?;

    Ok(())
}

pub fn load_config(file: &str) -> Result<Conf, Box<dyn std::error::Error>> {
    let conf_string = std::fs::read_to_string(file)?;
    let conf: Conf = toml::from_str(&conf_string)?;
    Ok(conf)
}