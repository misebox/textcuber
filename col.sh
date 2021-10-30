#!/bin/bash

reset() {
    echo -en "\033[0m"
}
plain() {
    echo -en "\033[40m${1}"
    reset
}
white() {
    echo -en "\033[47m${1}"
    reset
}
red() {
    echo -en "\033[41m${1}"
    reset
}
green() {
    echo -en "\033[42m${1}"
    reset
}

echo
plain "   .___.___.___."
echo
plain "  /"
red " R "
plain "|"
red " R "
plain "|"
red " R "
plain "/"
green "G"
plain "|"
echo
plain " /"
red " R "
plain "|"
red " R "
plain "|"
red " R "
plain "/"
green "G"
plain "/|"
echo
plain "|"
red " R "
plain "|"
red " R "
plain "|"
red " R "
plain "|"
green "G"
plain "/"
green "G"
plain "|"
echo
echo "|---|---|---|"
plain "|"
green " G "
plain "|"
green " G "
plain "|"
green " G "
plain "|"
plain "/"
echo
echo "|---|---|---|"
green "| G | G | G |"
echo
echo "|---|---|---|"

reset


