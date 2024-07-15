mod brew;
mod dev;
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[clap(name = "dotfiles_manager")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    InstallBrew {
        #[clap(required = false)]
        apps: Vec<String>,
    },
    InstallAllBrew,
    InstallDevSetup,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::InstallBrew { apps } => {
            if apps.is_empty() {
                println!("No apps specified. Installing all predefined apps...");
                brew::install_apps(brew::get_all_package_names()).await?;
            } else {
                brew::install_apps(apps).await?;
            }
        }
        Command::InstallAllBrew => {
            brew::install_apps(brew::get_all_package_names()).await?;
        }
        Command::InstallDevSetup => {

        }
        }
    }

    Ok(())
}
