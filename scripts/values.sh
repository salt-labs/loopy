#!/usr/bin/env bash

#--------------------------------------------------
# Variables (User)
#--------------------------------------------------

# These are variables expected to be set by the user.

# Enable this if you want cert-manager installed.
export ENABLE_CERT_MANAGER="false"

# Set this to be one of the supported ingress controllers.
# Options are; contour, contour-gateway, ako, none
export INGRESS_TYPE="contour"

# Enable this if you want PostgreSQL installed.
export ENABLE_POSTGRESQL="false"

# Enable this if you intend on using a shared-home volume.
# This will install a non-production NFS server.
export ENABLE_NFS="false"

# Enable this to install all configured Atlassian Helm charts.
export ENABLE_ATLASSIAN_HELM_CHARTS="true"

#--------------------------------------------------
# Helm Values
#--------------------------------------------------

# There are a couple of example values files in this repo.
# You can set a suffix to be appended to the end of the values file name.
# Example using a suffix of '-ABC' will turn 'values.yaml' into 'values-ABC.yaml'

# Each one of these examples is configured for different purposes.

# The default 'values.yaml' is overridden everytime with the values from the Helm chart.
#export HELM_VALUES_SUFFIX=""

# Current status: WIP
# Option 1: Setup for use with INGRESS_TYPE=contour and HTTP apps
export HELM_VALUES_SUFFIX=".1"

# Current status: TODO
# Option 2: Setup for use with INGRESS_TYPE=contour and HTTPS apps
#export HELM_VALUES_SUFFIX=".2"

# Current status: TODO
# Option 3: Setup for use with INGRESS_TYPE=contour-gateway and HTTPS apps
#export HELM_VALUES_SUFFIX=".3"

# Current status: TODO
# Option 4: Setup for use with INGRESS_TYPE=ako and HTTPS apps
#export HELM_VALUES_SUFFIX=".4"
