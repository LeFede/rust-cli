

#!/bin/bash

set -e  # Termina el script si hay un error

BIN_NAME="cli"
SOURCE_PATH="target/release/$BIN_NAME"
DEST_PATH="/usr/local/bin/$BIN_NAME"
TEMPLATE_PATH="template"

export PATH="$HOME/.cargo/bin:$PATH"

cargo build --release

# Verifica si el binario existe
if [[ ! -f "$SOURCE_PATH" ]]; then
    echo "Error: No se encontró el binario en '$SOURCE_PATH'. Compila primero con 'cargo build --release'."
    exit 1
fi

# Mueve el binario con sudo
echo "Moviendo '$SOURCE_PATH' a '$DEST_PATH'..."
sudo cp "$SOURCE_PATH" "$DEST_PATH"

sudo rm -rf "/usr/local/share/$BIN_NAME"
# Copiar los archivos de template
echo "Copiando template a /usr/local/share/$BIN_NAME..."
sudo cp -r "$TEMPLATE_PATH" "/usr/local/share/$BIN_NAME"

# Asegura permisos correctos
sudo chmod +x "$DEST_PATH"

echo "✅ Instalación completada. Puedes ejecutar '$BIN_NAME' desde cualquier lugar."
# clear
