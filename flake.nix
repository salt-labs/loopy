{
  description = "loopy";

  inputs = {
    nixpkgs = {
      type = "github";
      owner = "NixOS";
      repo = "nixpkgs";
      ref = "nixos-22.11";
      flake = true;
    };

    nixpkgs-unstable = {
      type = "github";
      owner = "NixOS";
      repo = "nixpkgs";
      ref = "nixos-unstable";
      flake = true;
    };

    devenv = {
      type = "github";
      owner = "cachix";
      repo = "devenv";
      ref = "main";
      flake = true;
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      type = "github";
      owner = "nix-community";
      repo = "fenix";
      ref = "main";
      flake = true;
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    nixpkgs-unstable,
    devenv,
    fenix,
    ...
  } @ inputs: let
    supportedSystems = [
      "aarch64-darwin"
      "aarch64-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];

    _targets = [
      "aarch64-apple-darwin"
      "aarch64-unknown-linux-gnu"
      "i686-unknown-linux-gnu"
      "x86_64-apple-darwin"
      "x86_64-unknown-linux-gnu"
    ];

    forAllSystems = f:
      builtins.listToAttrs (map (buildPlatform: {
          name = buildPlatform;
          value = builtins.listToAttrs (map (hostPlatform: {
              name = hostPlatform;
              value = f buildPlatform hostPlatform;
            })
            supportedSystems);
        })
        supportedSystems);

    pkgsImportCrossSystem = buildPlatform: hostPlatform:
      import nixpkgs {
        system = buildPlatform;
        overlays = [];
        config = {
          allowUnfree = true;
          allowUnfreePredicate = _: true;
        };
        crossSystem =
          if buildPlatform == hostPlatform
          then null
          else {
            config = hostPlatform;
          };
      };

    flattenPackages = systems:
      builtins.foldl' (acc: system:
        builtins.foldl' (
          innerAcc: hostPlatform:
            innerAcc // {"${system}.${hostPlatform}" = systems.${system}.${hostPlatform};}
        )
        acc (builtins.attrNames systems.${system})) {} (builtins.attrNames systems);
  in {
    ###############
    ## Packages
    ###############

    packages = flattenPackages (forAllSystems (buildPlatform: hostPlatform: let
      # Build Platform
      system = buildPlatform;
      pkgs = pkgsImportCrossSystem buildPlatform buildPlatform;

      # Rust
      rustProfile = fenix.packages.${system}.complete;
      rustToolchain = rustProfile.toolchain;
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };

      # Host Platform
      crossPkgs = pkgsImportCrossSystem buildPlatform hostPlatform;
      #defaultPackage.${buildPlatform} = self.packages."${buildPlatform}.${hostPlatform}".loopy;
    in {
      loopy = import ./nix/packages/loopy {
        inherit pkgs;
        inherit crossPkgs;
        inherit rustPlatform;
      };
    }));

    # Set the default package for the current system.
    defaultPackage = builtins.listToAttrs (map (system: {
        name = system;
        value = self.packages."${system}.${system}".loopy;
      })
      supportedSystems);

    ###############
    ## DevShells
    ###############

    devShells = flattenPackages (forAllSystems (buildPlatform: hostPlatform: let
      # Build Platform
      system = buildPlatform;
      pkgs = pkgsImportCrossSystem buildPlatform buildPlatform;

      # Rust
      rustProfile = fenix.packages.${system}.complete;

      # Host Platform
      crossPkgs = pkgsImportCrossSystem buildPlatform hostPlatform;
    in {
      devenv = import ./nix/devshells/devenv {
        inherit inputs;
        inherit system;
        inherit pkgs;
        inherit crossPkgs;
        inherit rustProfile;
      };
    }));

    # Set the default devshell to the one for the current system.
    devShell = builtins.listToAttrs (map (system: {
        name = system;
        value = self.devShells."${system}.${system}".devenv;
      })
      supportedSystems);
  };
}
