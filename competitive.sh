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


readonly EXE_DIR="$(dirname $(realpath $0))"
readonly EXE_NAME="$(basename $0)"

declare -Ar COLOR=(
	[bold]="\e[1m"

	[bgreen]="\e[1;32m"
	[bred]="\e[1;31m"
	[byellow]="\e[1;33m"

	[off]="\e[0m"
)

set -e

info()
{
	local -r msg="$*"

	printf "${COLOR[bgreen]}*${COLOR[off]} %s\n" "${msg}"
}

info_in_square()
{
	local -r msg="$*"

	echo
	printf "=%.0s" $(seq 0 $((${#msg} + 2))); echo
	info "${msg}"
	printf "=%.0s" $(seq 0 $((${#msg} + 2))); echo
	echo
}

error()
{
	local -r msg="$*"

	printf "${COLOR[bred]}*${COLOR[off]} %s\n" "${msg}"
}

die()
{
	local -r msg="$*"

	error "${msg}"; echo
	usage
	exit 2
}

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
		echo -e "  ${COLOR[byellow]}${EXE_NAME}${COLOR[off]} ${COLOR[bold]}[options] <subcommand> [subcommand options]${COLOR[bold]}"
	}

	header
	echo
	usage_new_menu "Subcommands"
	usage_new_subcommand "evalenv" "Prints the instructions needed to load a suitable environment"
	usage_new_subcommand "init" "Creates a new problem"
	usage_new_subcommand "run" "Runs the current problem"
	echo
	usage_new_menu "Options"
	usage_new_option "-h" "--help" "Prints this message"
	usage_new_option "-p" "--problemset" "Changes the problem name"
}

usage_subcommand_header()
{
	local -r header="$1"; shift
	local -r args="$@"

	echo -en "${COLOR[bgreen]}usage:${COLOR[off]} ${COLOR[byellow]}${EXE_NAME}${COLOR[off]} ${COLOR[bold]}[options] ${COLOR[byellow]}${header}${COLOR[off]} ${COLOR[bold]}[subcommand options]${COLOR[off]}"

	for i in "${args[@]}"; do
		if grep -q "^o" <<< "${i}"; then
			printf " ${COLOR[bold]}[%s]${COLOR[off]}" "${i#o}"
		elif [[ -n "${i}" ]]; then
			printf " ${COLOR[bold]}<%s>${COLOR[off]}" "${i}"
		fi
	done

	echo; echo
}

evalenv()
{
	local -Ar env=(
		[PATH]="${EXE_DIR}"
	)

	for key in "${!env[@]}"; do
		echo "export ${key}=\"\${${key}}:${env[${key}]}\""
	done
}

init()
{
	usage()
	{
		usage_subcommand_header "init" "problem name"
		usage_new_menu "Subcommand options"
		usage_new_option "-h" "--help" "Prints this message"
		usage_new_option "-p" "--platform" "Sets the platform"
		usage_new_option "-u" "--url" "Sets the problem url"
	}

	local platform problem_name problem_url

	if [[ $# -eq 0 ]]; then
		usage
		exit 0
	fi

	while [[ $# -gt 0 ]]; do
		case "$1" in
			-h|--help)
				usage
				exit 0
			;;
			-p|--platform)
				platform="$2"
			;;
			-u|--url)
				problem_url="$2"
			;;
			-*)
				die "'$1' is not a valid option"
			;;
			*)
				problem_name="$1"
			;;
		esac

		shift
	done

	info "Initializing ${problem_name}…"

	local -r dir="${EXE_DIR}/${problem_name}"

	case "${platform}" in
		""|codeforces)
			local suffix="$(sed -En "s|([0-9]+)([A-Z])|\1/\2|p" <<< "${problem_name}")"
			problem_url="https://codeforces.com/problemset/problem/${suffix}/"

			unset suffix
		;;
		custom)
		;;
		*)
			die "'$1' is an unknown platform"
		;;
	esac

	[[ -z "${problem_name}" ]] && die "The problem name cannot be empty"

	install -dm755 "${problem_name}/"
	install -m644 "${EXE_DIR}/templates"/{problem.cpp,CMakeLists.txt} "${dir}"

	sed -i \
		-e "s/@PROBLEM@/${problem_name}/g" \
		-e "s|@URL@|${problem_url}|g" \
		"${dir}"/*

	exit 0
}

run()
{
	usage()
	{
		usage_subcommand_header "run"
		usage_new_menu "Subcommand options"
		usage_new_option "-h" "--help" "Prints this message"
		usage_new_option "-p" "--problemset" "Overrides the default problem name"
	}

	local problem_name="$(basename "${PWD}")"

	if [[ $# -eq 0 ]]; then
		usage
		exit 0
	fi

	while [[ $# -gt 0 ]]; do
		case "$1" in
			-h|--help)
				usage
				exit 0
			;;
			-p|--problemset)
				problem_name="$2"
			;;
			*)
				die "'$1' is not a valid option"
			;;
		esac

		shift
	done

	[[ -z "${problem_name}" ]] && die "The problem name cannot be empty"

	local -r build_dir="${EXE_DIR}/${problem_name}/build"

	cmake -B "${build_dir}" -G Ninja
	ninja -C "${build_dir}" -j"$((nproc + 1))"

	info_in_square "Starting ${problem_name}…"

	exec "${EXE_DIR}/${problem_name}/build/problem"
}

if [[ $# -eq 0 ]]; then
	usage
	exit 0
fi

while [[ $# -gt 0 ]]; do
	case "$1" in
		-h|--help)
			usage
			exit 0
		;;
		evalenv)
			evalenv
			exit 0
		;;
		init)
			shift
			init "$@"
		;;
		run)
			shift
			run "$@"
		;;
		-*)
			die "'$1' is not a valid option"
		;;
		*)
			die "'$1' is not a valid subcommand"
		;;
	esac

	shift
done
