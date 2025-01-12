# MSI Dragon center for linux

## Introduction

This software is intended to replace the MSI dragon center. It is based on the
[MSI embedded controller](https://github.com/BeardOverflow/msi-ec).

## Installation

### Nix package manager

To install the package using `nix`, run the following command:
```bash
nix profile install github:m1dugh/DragonCenterForLinux#dragon-center
```

## Running

The project can be ran using the command.
```
dragon-center
```

which requires the use of polkit in order to be ran as root.
