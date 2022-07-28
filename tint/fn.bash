#!/usr/bin/env bash
python -V || { echo "python required."; exit 1; }

color_dark () { # numerator (arg1) and denominator (arg2) of a number 0<x<1
    # for tuned (dimmed) rgb color codes, of reversed 400-800nm color spectrum...
    # aka return rainbow colors codes, for values between 0 (red) and 1 (violet)
    local a n="$1" d="$2"
    shift 2
    # of seq 400 $(dc -e "7k 1 $n / 400 * p") 800
    set $(./nm2rgb.py $(dc -e "7k 1 $d / $n * 2 - _400 * p") )
    # set the background, with tinted modifications
    printf '\033[48;2;%d;%d;%dm' $(( $1 *1000 *47/100 /1000 )) $(( $2 *1000 *33/100 /1000 )) $(( $3 *1000 *59/100 /1000 ))
    }

export color_reset="\033[0m"

for d in 3 29 ; do
    export d
    time seq 1 $d | while read n ; do
        printf "%02x $(color_dark $n $d)%24s${color_reset} %s\n" "$n" "${RANDOM}${RANDOM}${RANDOM}${RANDOM}" "${RANDOM}"
        done # n
    echo
    done # e
