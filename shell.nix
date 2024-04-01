{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-23.11.tar.gz") {} }:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.pkg-config
  ];
  buildInputs = [
    pkgs.cacert
    pkgs.rustup
    pkgs.protobuf
    pkgs.perl
    pkgs.cmake
    pkgs.clang
    pkgs.postgresql              # needed to build the diesel cli utility
    pkgs.go                     # go api
    pkgs.nodejs                 # js api + ui
    pkgs.yarn
    pkgs.protoc-gen-grpc-web    # grpc-web api
    pkgs.protoc-gen-go          # go api
    pkgs.protoc-gen-go-grpc
  ];
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include";
  DOCKER_BUILDKIT = "1";
  NIX_STORE = "/nix/store";
}
