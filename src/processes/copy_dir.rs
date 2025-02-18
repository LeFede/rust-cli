use std::fs;
use std::io;
use std::path::Path;

pub fn copy_dir(src: &str, dst: &str) -> io::Result<()> {
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    // Crea la carpeta de destino si no existe
    if !dst_path.exists() {
        fs::create_dir_all(dst_path)?;
    }

    // Itera sobre los archivos en el directorio fuente
    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dst_entry_path = dst_path.join(entry.file_name());

        if entry_path.is_dir() {
            // Si es un directorio, llama recursivamente
            copy_dir(
                entry_path.to_str().unwrap(),
                dst_entry_path.to_str().unwrap(),
            )?;
        } else {
            // Si es un archivo, lo copia
            fs::copy(entry_path, dst_entry_path)?;
        }
    }
    Ok(())
}
