use std::error::Error;

use std::fs::File;
use std::io::Write;

pub fn create_component_header(name: &str) -> Result<(), Box<dyn Error>> {
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
