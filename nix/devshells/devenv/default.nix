{
  inputs,
  system,
  pkgs,
  crossPkgs,
  rustProfile,
  ...
}: let
  cargoComponents = with rustProfile.withComponents; [
    "cargo"
    "clippy"
    "rust-src"
    "rustc"
    "rustfmt"
  ];

  # Check if the package is supported on the current system
  muslSupported = !(builtins.elem system ["aarch64-darwin" "x86_64-darwin"]);
  musl =
    if muslSupported
    then pkgs.musl
    else null;

  specialPkgs = [
    musl
  ];
  supportedPkgs = builtins.filter (pkg: pkg != null) specialPkgs;
in
  inputs.devenv.lib.mkShell {
    inherit inputs;
    inherit pkgs;

    modules = [
      {
        # https://devenv.sh/reference/options/

        packages = with pkgs;
          [
            figlet
            hello

            nixpkgs-fmt
            statix

            sops
            #sops-init-gpg-key
            #sops-import-keys-hook
            ssh-to-age
            ssh-to-pgp
            age

            bash
            bash-completion

            gnutar

            # Spelling
            hunspell
            hunspellDicts.en_AU-large

            # Kubernetes
            kubectl
            kind

            # Rust
            rustup
            trunk
            (rustProfile.withComponents cargoComponents)

            # Other
            binutils
            bzip2
            clang
            cmake
            figlet
            file
            gcc
            gnutar
            lld
            openssl
            perl
            pkgconf
            xxd
            zlib
            zstd
          ]
          ++ supportedPkgs;

        env = {
          DEVENV_DEVSHELL_ROOT = builtins.toString ./.;
        };

        enterShell = ''
          # Cargo
          export CARGO_HOME=$PROJECT_DIR/.direnv/cargo
          mkdir -p $CARGO_HOME

          # Linters
          export HUNSPELL_CONFIG=''${PROJECT_DIR}/.linters/config/hunspell.conf
          export PRETTIER_CONFIG=''${PROJECT_DIR}/.linters/config/.prettierrc.yaml
          export YAMLLINT_CONFIG_FILE=''${PROJECT_DIR}/.linters/config/.yamllint.yml

          figlet $PROJECT_SHELL

          hello \
            --greeting \
            "
            Welcome ''${USER}!

            Project: ''${PROJECT_NAME:-Unknown}
            Shell: ''${PROJECT_SHELL:-Unknown}
            Directory: ''${PROJECT_DIR:-Unknown}
            "
        '';

        pre-commit = {
          default_stages = ["commit"];

          excludes = ["README.md"];

          hooks = {
            # Nix
            alejandra.enable = true;
            nixfmt.enable = false;
            nixpkgs-fmt.enable = false;
            deadnix.enable = true;
            statix.enable = true;

            # GitHub Actions
            actionlint.enable = true;

            # Bash
            bats.enable = true;
            shellcheck.enable = true;
            shfmt.enable = true;

            # Rust
            cargo-check.enable = true;
            clippy.enable = true;
            rustfmt.enable = true;

            # Spelling
            hunspell.enable = false;
            typos.enable = true;

            # Git commit messages
            commitizen.enable = true;

            # Markdown
            markdownlint = {
              enable = true;
            };
            mdsh.enable = true;

            # Common
            prettier.enable = true;

            # YAML
            yamllint.enable = true;
          };

          settings = {
            clippy = {
              denyWarnings = true;
              offline = false;
            };

            deadnix = {
              edit = false;
              noUnderscore = true;
              noLambdaPatternNames = true;
              noLambdaArg = true;
            };

            markdownlint = {
              config = {
                # No hard tabs allowed.
                no-hard-tabs = true;

                # First line headings
                MD041 = false;

                # Empty URLs
                MD042 = false;

                # Unordered list intendation.
                MD007 = {
                  indent = 2;
                };

                # Training spaces
                MD009 = {
                  br_spaces = 2;
                };

                # Line length
                MD013 = false;

                # Inline HTML
                MD033 = false;

                # List marker spaces.
                # Disabled for use with prettier.
                MD030 = false;
              };
            };

            prettier = {
              output = "check";
              write = true;
            };

            typos = {
              format = "long";
              diff = true;
              write = false;
            };

            yamllint = {
              configPath = ".linters/config/.yamllint.yml";
            };
          };
        };

        devcontainer.enable = true;

        devenv = {
          flakesIntegration = true;
          #warnOnNewVersion = true;
        };

        difftastic.enable = true;

        #hosts = {"example.com" = "1.1.1.1";};

        languages = {
          gawk = {enable = true;};

          nix = {enable = true;};

          rust = {
            enable = true;
            version = "stable";
          };
        };

        starship = {
          enable = true;
          package = pkgs.starship;
          config = {
            enable = true;
            path = "/home/$USER/.config/starship.toml";
          };
        };
      }
    ];
  }
