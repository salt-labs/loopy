# loopy

## Overview

A Kubernetes packaging helper utility that tries to prevent you from going [loopy](https://www.urbandictionary.com/define.php?term=Loopy).

It's useful for those times when you find yourself testing the same [inner loop](https://tanzu.vmware.com/developer/tv/talk/118/) like repeatedly installing and upgrading the same set of Helm charts on a `kind` cluster after only a couple lines of changes to a values file.

![loopy](./docs/images/loopy.gif)

## Installation

- For Rust users, `loopy` is available with `cargo`.

```bash
cargo install loopy
```

- For everyone else, pre-packaged binaries are available in [releases](https://github.com/salt-labs/loopy/releases).

## Usage

Once you have `loopy` installed, there are couple of things you need to do to get up and running.

- Create the folder structure.

```bash
# Where you want to store your loopy configuration.
LOOPY_HOME="${HOME}/loopy"

# Create the folder structure.
mkdir --parents "${LOOPY_HOME}/config/{capi,helm,manifests,carvel}"

cd "${LOOPY_HOME}"
```

- Define your configuration in `loopy.yaml`

```bash
# If you need a starter sample.
curl --output "${LOOPY_HOME}/loopy.yaml" https://raw.githubusercontent.com/salt-labs/loopy/trunk/config/loopy.yaml

vim loopy.yaml
```

- If you need a `kind` cluster for local testing, there is a sample configuration in the `config/capi/kind` directory with usage instructions in the [kind](./docs/kind.md) section.

- Before continuing, ensure you are connected to the cluster.

```bash
kubectl cluster-info
```

- Review the available commands.

```bash
loopy --help
```

- When ready, run `loopy`

```bash
loopy --config loopy.yaml --action install
```

- When finished, run `loopy` again to clean up.

```bash
loopy --config loopy.yaml --action uninstall
```

## Dependencies

If you don't already have the dependencies installed, `loopy` will ask to install them for you if you are internet connected.

Once that's done you can add the `vendor` folder to the PATH of your current shell and go from there.

```bash
export PATH=$(pwd)/vendor:$PATH
```

## Development

There are a number of packages required for development.

You can either use the DevShell contained within this repositories Nix flake or install some of the following packages depending on your Operating System.

```bash
# CentOS
yum install \
    autoconf \
    clang \
    cmake \
    file-devel \
    gcc \
    git \
    make \
    openssl \
    openssl-devel

# Ubuntu
sudo apt update
sudo apt install -y \
    autoconf \
    clang \
    cmake \
    gcc \
    git \
    libmagic-dev \
    libssl-dev \
    make \
    openssl
```
