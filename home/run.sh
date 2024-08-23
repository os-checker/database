# This script should be run in root dir.

set +e

echo "jq -f home/0_all-target-triples.jq $UI_JSON >ui/home/all-target-triples.json"
jq -f home/0_all-target-triples.jq "$UI_JSON" >ui/home/all-target-triples.json

echo "jq -f home/1_array-of-target-triples.jq $UI_JSON >ui/home/array-of-target-triples.json"
jq -f home/1_array-of-target-triples.jq "$UI_JSON" >ui/home/array-of-target-triples.json
