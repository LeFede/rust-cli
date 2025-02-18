use std::error::Error;
use std::fs;

pub fn update_systems_h(name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = "src/systems/systems.h";
    let contents = fs::read_to_string(file_path)?;

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
