#!/bin/bash

TOTAL=$(find . -type f -iwholename "./examples/*.rs" | wc -l)
count=1

echo "Start of examples:"

for file in examples/*.rs
do 
    echo
    echo "Running example \`$(basename "$file" .rs)\`":
    echo
    cargo run --example $(basename "$file" .rs);
    if [ "$1" != "--all" ] && [ "$count" != "$TOTAL" ]
    then
        echo
        read -p "Press Enter to run the next example..."
    fi
    count=$((count + 1))
done
echo
echo "End of examples."
