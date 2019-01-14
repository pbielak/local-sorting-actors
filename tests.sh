#!/bin/bash
for n in $1; do
  echo "N = $n"
  for r in $(seq 1 $2); do 
    ./target/release/local-sorting-actors -i $3 -o /tmp/output.txt -n $n -k $n;
  done
done

