#!/usr/bin/env bash

for file in ./test_data/*
do
  if [ -f "$file" ]
  then
    xmllint -o "$file" --format "$file"
  fi
done
