#!/bin/sh

YEAR=$1
DAY=$2

curl -H "Cookie: session=$(cat .aocsession)" "https://adventofcode.com/${YEAR}/day/${DAY}/input" > "inputs/y${YEAR}p${DAY}.txt"
