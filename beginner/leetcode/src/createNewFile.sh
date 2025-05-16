# #!/bin/zsh
# Check if two arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <folder_name> <file_name>"
    exit 1
fi

FOLDER_NAME=$1
FILE_NAME=$2

# Ensure the folder exists
if [ ! -d "$FOLDER_NAME" ]; then
    echo "Error: Folder '$FOLDER_NAME' does not exist."
    exit 1
fi

# Create the Rust file inside the folder
FILE_PATH="$FOLDER_NAME/$FILE_NAME.rs"

if [ -f "$FILE_PATH" ]; then
    echo "File '$FILE_NAME.rs' already exists in '$FOLDER_NAME'."
else
    touch "$FILE_PATH"
    echo "Created file: $FILE_PATH"
fi

# Append to mod.rs if the module is not already included
MOD_FILE="$FOLDER_NAME/mod.rs"
MOD_ENTRY="pub mod $FILE_NAME;"

if ! grep -q "$MOD_ENTRY" "$MOD_FILE"; then
    echo "$MOD_ENTRY" >> "$MOD_FILE"
    echo "Updated mod.rs with: $MOD_ENTRY"
else
    echo "mod.rs already contains '$MOD_ENTRY'"
fi
