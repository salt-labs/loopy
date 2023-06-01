#!/usr/bin/env bash

# https://www.wcooke.org/2021/02/debian-mirror-kubernetes/
# http://au.archive.ubuntu.com/ubuntu/pool/main/l/linux/linux-headers-5.4.0-97-generic_5.4.0-97.110_amd64.deb

set -euo pipefail

export LOGLEVEL="${LOGLEVEL:=DEBUG}"
export EXECUTABLE="debmirror"
export KEYSERVER="${KEYSERVER:=keyserver.ubuntu.com}"

# shellcheck disable=SC1091
source /scripts/functions.sh || {
	echo "Failed to source functions.sh"
	exit 1
}

if [[ ${1#-} != "${1}" ]]; then

	set -- "${EXECUTABLE}" "$@"

fi

if [[ ${1} == "${EXECUTABLE}" ]]; then

	shift

	writeLog "INFO" "Updating certificates"
	update-ca-certificates || {
		writeLog "WARN" "Failed to update certificates"
	}

	if [[ ${KEYS:-EMPTY} == "EMPTY" ]]; then

		writeLog "WARN" "No GPG keys specified, skipping."

	else

		writeLog "INFO" "Adding GPG keys: ${KEYS}"

		for KEY in ${KEYS}; do

			writeLog "INFO" "Adding key: ${KEY}"

			gpg --keyserver "${KEYSERVER}" --no-default-keyring --keyring trustedkeys.gpg --recv-keys "${KEY}" || {
				writeLog "WARN" "Failed to add key: ${KEY} using gpg, attempting failback..."

				curl -sSL "https://${KEYSERVER}/pks/lookup?op=get&search=0x${KEY}" | gpg --no-default-keyring --keyring trustedkeys.gpg --import || {
					writeLog "WARN" "Failed to add key: ${KEY} using curl HTTPS, trying HTTP..."

					# devskim: ignore DS137138
					curl -sSl "http://${KEYSERVER}/pks/lookup?op=get&search=0x${KEY}" | gpg --no-default-keyring --keyring trustedkeys.gpg --import || {
						writeLog "ERROR" "Failed to add key: ${KEY} using curl HTTP, giving up..."
					}

				}

			}

		done

		writeLog "INFO" "Finished adding GPG keys"

	fi

	# Check mandatory variables.
	checkVarEmpty "DEST" "Destination" && exit 100
	checkVarEmpty "HOST" "Host" && exit 101
	checkVarEmpty "ROOT" "Root" && exit 102
	checkVarEmpty "DIST" "Distribution" && exit 103
	checkVarEmpty "SECTION" "Section" && exit 104
	checkVarEmpty "ARCH" "Architecture" && exit 105
	checkVarEmpty "METHOD" "Method" && exit 106

	set -- /usr/bin/debmirror \
		"${DEST}" \
		--nosource \
		--host="${HOST}" \
		--root="${ROOT}" \
		--dist="${DIST}" \
		--section="${SECTION}" \
		--i18n \
		--arch="${ARCH}" \
		--passive \
		--cleanup \
		--method="${METHOD}" \
		--progress \
		"${OTHERARGS:-}" \
		--rsync-extra="${RSYNCEXTRA:=trace}" \
		"$@"

fi

writeLog "INFO" "Starting ${EXECUTABLE} with arguments: ${ARGS[*]}"

exec "$@"
