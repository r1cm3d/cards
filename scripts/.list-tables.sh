#!/bin/bash

function get-tn() {
  local tn
  tn=$(< "$1" grep -Po '(?<="TableName":\s")\w+')
  echo "$tn"
}

for tf in json/*_table.json; do
  tn="$tn$(get-tn "$tf") "
done

echo "$tn"