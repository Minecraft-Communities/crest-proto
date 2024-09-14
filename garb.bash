#! /usr/bin/env bash

declare -ga var_templates_list=(
    NIX_CFLAGS_COMPILE
    NIX_CFLAGS_COMPILE_BEFORE
    NIX_CFLAGS_LINK
    NIX_CXXSTDLIB_COMPILE
    NIX_CXXSTDLIB_LINK
    NIX_GNATFLAGS_COMPILE
)

function main {
    for item in "${var_templates_list[@]}"; do
        echo =====================================
        eval "echo ${item}=\${${item}}"
    done
}

main
