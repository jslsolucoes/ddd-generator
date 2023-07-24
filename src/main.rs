use clap::{Args, Parser, Subcommand};

mod cmd;
mod common;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'm', long = "mode")]
    #[clap(value_enum, default_value_t = cmd::Mode::Execute)]
    mode: cmd::Mode,
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

fn main() -> Result<(), cmd::uc::GenerateUseCaseError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::UseCase(use_case_args) => {
            let use_case_input = cmd::uc::GenerateUseCaseInput {
                mode: cli.mode.clone(),
                package: use_case_args.package.clone(),
                name: use_case_args.name.clone(),
                fields: use_case_args.fields.clone(),
            };
            let result = cmd::uc::generate(&use_case_input);
            match result {
                Ok(output) => {
                    match use_case_input.mode {
                        cmd::Mode::Simulate => {
                            return Ok(());
                        }
                        cmd::Mode::Execute => println!("File {} created", output.files[0].file_name)
                    }
                }
                Err(err) => {
                    match err {
                        cmd::uc::GenerateUseCaseError::RenderError => {
                            println!("Error rendering template {:?}", err)
                        }
                        cmd::uc::GenerateUseCaseError::TemplateError => {
                            println!("Error registering template {:?}", err)
                        }
                        cmd::uc::GenerateUseCaseError::Error => {
                            println!("Error creating file {:?}", err)
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
