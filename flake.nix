{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
        };

        devShells.default = with pkgs; let
          toolchain = rust-bin.fromRustupToolchainFile ./toolchain.toml;
        in
          mkShell.override {stdenv = pkgs.pkgsCross.avr.stdenv;} {
            nativeBuildInputs = [
              toolchain
              avrdude
              ravedude
              rust-analyzer-unwrapped
              pkg-config
              systemdMinimal
              cargo-generate
              gdb
            ];
            packages = [
            ];
            inherit toolchain;
            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          };
      };
    };
}
