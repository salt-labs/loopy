# Readme

## Table of Contents

<!-- TOC -->

- [Readme](#readme)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Quickstart](#quickstart)
  - [Local Testing](#local-testing)

<!-- /TOC -->

## Overview

In this directory, you will find instructions for getting the Helm charts for deploying Atlassian products onto Tanzu in a default configuration.

These are the data center helm charts available from [here](https://atlassian.github.io/data-center-helm-charts/)

This might be useful for a quick development loop to test changes or different versions.

## Quickstart

A quickstart script for deploying Atlassian products with Helm using default values.

**NOTE:** Before starting ensure you meet the minimum requirements.

- Run all commands from the `helm/atlassian` directory
- You are connected to the internet
- You have both `kubectl` v1.21+ and `helm` v3 installed
- You are connected to a kubernetes cluster v1.21+

- Set your values in the `quickstart_values.sh` file.

```bash
vim quickstart_values.sh
```

- Ensure you have internet access and `kubectl` connected to the destination cluster and run the following.

```bash
./quickstart.sh
```

- Should you wish to undo all changes, run a cleanup.

```bash
./quickstart.sh cleanup
```

## Local Testing

If you want to deploy a `kind` cluster for local testing with Docker, you can run the following.

```bash
# Create the following directory structure for the persistent storage
# or change the paths in the kind config.yaml to suit your needs.
mkdir --parents /mnt/kind/{config,hostpath-provisioner}

# 'YOLO' the permissions on the directories.
chmod 777 /mnt/kind/{config,hostpath-provisioner}

# Copy the local storage configuration file.
cp --force ../kind/hostpath-provisioner.yaml /mnt/kind/config/hostpath-provisioner.yaml

# Deploy the cluster
kind create cluster --name atlassian --config ../kind/config.yaml

kind get clusters
kubectl get nodes
```
