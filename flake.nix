{
  description = "ds-algo development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    flake-parts,
    nixpkgs,
    crane,
    rust-overlay,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = {system, ...}: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "clippy"
            "rust-src"
            "rustfmt"
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;
        commonArgs = {
          inherit src;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        package = import ./nix/package.nix {
          inherit
            pkgs
            craneLib
            src
            cargoArtifacts
            rustToolchain
            ;
        };

        checks = import ./nix/checks.nix {
          inherit
            pkgs
            craneLib
            src
            cargoArtifacts
            package
            ;
          root = ./.;
        };
      in {
        formatter = pkgs.alejandra;

        packages.default = package;
        apps.default = {
          type = "app";
          program = "${package}/bin/ds-algo";
        };

        inherit checks;

        devShells.default = import ./nix/shell.nix {
          inherit pkgs rustToolchain;
        };
      };
    };
}
