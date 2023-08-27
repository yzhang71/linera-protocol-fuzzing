#!/bin/bash

input_testcase="$1"
folder_count=30
i=0
mkdir -p split_testcases

for file in "${input_testcase}"/*; do
  if [ -f "$file" ]; then
    folder=$((i % folder_count + 1))
    mkdir -p "split_testcases/folder${folder}"
    cp "$file" "split_testcases/folder${folder}/"
    i=$((i + 1))
  fi
done

