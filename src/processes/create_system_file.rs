use std::error::Error;
use std::fs;
use std::io::Write;

pub fn create_system_file(name: &str) -> Result<(), Box<dyn Error>> {
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
