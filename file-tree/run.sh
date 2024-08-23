# This script should be run in root dir.

set +e

echo "jq -f file-tree/0_all-target-triples.jq $UI_JSON >ui/file-tree/all-target-triples.json"
jq -f file-tree/0_all-target-triples.jq "$UI_JSON" >ui/file-tree/all-target-triples.json

echo "jq -f file-tree/1_array-of-target-triples.jq $UI_JSON >ui/file-tree/array-of-target-triples.json"
jq -f file-tree/1_array-of-target-triples.jq "$UI_JSON" >ui/file-tree/array-of-target-triples.json
