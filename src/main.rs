use clap::{Parser, Subcommand};
use cli::processes::*;

#[derive(Parser)]
#[command(name = "panduwu")]
#[command(about = "panduwu")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Inicializa Pandora (Flecs + Raylib)",
        long_about = "Este comando inicializa el proyecto con la configuración predeterminada."
    )]
    Init { name: String },
    #[command(about = "Crea un componente", long_about = "Crea un componente")]
    Component { name: String },
    #[command(about = "Crea un sistema", long_about = "Crea un sistema")]
    System { name: String },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init { name } => {
            let template_dir = "/usr/local/share/cli";
            let dest_dir = name;
            if let Err(e) = copy_dir(template_dir, &dest_dir) {
                eprintln!("Error copiando template: {}", e);
            }
        }

        Commands::Component { name } => {
            if let Err(e) = create_component(&name) {
                eprintln!("Error creando componente: {}", e);
            }
            if let Err(e) = create_component_header(&name) {
                eprintln!("Error creando componente.h: {}", e);
            }

            if let Err(e) = update_register_c(&name) {
                eprintln!("Error creando componente.h: {}", e);
            }
        }

        Commands::System { name } => {
            if let Err(e) = update_register_systems_c(&name) {
                eprintln!("Error creando system.c: {}", e);
            }

            if let Err(e) = create_system_file(&name) {
                eprintln!("Error creando system.c: {}", e);
            }

            if let Err(e) = update_systems_h(&name) {
                eprintln!("Error creando system.h: {}", e);
            }
        }
    }
}
