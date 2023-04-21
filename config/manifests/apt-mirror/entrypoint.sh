#!/usr/bin/env bash

# https://www.wcooke.org/2021/02/debian-mirror-kubernetes/

set -euo pipefail

EXECUTABLE="debmirror"

# TODO: Import common functions.

if [[ "${1#-}" != "${1}" ]]; then

    set -- ${EXECUTABLE} "$@"

fi

if [[ "${1}" "${EXECUTABLE}" ]]; then

    shift

    writeLog "INFO" "Importing GPG keys"

    for KEY in ${KEYS};


