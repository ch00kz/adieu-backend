{
  description = "Adieu Backend Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, unstable, ... }: let
    system = "aarch64-darwin";
    pkgs = import nixpkgs {
      inherit system;
    };
    unstablePkgs = import unstable {
      inherit system;
    };
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in pkgs.mkShell {
      packages = [
        pkgs.just                   # Just command runner
        pkgs.rustup                 # Rust toolchain installer - includes: cargo, rustc, rustfmt, rust-analyzer, etc
        pkgs.sqlx-cli               # SQLx CLI - manage database creation, migrations, etc
        pkgs.postgresql_17          # PostgreSQL 17 - includes psql, createdb, createuser, dropdb, dropuser, etc
        unstablePkgs.typeshare      # Command Line Tool for generating language files with typeshare
      ];

      shellHook = ''
        echo "Exporting .env";
        set -a;
        source .env;
        set +a;
      '';
    };
  };
}
