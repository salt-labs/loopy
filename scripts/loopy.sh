#!/usr/bin/env bash

clear

set -euo pipefail

#--------------------------------------------------
# URLS
#--------------------------------------------------

# https://artifacthub.io/
# https://github.com/atlassian/data-center-helm-charts
# https://github.com/bitnami/charts
# https://cert-manager.io/docs/troubleshooting/webhook/
# https://cert-manager.io/docs/usage/ingress/
# https://gateway-api.sigs.k8s.io/

#--------------------------------------------------
# Variables (User)
#--------------------------------------------------

# Set all user variables in quickstart_values.sh

#--------------------------------------------------
# Variables (Internal)
#--------------------------------------------------

# These are variables that are not meant to be set by the user.

# For script debugging
ENABLE_DRY_RUN="false"

# Ingress defaults.
ENABLE_CONTOUR="false"
ENABLE_CONTOUR_GATEWAY="false"
ENABLE_AKO="false"

REQUIRED_TOOLS=(
	"kubectl"
	"helm"
)

kubectl_args=()

HELM_REPOSITORIES=(
	#"salt-labs https://helm.saltlabs.cloud"
	"bitnami https://charts.bitnami.com/bitnami"
	"jetstack https://charts.jetstack.io"
	"atlassian https://atlassian.github.io/data-center-helm-charts"
	"nfs-ganesha https://kubernetes-sigs.github.io/nfs-ganesha-server-and-external-provisioner/"
	"ako https://projects.registry.vmware.com/chartrepo/ako"
)

CHARTS_DEPENDENCIES=(
	"bitnami cert-manager"
	"bitnami contour"
	"bitnami postgresql-ha"
	"nfs-ganesha nfs-server-provisioner"
	"ako ako"
)

CHARTS_ATLASSIAN=(
	"atlassian bamboo"
	#"atlassian bamboo-agent"
	"atlassian bitbucket"
	"atlassian confluence"
	"atlassian crowd"
	"atlassian jira"
)

#--------------------------------------------------
# Functions
#--------------------------------------------------

function message() {

	echo "[$(date -u)] ${1}"

}

function check_tools() {

	for tool in "${REQUIRED_TOOLS[@]}"; do
		message "Checking for ${tool}..."
		if ! command -v "${tool}" &>/dev/null; then
			message "The tool ${tool} is not installed."
			return 1
		fi
	done

	return 0

}

function install_helm_repository() {

	local name="${1}"
	local url="${2}"

	if ! helm repo list 2>/dev/null | grep -q "${name}"; then

		message "Adding helm repository ${name}..."
		helm repo add "${name}" "${url}" || return 1

	else

		message "Helm repository ${name} already exists."

	fi

	return 0

}

function install_helm_repositories() {

	for repository in "${HELM_REPOSITORIES[@]}"; do

		local name
		local url

		name=$(echo "${repository}" | cut -d' ' -f1)
		url=$(echo "${repository}" | cut -d' ' -f2)

		message "Installing helm repository ${name}..."

		install_helm_repository "${name}" "${url}" || {
			message "Failed to install helm repository ${name}."
			return 1
		}

	done

	message "Updating helm repositories..."

	helm repo update || {
		message "Failed to update helm repositories."
		return 1
	}

	return 0

}

function prepare_helm_chart() {

	local chart_repo="${1}"
	local chart_name="${2}"
	local chart_dir="${3}"
	local chart_values="${4}"

	# If no existing chart directory exists, create one.
	if [[ ! -d ${chart_dir} ]]; then

		message "Creating chart directory for ${chart_name}..."

		if [[ ${ENABLE_DRY_RUN:-false} == "true" ]]; then

			message "DRY RUN: mkdir --parents ${chart_dir}"

		else

			mkdir --parents "${chart_dir}" || {
				message "Failed to create chart directory for ${chart_name}."
				return 1
			}

		fi

	else

		message "A chart directory already exists for ${chart_name}."

	fi

	# Overwrite the default values.yaml with the latest defaults.
	message "Updating default values.yaml for ${chart_name}..."

	if [[ ${ENABLE_DRY_RUN:-false} == "true" ]]; then

		message "DRY RUN: helm show values ${chart_repo}/${chart_name} > ${chart_values}"

	else

		helm show values "${chart_repo}/${chart_name}" >"${chart_values}" || {
			message "Failed to create default values.yaml for ${chart_name}."
			return 1
		}

	fi

	return 0

}

function install_helm_defaults() {

	local name
	local url
	local chart_dir
	local chart_values

	for chart_and_repo in "${CHARTS_DEPENDENCIES[@]}"; do

		chart_repo=$(echo "${chart_and_repo}" | cut -d' ' -f1)
		chart_name=$(echo "${chart_and_repo}" | cut -d' ' -f2)
		chart_dir="../${chart_name}"
		chart_values="${chart_dir}/values.yaml"

		prepare_helm_chart "${chart_repo}" "${chart_name}" "${chart_dir}" "${chart_values}" || {
			message "Failed to prepare helm chart ${chart_name}."
			return 1
		}

	done

	for chart_and_repo in "${CHARTS_ATLASSIAN[@]}"; do

		chart_repo=$(echo "${chart_and_repo}" | cut -d' ' -f1)
		chart_name=$(echo "${chart_and_repo}" | cut -d' ' -f2)
		chart_dir="./charts/${chart_name}"
		chart_values="${chart_dir}/values.yaml"

		prepare_helm_chart "${chart_repo}" "${chart_name}" "${chart_dir}" "${chart_values}" || {
			message "Failed to prepare helm chart ${chart_name}."
			return 1
		}

	done

	return 0

}

function install_helm_release() {

	local name="${1}"
	local chart="${2}"
	local namespace="${3}"
	local values="${4}"

	if ! helm list -A | grep -q "${name}"; then

		message "Installing helm release ${name}..."

		if [[ ${ENABLE_DRY_RUN:-false} == "true" ]]; then

			message "DRY RUN: show template only"

			helm template "${name}" \
				--values "${values}" \
				--namespace "${namespace}" \
				"${chart}" >"${TEMP_DIR}/${name}" || {
				message "Failed to template helm release ${name}."
				return 1
			}

		else

			helm install "${name}" \
				--values "${values}" \
				--namespace "${namespace}" \
				--create-namespace \
				"${chart}" || {
				message "Failed to install helm release ${name}."
				return 1
			}

		fi

	else

		message "Upgrading helm release ${name}..."

		if [[ ${ENABLE_DRY_RUN:-false} == "true" ]]; then

			message "DRY RUN: show template only"

			helm template "${name}" \
				--values "${values}" \
				--namespace "${namespace}" \
				"${chart}" || {
				message "Failed to template helm release ${name}."
				return 1
			}

		else

			helm upgrade "${name}" \
				--values "${values}" \
				--namespace "${namespace}" \
				"${chart}" || {
				message "Failed to upgrade helm release ${name}."
				return 1
			}

		fi

	fi

	return 0

}

function apply_rbac() {

	local manifests_dir="${1}"

	if [[ -d ${manifests_dir} ]]; then

		# There needs to be a namespace to apply permissions into.
		if [[ -f "${manifests_dir}/namespace.yaml" ]]; then

			kubectl apply -f "${manifests_dir}/namespace.yaml" "${kubectl_args[@]}" || {
				message "Failed to apply namespace from ${manifests_dir}/namespace.yaml"
				return 1
			}

		else

			message "ERROR: No namespace.yaml found"
			return 1

		fi

		# There needs to be a file named "rbac.yaml" iin the directory.
		if [[ -f "${manifests_dir}/rbac.yaml" ]]; then

			kubectl apply -f "${manifests_dir}/rbac.yaml" "${kubectl_args[@]}" || {
				message "Failed to apply rbac from ${manifests_dir}/rbac.yaml"
				return 1
			}

		else

			message "ERROR: No rbac.yaml found!"
			return 1

		fi

	else

		message "No directory found ${manifests_dir}, unable to apply RBAC"
		return 1

	fi

	return 0

}

function apply_manifests() {

	local manifests_dir="${1}"

	if [[ -d ${manifests_dir} ]]; then

		# There needs to be at least one yaml file in the directory.
		if [[ $(find "${manifests_dir}" -type f -name '*.yaml' -o -name '*.yml' | wc -l) -eq 0 ]]; then
			message "No yaml files found in ${manifests_dir}, skipping"
			return 0
		fi

		message "Applying manifests from ${manifests_dir}..."
		kubectl apply -f "${manifests_dir}" "${kubectl_args[@]}" || {
			message "Failed to apply manifests from ${manifests_dir}."
			return 1
		}

	else

		message "No directory found ${manifests_dir}, skipping"
		return 0

	fi

	return 0

}

function cleanup() {

	local releases
	local release
	local namespace

	message "Cleaning up..."

	# Record every release in the cluster.
	releases=$(helm list -A -o json | jq -r '.[] | .name + " " + .namespace')

	if [[ ${#releases} -eq 0 ]]; then
		message "No releases found in the cluster."
	else

		# Uninstall all helm releases in the cluster using the namespace.
		while read -r release namespace; do
			message "Uninstalling release: $release in namespace: $namespace"
			helm uninstall "$release" -n "$namespace" || {
				message "Failed to uninstall release: $release in namespace: $namespace"
				return 1
			}
			kubectl delete namespace "$namespace" --ignore-not-found
		done <<<"$releases"

	fi

	# Remove all helm repositories in the cluster.
	repositories=$(helm repo list -o json | jq -r '.[] | .name')

	if [[ ${#repositories} -eq 0 ]]; then
		message "No repositories found in the cluster."
	else

		# Uninstall all helm repositories in the cluster using the namespace.
		while read -r repository; do
			message "Uninstalling repository: $repository"
			helm repo remove "$repository" || {
				message "Failed to uninstall repository: $repository"
				return 1
			}
		done <<<"$repositories"

	fi

	message "Cleanup complete."
	exit 0
}

#--------------------------------------------------
# Load user variables.
#--------------------------------------------------

if [[ -f "values.sh" ]]; then
	# shellcheck disable=SC1091
	source "values.sh"
else
	message "values.sh not found."
	exit 1
fi

if [[ ${ENABLE_DRY_RUN:-false} == "true" ]]; then

	kubectl_args=(
		"--dry-run=server"
	)

	TEMP_DIR="$(mktemp --directory)"

fi

#--------------------------------------------------
# Cleanup
#--------------------------------------------------

# Used for local testing loop.
if [[ ${1:-} == "cleanup" ]]; then

	cleanup || {
		message "Failed to cleanup."
		exit 1
	}

	message "Cleanup complete."
	exit 0

fi

#--------------------------------------------------
# Pre-flight checks
#--------------------------------------------------

case $INGRESS_TYPE in

"contour")
	message "Enabling Contour ingress controller."
	ENABLE_CONTOUR="true"
	;;

"contour-gateway")
	message "Enabling Contour Gateway ingress controller."
	ENABLE_CONTOUR_GATEWAY="true"
	;;

"ako")
	message "Enabling AVI Network Kubernetes Operartor ingress controller."
	ENABLE_AKO="true"
	;;

"none")
	message "No ingress controller will be installed."
	;;

*)
	message "The ingress type ${INGRESS_TYPE} is not supported."
	exit 99
	;;

esac

#--------------------------------------------------
# Main
#--------------------------------------------------

echo -e "\n"
echo "This will install or upgrade the following charts into your connected cluster:"

for CHART in "${CHARTS_ATLASSIAN[@]}"; do
	echo -e "\t- ${CHART}"
done
echo -e "\n"
MESSAGE="Do you wish to continue? Y/N: "
read -rp "${MESSAGE}" -n 1 CHOICE
echo -e "\n"

if [[ ! ${CHOICE} =~ [Yy] ]]; then

	echo "Cancelled at user request."
	exit 0

fi

# Check the prerequisite tools are installed.
# tools: kubectl, helm v3+, kapp
check_tools || {
	message "Failed to find all required tools."
	exit 1
}

# Install the necessary helm repositories.
install_helm_repositories || {
	message "Failed to install helm repositories."
	exit 2
}

# Write a default values.yaml file for each chart.
install_helm_defaults || {
	message "Failed to install helm defaults."
	exit 3
}

#--------------------------------------------------
# Dependencies
#--------------------------------------------------

#-------------------------
# Cert Manager
#-------------------------

if [[ ${ENABLE_CERT_MANAGER} == "true" ]]; then

	apply_rbac "../cert-manager/manifests" || {
		message "Failed to create cert-manager namespace and permissions"
		exit 4
	}

	install_helm_release \
		cert-manager \
		"bitnami/cert-manager" \
		"cert-manager" \
		"../cert-manager/values${HELM_VALUES_SUFFIX:-}.yaml" || {
		message "Failed to install cert-manager."
		exit 4
	}

	if [[ ${ENABLE_DRY_RUN:-false} != "true" ]]; then

		message "Waiting for cert-manager pods to be ready..."

		kubectl wait --for=condition=Ready pods -l app.kubernetes.io/name=cert-manager -n cert-manager --timeout=300s || {
			message "Failed to wait for cert-manager pods to be ready."
			exit 4
		}
		message "Waiting for cert-manager certificate injector to be ready..."
		sleep 60
		message "Still waiting for cert-manager certificate injector to be ready..."
		sleep 60
		message "Still waiting for cert-manager certificate injector to be ready..."
		sleep 60
		message "Still waiting for cert-manager certificate injector to be ready..."

	fi

	# kubectl get crd issuers.cert-manager.io -oyaml | grep inject
	# cert-manager.io/inject-ca-from-secret: cert-manager/cert-manager-webhook-ca
	# cert-manager.io/inject-apiserver-ca: "true"
	# cert-manager.io/cluster-issuer: ca-selfsigned-issuer
	# cert-manager.io/issuer: ca-selfsigned-issuer
	apply_manifests "../cert-manager/manifests" || {
		message "Failed to apply cert-manager manifests."
		exit 4
	}

	message "Displaying cert-manager endpoints..."
	kubectl get endpoints -n cert-manager || true

fi

#-------------------------
# Ingress
#-------------------------

# Provision an ingress controller if the correct
# variable is set to true.

if [[ ${ENABLE_CONTOUR} == "true" ]]; then

	apply_rbac "../contour/manifests" || {
		message "Failed to create contour namespace and permissions"
		exit 4
	}

	install_helm_release \
		contour \
		"bitnami/contour" \
		"contour" \
		"../contour/values${HELM_VALUES_SUFFIX:-}.yaml" || {
		message "Failed to install Contour ingress controller."
		exit 5
	}

	if [[ ${ENABLE_DRY_RUN:-false} != "true" ]]; then

		message "Waiting for Contour pods to be ready..."

		kubectl wait --for=condition=Ready pods -l app.kubernetes.io/name=contour -n contour --timeout=300s || {
			message "Failed to wait for Contour pods to be ready."
			exit 5
		}

	fi

	apply_manifests "../contour/manifests" || {
		message "Failed to apply Contour manifests."
		exit 5
	}

fi

if [[ ${ENABLE_CONTOUR_GATEWAY} == "true" ]]; then

	message "Installing Contour Gateway Provisioner..."
	kubectl apply -f ../contour-gateway/install.yaml

	if [[ ${ENABLE_DRY_RUN:-false} != "true" ]]; then

		message "Waiting for Contour pods to be ready..."
		kubectl wait --for=condition=Ready pods -l control-plane=contour-gateway-provisioner -n projectcontour --timeout=300s || {
			message "Failed to wait for Contour pods to be ready."
			exit 5
		}

	fi

	apply_manifests "../contour-gateway/manifests" || {
		message "Failed to apply Contour Operator manifests."
		exit 6
	}

fi

if [[ ${ENABLE_AKO} == "true" ]]; then

	install_helm_release \
		ako \
		"ako/ako" \
		"avi-system" \
		"../ako/values${HELM_VALUES_SUFFIX:-}.yaml" || {
		message "Failed to install AVI Operator ingress controller."
		exit 7
	}

	if [[ ${ENABLE_DRY_RUN:-false} != "true" ]]; then

		message "Waiting for AVI Operator pods to be ready..."

		kubectl wait --for=condition=Ready pods -l app.kubernetes.io/name=ako-operator -n avi-system --timeout=300s || {
			message "Failed to wait for AVI Operator pods to be ready."
			exit 7
		}

	fi

	apply_manifests "../ako/manifests" || {
		message "Failed to apply AVI Operator manifests."
		exit 7
	}

fi

#-------------------------
# PostgreSQL
#-------------------------

# Provision a PostgreSQL database if the variable
# ENABLE_POSTGRESQL is set to true.

if [[ ${ENABLE_POSTGRESQL} == "true" ]]; then

	install_helm_release \
		postgresql-ha \
		"bitnami/postgresql-ha" \
		"postgresql-ha" \
		"../postgresql-ha/values${HELM_VALUES_SUFFIX:-}.yaml" || {
		message "Failed to install PostgreSQL."
		exit 8
	}

	apply_manifests "../postgresql/manifests" || {
		message "Failed to apply PostgreSQL manifests."
		exit 8
	}

fi

#-------------------------
# NFS
#-------------------------

# Configure an NFS server if the variable
# ENABLE_NFS is set to true.

if [[ ${ENABLE_NFS} == "true" ]]; then

	install_helm_release \
		nfs-ganesha \
		"nfs-ganesha-server-and-external-provisioner/nfs-server-provisioner" \
		"nfs-ganesha" \
		"../nfs-server-provisioner/values${HELM_VALUES_SUFFIX:-}.yaml" || {
		message "Failed to install NFS server."
		exit 9
	}

	apply_manifests "../nfs-server-provisioner/manifests" || {
		message "Failed to apply NFS server manifests."
		exit 9
	}

fi

#--------------------------------------------------
# Atlassian
#--------------------------------------------------

if [[ ${ENABLE_ATLASSIAN_HELM_CHARTS} == "true" ]]; then

	# Install the Atlassian helm charts.
	# The charts are installed in the order they are listed in the array.

	for chart_and_repo in "${CHARTS_ATLASSIAN[@]}"; do

		chart_repo=$(echo "${chart_and_repo}" | cut -d' ' -f1)
		chart_name=$(echo "${chart_and_repo}" | cut -d' ' -f2)
		chart_dir="charts/${chart_name}"
		chart_values="${chart_dir}/values${HELM_VALUES_SUFFIX:-}.yaml"
		manifests_dir="${chart_dir}/manifests"

		apply_rbac "${manifests_dir}" || {
			message "Failed to create ${chart_name} namespace and permissions"
			exit 10
		}

		install_helm_release \
			"${chart_name}" \
			"${chart_repo}/${chart_name}" \
			"${chart_name}" \
			"${chart_values}" || {
			message "Failed to install ${chart_name}."
			exit 10
		}

		apply_manifests "${manifests_dir}" || {
			message "Failed to apply manifests for ${chart_name}."
			exit 10
		}

	done

else

	message "Skipping Atlassian helm chart installation."

fi

if [[ ${ENABLE_DRY_RUN:-false} == "true" ]]; then
	message "DRY RUN: Templates recorded in ${TEMP_DIR}"
fi

exit 0
