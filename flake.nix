{
  description = "A Rust project.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";

    rust-overlay.inputs = {
      nixpkgs.follows = "nixpkgs";
    };

    sm-json-data = {
      url = "github:vg-json-data/sm-json-data";
      flake = false;
    };
  };

  outputs = {
    flake-utils,
    nixpkgs,
    rust-overlay,
    sm-json-data,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
      in {
        devShell = pkgs.mkShell {
          SM_JSON_DATA = sm-json-data;

          nativeBuildInputs = [
            pkgs.rust-bin.stable.latest.default
          ];
        };
      }
    );
}
