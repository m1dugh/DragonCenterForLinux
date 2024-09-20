# MSI Dragon center for linux

## Introduction

This software is intended to replace the MSI dragon center. It is based on the
[MSI embedded controller](https://github.com/BeardOverflow/msi-ec).

## Installation

### Nix

This package comes with a Nix flake and can be installed like this:
In your flake:
```flake.nix
    # ...
    inputs.dragon-center.url = "github:m1dugh/DragonCenterForLinux";

    # ...

    outputs =
    {
        # ...
        , dragon-center
        # ...
    }: {
        # ...
    }
```

In your configuration:

```configuration.nix
{ system
, dragon-center
, ...
}:
{
    environment.systemPackages = [

        # ...

        dragon-center.packages.${system}.default

        # ...
    ];
}
```

for the package, or
```configuration.nix
{ system
, dragon-center
, ...
}:
{
    imports = [
        dragon-center.nixosModules.default
    ];

    hardware.msi.dragon-center = {
        enable = true;
        driver.enable = true;
    };
}
```

Which adds the `msi-ec` driver to your config.
