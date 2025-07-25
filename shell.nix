{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-25.05.tar.gz") {} }:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.pkg-config
  ];
  buildInputs = [
    pkgs.cacert
    pkgs.rustup
    pkgs.perl
    pkgs.cmake
    pkgs.clang
    pkgs.protobuf               # api
    pkgs.go                     # go api
    pkgs.protoc-gen-go          # go api
    pkgs.protoc-gen-go-grpc     # go api
    pkgs.protoc-gen-grpc-web    # grpc-web api
    pkgs.nodejs                 # js api + ui
    pkgs.yarn                   # ui
    pkgs.openssl
    pkgs.sqlite                 # sqlite binary + library for diesel
    pkgs.postgresql             # psql binary + library for diesel
    pkgs.cargo-cross            # cross-compiling
    pkgs.cargo-deb              # deb packaging
    pkgs.diesel-cli             # diesel cli
    pkgs.jq                     # json query cli tool
  ];
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include";
  DOCKER_BUILDKIT = "1";
  NIX_STORE = "/nix/store";
}
