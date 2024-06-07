{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05-small";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat.url = "github:edolstra/flake-compat";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs =
  { self
  , nixpkgs
  , flake-utils
  , rust-overlay
  , ...
  }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs { inherit system; overlays = [ (import rust-overlay) ]; };

    rust178 = pkgs.rust-bin.stable."1.78.0".default.override {
      extensions = [
        "rust-src"
        "rust-analyzer"
        "clippy"
      ];
    };

    devBuildInputs = with pkgs; [
      rust178
      nodejs_18
      cargo-watch
      git
      gitflow
      lazygit
    ];
  in
  {
    devShells = {
      default = pkgs.mkShell {
        buildInputs = devBuildInputs;
      };
    };
  });
}
