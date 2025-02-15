use clap::{Parser, Subcommand};
use cli::processes::*;

#[derive(Parser)]
#[command(name = "cli")]
#[command(about = "Veamos")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Inicializa el proyecto",
        long_about = "Este comando inicializa el proyecto con la configuración predeterminada."
    )]
    Init,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init => {
            let base = "tal";
            create_dir_all(&format!("{}/components", base));
            create_dir_all(&format!("{}/systems", base));

            let content = r#"
#ifndef _CONSTANTS_H
#define _CONSTANTS_H

#define WINDOW_ZOOM 4
#define WINDOW_WIDTH 1600
#define WINDOW_HEIGHT 900
#define WINDOW_TITLE "Game"
#define WINDOW_FPS 60
#define WINDOW_BACKGROUND (Color){80, 80, 140, 255}

typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned int u32;
typedef char i8;
typedef short i16;
typedef int i32;

#endif
"#;
            create_file(&format!("{}/constants.h", base), content);
        }
    }
}
