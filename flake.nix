{
  description = "Development environment for phobos";
 
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.flake-utils.url = "github:numtide/flake-utils";
 
  inputs.uefi-run.url = "github:yakuri354/uefi-run";
 
  outputs = { self, nixpkgs, rust-overlay, flake-utils, uefi-run }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
          toolchain = t: t.default.override {
            extensions = [ "rust-src" "rust-analyzer-preview" ];
            targets = [ "aarch64-unknown-none" ];
          };
          rust-dev = pkgs.rust-bin.selectLatestNightlyWith toolchain;
        in {
          defaultPackage = pkgs.buildEnv {
            name = "phobos-dev";
            paths = with pkgs; [
              rust-dev
              uefi-run.defaultPackage.${system}
              just
              jq
              buildPackages.buildPackages.qemu
            ];
          };
        }
      );
}
