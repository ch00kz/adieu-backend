{
  description = "Flake for rust dev";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };

  outputs = { self , nixpkgs ,... }: let
    system = "aarch64-darwin";
    # system = builtins.currentSystem;
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in pkgs.mkShell {
      packages = with pkgs; [
        rustup          # Rust toolchain installer - includes: cargo, rustc, rustfmt, rust-analyzer, etc
        sqlx-cli        # SQLx CLI - manage database creation, migrations, etc
        typeshare       # Command Line Tool for generating language files with typeshare
        postgresql_17   # PostgreSQL 17 - includes psql, createdb, createuser, dropdb, dropuser, etc
      ];

      shellHook = ''
        echo "Welcome to your developer environment!";
        echo "Exporting .env";
        set -a;
        source .env;
        set +a;
      '';
    };
  };
}
