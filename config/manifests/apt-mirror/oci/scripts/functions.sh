#!/usr/bin/env bash

##################################################
# Name: functions.sh
# Description: useful shell functions
##################################################

#########################
# Common
#########################

checkBin() {

	# Checks the binary name is available in the path

	local COMMAND="$1"

	#if ( command -v "${COMMAND}" 1> /dev/null ) ; # command breaks with aliases
	if (type -P "${COMMAND}" &>/dev/null); then
		writeLog "DEBUG" "The command $COMMAND is available in the Path"
		return 0
	else
		writeLog "DEBUG" "The command $COMMAND is not available in the Path"
		return 1
	fi

}

checkReqs() {

	# Make sure all the required binaries are available within the path
	for BIN in "${REQ_BINS[@]}"; do
		writeLog "DEBUG" "Checking for dependant binary ${BIN}"
		checkBin "${BIN}" || {
			writeLog "ERROR" "Please install the ${BIN} binary on this system in order to run ${SCRIPT}"
			return 1
		}
	done

	return 0

}

checkPermissions() {

	# Checks if the user is running as root

	if [ "${EUID}" -ne 0 ]; then
		return 1
	else
		return 0
	fi

}

checkLogLevel() {

	# Only the following log levels are supported.
	#   DEBUG
	#   INFO or INFORMATION
	#   WARN or WARNING
	#   ERR or ERROR

	local LEVEL="${1}"
	export LOGLEVEL

	# POSIX be gone!
	# LOGLEVEL="$( echo "${1}" | tr '[:lower:]' '[:upper:]' )"
	case "${LEVEL^^}" in

	"DEBUG" | "TRACE")

		export LOGLEVEL="DEBUG"

		;;

	"INFO" | "INFORMATION")

		export LOGLEVEL="INFO"

		;;

	"WARN" | "WARNING")

		export LOGLEVEL="WARN"

		;;

	"ERR" | "ERROR")

		export LOGLEVEL="ERR"

		;;

	*)

		writeLog "INFO" "An unknown log level of ${LEVEL^^} was provided, defaulting to INFO"
		export LOGLEVEL="INFO"

		;;

	esac

	return 0

}

writeLog() {

	local LEVEL="${1}"
	local MESSAGE="${2}"

	case "${LEVEL^^}" in

	"DEBUG" | "TRACE")

		LEVEL="DEBUG"

		# Do not show debug messages if the level is > debug
		if [ ! "${LEVEL^^}" = "${LOGLEVEL^^}" ]; then
			return 0
		fi

		;;

	"INFO" | "INFORMATION")

		LEVEL="INFO"

		# Do not show info messages if the level is > info
		if [ "${LOGLEVEL^^}" = "WARN" ] || [ "${LOGLEVEL^^}" = "ERR" ]; then
			return 0
		fi

		;;

	"WARN" | "WARNING")

		LEVEL="WARN"

		# Do not show warn messages if the level is > warn
		if [ "${LOGLEVEL^^}" = "ERR" ]; then
			return 0
		fi

		;;

	"ERR" | "ERROR")

		LEVEL="ERR"

		# Errors are always shown

		;;

	*)

		MESSAGE="Unknown log level ${LEVEL^^} provided to log function. Valid options are DEBUG, INFO, WARN, ERR"
		LEVEL="ERR"

		;;

	esac

	echo -e "$(date +"%Y/%m/%d %H:%M:%S") [${LEVEL^^}] ${MESSAGE}"

	return 0

}

function checkVarEmpty() {

	# Returns true if the variable is empty

	# NOTE:
	#	Pass this function the string NAME of the variable
	#	Not the expanded contents of the variable itself.

	local VAR_NAME="${1}"
	local VAR_DESC="${2}"

	if [[ ${!VAR_NAME:-EMPTY} == "EMPTY" ]]; then
		writeLog "ERROR" "The variable ${VAR_DESC} is empty."
		return 0
	else
		writeLog "DEBUG" "The variable ${VAR_DESC} is not empty, it is set to ${!VAR_NAME}"
		return 1
	fi

}

checkResult() {

	local RESULT="${1}"

	if [ "${RESULT}" -ne 0 ]; then
		return 1
	else
		return 0
	fi

}

#########################
# Export
#########################

# Export common functions
export -f checkBin checkReqs checkPermissions checkLogLevel writeLog checkVarEmpty checkResult

#########################
# End
#########################

writeLog "DEBUG" "Sourced and exported functions.sh"
