# This script should be run in root dir.

jq -f home/0_all-target-triples.jq tmp/os-checker.json
jq -f home/1_array-of-target-triples.jq tmp/os-checker.json
