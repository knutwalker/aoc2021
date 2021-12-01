#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

curl --cookie "session=$(cat .sessioncookie)" "https://adventofcode.com/2021/day/$1/input" > "src/input/day$1.txt";
