#!/bin/bash

# Copyright (C) 2021 Mattéo Rossillol‑‑Laruelle <beatussum@protonmail.com>
#
# This program is free software: you can redistribute it and/or modify it under
# the terms of the GNU General Public License as published by the Free Software
# Foundation, either version 3 of the License, or (at your option) any later
# version.
#
# This program is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
# FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
# details.
#
# You should have received a copy of the GNU General Public License along with
# this program. If not, see <https://www.gnu.org/licenses/>.


readonly EXE_DIR="$(dirname $0)"
readonly EXE_NAME="$(basename $0)"

declare -Ar COLOR=(
    [bold]="\e[1m"
    
    [bgreen]="\e[1;32m"
    [bred]="\e[1;31m"
    [byellow]="\e[1;33m"
    
    [off]="\e[0m"
)

set -e

usage_new_menu() 
{
    local -r menu="$1"
    
    printf "${COLOR[bgreen]}%s:${COLOR[off]}\n" \
        "${menu}"
}

usage_new_subcommand()
{
    local -r subcommand="$1"
    local -r desc="$2"
    
    printf "  ${COLOR[byellow]}%s${COLOR[off]} ${COLOR[bred]}:${COLOR[off]} %s\n" \
        "${subcommand}" "${desc}"
}

usage_new_option()
{
    local -r option="$1"
    local -r long_option="$2"
    local -r desc="$3"
    
    printf "  ${COLOR[bold]}%s${COLOR[off]} ${COLOR[bred]}:${COLOR[off]} %s\n" \
        "${option}, ${long_option}" "${desc}"
}

usage()
{
    header()
    {
        echo -e "${COLOR[bgreen]}usage:${COLOR[off]} ${COLOR[byellow]}${EXE_NAME}${COLOR[off]} ${COLOR[bold]}[options]${COLOR[off]}"
        echo -e "       ${COLOR[byellow]}${EXE_NAME}${COLOR[off]} ${COLOR[bold]}[options] <subcommand> [subcommand options]${COLOR[bold]}"
    }
    
    header
    echo
    usage_new_menu "Subcommands"
    usage_new_subcommand "init" "Create a new problemset"
    echo
    usage_new_menu "Options"
    usage_new_option "-h" "--help" "Print this message"
}

usage_subcommand_header()
{
    local -r header="$1"

    echo -e "${COLOR[bgreen]}usage:${COLOR[off]} ${COLOR[byellow]}${EXE_NAME}${COLOR[off]} ${COLOR[bold]}[options] ${COLOR[byellow]}${header}${COLOR[off]} ${COLOR[bold]}[subcommand options]${COLOR[off]}"
    echo
}

usage_init()
{
    usage_subcommand_header "init"
    usage_new_menu "Subcommand options"
    usage_new_option "-h" "--help" "Print this message"
}

init()
{
    local -r url="$1"
    
    local -r problem="$(sed -En 's|.*/([0-9])/([A-Z])|\1\2|p' <<< "${url}" )"
    
    install -dm755 "${problem}/"
    install -m644 "${EXE_DIR}/templates"/{problem.cpp,CMakeLists.txt} \
        "${problem}/"
        
    sed -i \
        -e "s/@PROBLEM@/${problem}/g" \
        -e "s|@URL@|${url}|g" \
        "${problem}"/*
}

while :; do
    case "$1" in
        ""|-h|--help)
            usage
        ;;
        init)
            shift
            case "$1" in
                ""|-h|--help)
                    usage_init
                ;;
                -*)
                    usage_init
                    exit 1
                ;;
                *)
                    init "$1"
                ;;
            esac
        ;;
        *)
            usage
            exit 1
        ;;
    esac
    
    shift
    
    [[ $# -eq 0 ]] && break
done
