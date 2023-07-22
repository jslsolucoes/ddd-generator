use clap::{Args, Parser, Subcommand};

mod cmd;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "create a new use case template", )]
    #[command(aliases = & ["uc"])]
    UseCase(UseCaseArgs),
}

#[derive(Args, Debug)]
struct UseCaseArgs {
    #[arg(short = 'p', long = "package")]
    package: String,
    #[arg(short = 'n', long = "name")]
    name: String,
    #[arg(short = 'f', long = "fields")]
    fields: Vec<String>,
}

fn main() -> Result<(), handlebars::RenderError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::UseCase(use_case_args) => {
            let use_case_input = cmd::uc::UseCaseInput {
                package: use_case_args.package.to_string(),
                name: use_case_args.name.clone(),
                fields: use_case_args.fields.clone(),
            };
            let result = cmd::uc::generate(use_case_input)?;
            match result {
                (_, file_name) => println!("Created file: {:?}", file_name)
            }
        }
    }

    Ok(())
}
