use std::error::Error;
use std::fs;
use std::io::{Read, Write};

pub fn update_register_c(name: &str) -> Result<(), Box<dyn Error>> {
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
