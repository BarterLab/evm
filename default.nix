{ pkgs ? (
  let
    nixpkgs = import <nixpkgs>;
    pkgs_ = (nixpkgs {});
    rustOverlay = (pkgs_.fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      rev = "e2a920faec5a9ebd6ff34abf072aacb4e0ed6f70";
      sha256 = "1lq7zg388y4wrbl165wraji9dmlb8rkjaiam9bq28n3ynsp4b6fz";
    });
  in (nixpkgs {
    overlays = [
      (import (builtins.toPath "${rustOverlay}/rust-overlay.nix"))
      (self: super: {
        rust = {
          rustc = super.rustChannels.stable.rust;
          cargo = super.rustChannels.stable.cargo;
        };
        rustPlatform = super.recurseIntoAttrs (super.makeRustPlatform {
          rustc = super.rustChannels.stable.rust;
          cargo = super.rustChannels.stable.cargo;
        });
      })
    ];
  }))
}:

with pkgs;

let

env = stdenv.mkDerivation {
  name = "sputnikvm-env";
  buildInputs = [
    rustc cargo capnproto
  ];
};
sputnikvm = rustPlatform.buildRustPackage (rec {
  name = "sputnikvm-${version}";
  version = "0.1.0";
  src = ./.;
  depsSha256 = "14i5gh4fvg4fpnirwrzmgycxqm118p1p5c1mnf386223vdg2qisp";
  buildInputs = [ capnproto ];
  doCheck = true;
  checkPhase = ''
    ${capnproto}/bin/capnp eval -b tests/mod.capnp all >> tests.bin
    cargo test
    target/release/gaslighter --capnp_test_bin tests.bin --artefact_dir target/release/
  '';
  });
in {
  inherit env sputnikvm;
}