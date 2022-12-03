#!/usr/bin/env bash

# Enable error options
set -Eeuo pipefail

# Enable debug
#set -x

gen_asm () {
    cargo asm --rust --lib "exper_borrowed_vs_owned_parameters::$1" > asm/$1.txt
}

gen_asm "message_borrowed"
gen_asm "message_owned"
gen_asm "messagemf_borrowed"
gen_asm "messagemf_owned"
gen_asm "invoke_message_default"
gen_asm "invoke_messagemf_default"
gen_asm "invoke_message_borrowed"
gen_asm "invoke_message_owned"
gen_asm "invoke_messagemf_borrowed"
gen_asm "invoke_messagemf_owned"
gen_asm "invoke_messagemf_borrowed2"
