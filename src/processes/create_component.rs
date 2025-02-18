use std::error::Error;
use std::fs;
use std::io::Write;

pub fn create_component(name: &str) -> Result<(), Box<dyn Error>> {
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
