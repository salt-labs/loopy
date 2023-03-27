# loopy

## Overview

A Kubernetes packaging helper utility that tries to prevent you from going [loopy](https://www.urbandictionary.com/define.php?term=Loopy).

It's useful for those times when you find yourself testing the same inner-feedback loop like repeatedly installing and upgrading the same set of Helm charts on a `kind` cluster for a one line change.

![loopy](./docs/images/loopy.gif)

## Installation

For Rust users, `loopy` is available with `cargo`.

```bash
cargo install loopy
```

For everyone else, pre-packaged binaries are available in [releases](https://github.com/salt-labs/looper/releases).

## Usage

- Define your configuration in `loopy.yaml`

```bash
vim loopy.yaml
```

- Deploy a `kind` cluster

```bash
kind create cluster --name loopy --config capi/kind/config.yaml
```

- Run `loopy`

```bash
loopy --config loopy.yaml
```

## Dependencies

If you don't already have the dependencies installed, `loopy` will ask to install them for you if you are internet connected.

Once that's done you can add the `vendor` folder to the PATH of your current shell and go from there.

```bash
export PATH=$(pwd)/vendor:$PATH
```
