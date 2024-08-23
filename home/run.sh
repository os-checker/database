# This script should be run in root dir.

set +e

jq -f home/0_all-target-triples.jq tmp/os-checker.json >ui/home/all-target-triples.json
jq -f home/1_array-of-target-triples.jq tmp/os-checker.json >ui/home/array-of-target-triples.json
