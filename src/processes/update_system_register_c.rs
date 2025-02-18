use std::error::Error;
use std::fs;
use std::io::{Read, Write};

pub fn update_register_systems_c(name: &str) -> Result<(), Box<dyn Error>> {
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
