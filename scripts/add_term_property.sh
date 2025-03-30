#!/bin/bash

# Check if a file path is provided
if [ $# -lt 1 ]; then
  echo "Usage: $0 <file_path>"
  exit 1
fi

FILE_PATH="$1"
BACKUP_FILE="${FILE_PATH}.bak"

# Check if file exists
if [ ! -f "$FILE_PATH" ]; then
  echo "Error: File $FILE_PATH does not exist."
  exit 1
fi

# Create a backup of the original file
cp "$FILE_PATH" "$BACKUP_FILE"
echo "Created backup at $BACKUP_FILE"

# Find node definitions and add term property if missing
# This uses a sed pattern that:
# 1. Finds lines that start a node definition (:NodeName
# 2. Captures the node name (between : and : or between : and {)
# 3. Then looks for the next line with "domain:" but no "term:"
# 4. Adds the term property line after the domain line

# First, extract node names and prepare for insertion
TMP_FILE=$(mktemp)
grep -n "^[[:space:]]*(:.*{" "$FILE_PATH" | while IFS=: read -r line_num line; do
  # Extract node name (between the first : and the next : or {)
  node_name=$(echo "$line" | sed -E 's/^\s*\(:([^:{}]+).*$/\1/')
  
  # Check if line already has a term property within the next 7 lines
  has_term=$(sed -n "$((line_num)),$((line_num+7))p" "$FILE_PATH" | grep -c "term:")
  
  if [ "$has_term" -eq 0 ]; then
    # If no term property, find the domain line to insert after
    domain_line_num=$(grep -n "domain:" "$FILE_PATH" | awk -F: -v start="$line_num" '$1 >= start {print $1; exit}')
    
    if [ -n "$domain_line_num" ]; then
      echo "$domain_line_num:  term: \"$node_name\"," >> "$TMP_FILE"
    fi
  fi
done

# Sort the insertion lines by line number (descending to avoid offset issues)
sort -t: -k1,1nr "$TMP_FILE" > "${TMP_FILE}.sorted"

# Apply the insertions from bottom to top to avoid line number shifts
while IFS=: read -r line_num insert_text; do
  sed -i "${line_num}a\\${insert_text}" "$FILE_PATH"
done < "${TMP_FILE}.sorted"

# Clean up temporary files
rm "$TMP_FILE" "${TMP_FILE}.sorted"

echo "Added missing term properties to all nodes"
echo "Verify the changes with: diff $BACKUP_FILE $FILE_PATH"

# Run the standardization checker on the file
if [ -f "./scripts/standardize_vocabulary.sh" ]; then
  echo "Running standardization check..."
  ./scripts/standardize_vocabulary.sh "$FILE_PATH"
else
  echo "Standardization checker script not found."
fi

exit 0 