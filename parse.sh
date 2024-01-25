#!/bin/zsh
for i in {1..30}; do
    file="$HOME/juxt/xtdb-rs/q-tpch/q$i.edn"
    if [[ -f $file ]]; then
        echo "XXX-> $file"
        cat $file | ./xtql_to_json |jq > out/q$i.json
    fi
done
