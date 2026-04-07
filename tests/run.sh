#!/bin/bash

set -e

for file in $(find tests -type f -name '*.hurl' | sort -V); do
    hurl --test --variables-file tests/variables.env --error-format long "$file"
done
