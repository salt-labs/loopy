# Kind

## Overview

Notes on working with `kind` for local testing.

## Local Testing

- Kind with persistent storage.

```bash
# Create the following directory structure for the persistent storage
# or change the paths in the kind config.yaml to suit your needs.
mkdir --parents /mnt/kind/{config,hostpath-provisioner}

# 'YOLO' the permissions on the directories.
chmod 777 /mnt/kind/{config,hostpath-provisioner}

# Copy the local storage configuration file.
cp --force ../kind/hostpath-provisioner.yaml /mnt/kind/config/hostpath-provisioner.yaml

# Deploy the cluster
kind create cluster --name loopy --config ./config/capi/kind/config.yaml

kind get clusters
kubectl get nodes
```
