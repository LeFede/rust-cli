use clap::{Parser, Subcommand};
use cli::processes::*;
use std::error::Error;

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};

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

fn update_systems_h(name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = "src/systems/systems.h";
    let mut contents = fs::read_to_string(file_path)?;

    // Verificar si ya existe la declaración del sistema
    if contents.contains(&format!("void {}(ecs_iter_t *it);", name)) {
        return Ok(());
    }

    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    // Insertar ordenadamente la declaración del sistema
    let insert_index = lines
        .iter()
        .position(|line| line.starts_with("extern void define_systems"))
        .unwrap_or(lines.len());

    let mut includes: Vec<String> = lines
        .iter()
        .filter(|line| line.starts_with("void ") && line.ends_with("(ecs_iter_t *it);"))
        .cloned()
        .collect();

    includes.push(format!("void {}(ecs_iter_t *it);", name));
    includes.sort();

    // Eliminar las declaraciones antiguas y reemplazar con la lista ordenada
    lines.retain(|line| !line.starts_with("void ") || !line.ends_with("(ecs_iter_t *it);"));
    lines.splice(insert_index..insert_index, includes);

    // Escribir de nuevo el archivo
    fs::write(file_path, lines.join("\n"))?;
    Ok(())
}

fn create_system_file(name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = format!("src/systems/{}.c", name);

    // Verificar si el archivo ya existe para no sobrescribirlo
    if fs::metadata(&file_path).is_ok() {
        return Ok(());
    }

    let content = format!(
        r#"#include "systems.h"

void {}(ecs_iter_t *it) {{
  // const Singleton *s = ecs_singleton_get(it->world, Singleton);
  // Component *c = ecs_field(it, Component, 0);
}}
"#,
        name
    );

    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn update_register_systems_c(name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = "src/systems/register.c";
    let mut contents = String::new();

    // Leer el archivo
    {
        let mut file = fs::File::open(file_path)?;
        file.read_to_string(&mut contents)?;
    }

    // Verificar si ya existe el sistema
    if contents.contains(&format!("ECS_SYSTEM(world, {}, EcsOnUpdate);", name)) {
        return Ok(()); // No hacer nada si ya existe
    }

    // Modificar el contenido
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    // Insertar ECS_SYSTEM dentro de define_systems
    if let Some(start_index) = lines
        .iter()
        .position(|line| line.starts_with("void define_systems"))
    {
        let mut insert_pos = start_index + 1;
        while insert_pos < lines.len() && lines[insert_pos].trim().starts_with("ECS_SYSTEM") {
            insert_pos += 1;
        }
        lines.insert(
            insert_pos,
            format!("  ECS_SYSTEM(world, {}, EcsOnUpdate);", name),
        );
    }

    // Escribir de nuevo el archivo
    let mut file = fs::File::create(file_path)?;
    file.write_all(lines.join("\n").as_bytes())?;
    Ok(())
}

fn update_register_c(name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = "src/components/register.c";
    let mut contents = String::new();

    // Leer el archivo
    {
        let mut file = fs::File::open(file_path)?;
        file.read_to_string(&mut contents)?;
    }

    // Verificar si ya existe el componente
    if contents.contains(&format!("ECS_COMPONENT_DECLARE({});", name)) {
        return Ok(()); // No hacer nada si ya existe
    }

    // Modificar el contenido
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    // Insertar ECS_COMPONENT_DECLARE ordenadamente
    let mut insert_index = lines
        .iter()
        .position(|line| line.starts_with("void define_components"))
        .unwrap_or(lines.len());
    while insert_index > 0 && lines[insert_index - 1].starts_with("ECS_COMPONENT_DECLARE") {
        insert_index -= 1;
    }
    lines.insert(insert_index, format!("ECS_COMPONENT_DECLARE({});", name));

    // Insertar ECS_COMPONENT_DEFINE dentro de define_components
    if let Some(start_index) = lines
        .iter()
        .position(|line| line.starts_with("void define_components"))
    {
        let mut insert_pos = start_index + 1;
        while insert_pos < lines.len()
            && lines[insert_pos].trim().starts_with("ECS_COMPONENT_DEFINE")
        {
            insert_pos += 1;
        }
        lines.insert(
            insert_pos,
            format!("  ECS_COMPONENT_DEFINE(world, {});", name),
        );
    }

    // Escribir de nuevo el archivo
    let mut file = fs::File::create(file_path)?;
    file.write_all(lines.join("\n").as_bytes())?;
    Ok(())
}

fn create_component_header(name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = format!("src/components/{}.h", name);
    let mut file = File::create(&file_path)?;

    let uppercase_name = name.to_uppercase();
    let content = format!(
        "#ifndef _{}_H\n#define _{}_H\n#include <flecs.h>\n\ntypedef struct {} {{\n  int x, y;\n}} {};\n\nextern ECS_COMPONENT_DECLARE({});\n#endif\n",
        uppercase_name, uppercase_name, name, name, name
    );

    file.write_all(content.as_bytes())?;
    Ok(())
}

fn create_component(name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = "src/components/components.h";
    let contents = fs::read_to_string(file_path)?;

    let mut includes = Vec::new();
    let mut other_lines = Vec::new();
    let mut inside_include_block = false;

    // Procesamos el archivo línea por línea
    for line in contents.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("#include") {
            includes.push(trimmed.to_string());
            inside_include_block = true;
        } else {
            if inside_include_block && !trimmed.starts_with("#include") {
                inside_include_block = false;
            }
            other_lines.push(line.to_string());
        }
    }

    // Agregar el nuevo include si no está ya en la lista
    let new_include = format!("#include \"{}.h\"", name);
    if !includes.contains(&new_include) {
        includes.push(new_include);
    }

    // Ordenamos alfabéticamente los includes
    includes.sort();

    // Reconstruimos el contenido con el formato deseado
    let mut new_content = String::new();
    let mut inserted_includes = false;
    let mut added_blank_line = false;

    for (i, line) in other_lines.iter().enumerate() {
        // Agregar una línea en blanco antes de los includes
        if !inserted_includes && line.starts_with("extern void define_components") {
            if !new_content.ends_with("\n\n") {
                new_content.push('\n');
            }
            for inc in &includes {
                new_content.push_str(inc);
                new_content.push('\n');
            }
            inserted_includes = true;

            // Asegurar una línea en blanco después de los includes
            if !new_content.ends_with("\n\n") {
                new_content.push('\n');
            }
        }

        // Evitar múltiples líneas en blanco seguidas
        if line.trim().is_empty() {
            if !added_blank_line {
                new_content.push('\n');
            }
            added_blank_line = true;
        } else {
            new_content.push_str(line);
            new_content.push('\n');
            added_blank_line = false;
        }
    }

    // Escribimos el contenido formateado en el archivo
    let mut file = fs::File::create(file_path)?;
    file.write_all(new_content.trim_end().as_bytes())?;

    Ok(())
}
