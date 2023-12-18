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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Doctor(_) => {}
        Commands::Test(test) => {
            let file = &test.file;
        }
        Commands::Run(run) => match run {
            Run::Server(server) => {
                let port = server.port;
            }
            Run::Local(local) => {
                let directory = &local.directory;
                let suffix = &local.suffix;
            }
        },
    }
}
