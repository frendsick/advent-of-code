#!/bin/sh

# Variables
template_day="day00"
day="day$1"

# Check command line arguments
if [ $# -ne 1 ]
then
    echo "Usage: $0 <day_number>";
    exit 1
fi

# Do not overwrite existing files
if [ -e $day ]
then
    echo "Folder $day already exists"
    exit 1
fi

# Create directory for the day
cp -r template-gleam $day

# Change all occurences of the template day marker to the current day
find $day -type f -not -path "*/\.*" -exec sed -i "s/$template_day/$day/g" {} +

# Rename template files
mv $day/src/$template_day.gleam $day/src/$day.gleam
mv $day/test/$template_day\_test.gleam $day/test/$day\_test.gleam
