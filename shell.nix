let
  pkgs = import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [
        pkgs.cargo
        pkgs.cargo-update
        pkgs.clippy
        pkgs.rustPlatform.rustcSrc
        pkgs.rustc
        pkgs.rustfmt
    ];
  };
in with pkgs;

mkShell {
  name = "podcasters-collector";
  buildInputs = [
    libiconv
    openssl
    pkgconfig
    rust-analyzer
    rust-toolchain
    statix
  ] ++ 
  lib.optionals (!stdenv.isDarwin) [
    procps
  ] ++
  lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
    darwin.libobjc
  ]
  ;

  NIX_ENFORCE_PURITY = 0;
  NIX_SHELL_PRESERVE_PROMPT=1;
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  #shellHook = ''
  #  export RUST_SRC_PATH=${pkgs.rustPlatform.rustcSrc}
  #'';

}

