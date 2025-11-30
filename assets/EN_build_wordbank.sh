#!/usr/bin/env bash

file='EN_all.txt'
charcount=5

function save_en_file() {    
    # keep only a-z and A-Z, at specified length, lowercase, sort
    grep -E '^[a-zA-Z]{'$1'}$' "$file" | \
        tr 'A-Z' 'a-z' | \
        sort
}

save_en_file "$charcount" > "EN_${charcount}-char_wordlist.txt"
