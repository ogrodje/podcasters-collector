let
  pkgs =
    import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [pkgs.rustfmt pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc];
  };
in with pkgs;
mkShell {
  name = "anchor-collector";
  buildInputs = [
    darwin.apple_sdk.frameworks.Security
    libiconv
    openssl
    pkgconfig
    rust-analyzer
    rust-toolchain
  ];
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  #shellHook = ''
  #  export RUST_SRC_PATH=${pkgs.rustPlatform.rustcSrc}
  #'';

}

