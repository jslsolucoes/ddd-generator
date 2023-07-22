use clap::{Args, Parser, Subcommand};

use crate::cmd::uc::GenerateUseCaseError;

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

fn main() -> Result<(), GenerateUseCaseError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::UseCase(use_case_args) => {
            let use_case_input = cmd::uc::UseCaseInput {
                package: use_case_args.package.clone(),
                name: use_case_args.name.clone(),
                fields: use_case_args.fields.clone(),
            };
            let result = cmd::uc::generate(&use_case_input);
            match result {
                Ok((_, file_name)) => {
                    println!("File {} created", file_name);
                }
                Err(err) => {
                    match err {
                        GenerateUseCaseError::RenderError => {
                            println!("Error rendering template {:?}", err)
                        }
                        GenerateUseCaseError::TemplateError => {
                            println!("Error registering template {:?}", err)
                        }
                        GenerateUseCaseError::Error => {
                            println!("Error creating file {:?}", err)
                        }
                    }
                }
            }


            let result2 = cmd::uc::generate(&use_case_input);
            match result2 {
                Ok((_, file_name)) => {
                    println!("File {} created", file_name);
                }
                Err(err) => {
                    match err {
                        GenerateUseCaseError::RenderError => {
                            println!("Error rendering template {:?}", err)
                        }
                        GenerateUseCaseError::TemplateError => {
                            println!("Error registering template {:?}", err)
                        }
                        GenerateUseCaseError::Error => {
                            println!("Error creating file {:?}", err)
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
