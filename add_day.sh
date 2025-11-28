#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: addDay <day_number>"
    echo "e.g. addDay 05"
    exit 1
fi

folder_name="$1"

# Validate input format and extract day number
if [[ "$folder_name" =~ day([0-9]+) ]]; then
    # It's a path-like string containing "day" followed by digits
    day_num="${BASH_REMATCH[1]}"
    day_number=$(printf 'day%02d' "$((10#$day_num))")
elif [[ "$folder_name" =~ ^[0-9]+$ ]]; then
    # It's just a number (1, 01, 10, etc.)
    day_number=$(printf 'day%02d' "$((10#$folder_name))")
    folder_name="./2025/$day_number"
else
    echo "Error: Invalid input '$folder_name'"
    echo "Must be either:"
    echo "  - A day number: 1, 01, 10, 25 (etc.)"
    echo "  - A path with 'day': ./practice/2023/day01"
    exit 1
fi

echo "Folder: $folder_name"
echo "Day: $day_number"

# Check if the 'template' folder exists.
if [ ! -d "template" ]; then
    echo "Error: 'template' folder does not exist."
    exit 1
fi

# Create a new folder with the given name.
mkdir "$folder_name"

# Copy the contents of the 'template' folder to the new folder.
cp -r template/* "$folder_name"

# Replace '{{day_number}}' with the provided value in all files inside the new folder.
LC_ALL=C find "$folder_name" -type f -exec sed -i '' "s/{{day_number}}/$day_number/g" {} \;
