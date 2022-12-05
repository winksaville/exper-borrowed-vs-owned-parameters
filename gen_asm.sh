#!/usr/bin/env bash

# Enable error options
set -Eeuo pipefail

# Enable debug
#set -x

gen_asm () {
    cargo asm --rust --lib "exper_borrowed_vs_owned_parameters::$1" > asm/$1.txt
}

#gen_asm "invoke_msgnf_default" # optimized away
gen_asm "invoke_msgof_default"
gen_asm "invoke_msgsf_default"
gen_asm "invoke_msgmf_default"
#gen_asm "invoke_boxed_msgnf_default" # optimized away
gen_asm "invoke_boxed_msgof_default"
gen_asm "invoke_boxed_msgsf_default"
gen_asm "invoke_boxed_msgmf_default"
#gen_asm "invoke_msgnf_borrowed" # optimized away
gen_asm "invoke_msgnf_owned"     # This is just a `ret`
gen_asm "invoke_msgof_borrowed"
gen_asm "invoke_msgof_owned"
gen_asm "invoke_msgsf_borrowed"
gen_asm "invoke_msgsf_owned"
gen_asm "invoke_msgmf_borrowed"
gen_asm "invoke_msgmf_owned"
gen_asm "invoke_boxed_msgnf"     # This is just a `ret`
gen_asm "invoke_boxed_msgof"
gen_asm "invoke_boxed_msgsf"
gen_asm "invoke_boxed_msgmf"

