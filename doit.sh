#!/bin/zsh
for i in {1..30}; do
    file="$HOME/juxt/xtdb-rs/q-tpch/q$i.edn"
    if [[ -f $file ]]; then
        cat $file | jet | ./xtdb_query > out/q$i.out
        echo $file
    fi
done
