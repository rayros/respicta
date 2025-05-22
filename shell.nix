let
  rustVersion = "latest";
  rust_overlay = import (
    builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"
  );
  pkgs = import <nixpkgs> {
    overlays = [
      rust_overlay
      (_: prev: {
        my-rust = prev.rust-bin.stable.${rustVersion}.default.override {
          extensions = [
            "rust-src" # for rust-analyzer
            "rust-analyzer"
          ];
        };
      })
    ];

  };
in
pkgs.mkShell {
  packages = with pkgs; [
    my-rust
    pkg-config
    openssl
    nasm
    imagemagick
    clang
    llvmPackages.libclang
    just
    gifsicle
  ];

  shellHook = ''
    export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
    export LD_LIBRARY_PATH=${pkgs.llvmPackages.libclang.lib}/lib:$LD_LIBRARY_PATH
    export PKG_CONFIG_PATH=${pkgs.llvmPackages.libclang}/lib/pkgconfig:$PKG_CONFIG_PATH
    mkdir -p tmp
    export TMPDIR=$(pwd)/tmp
  '';
}