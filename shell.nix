let
  pkgs = import <nixpkgs> {};
in
pkgs.clangStdenv.mkDerivation {
  name = "flatbuffers-example";
  version = "0.0.0.1";
  src = pkgs.lib.cleanSource ./.;
  buildInputs = with pkgs; [
    flatbuffers
    grpc
    protobuf
    rustc
    cargo
  ];
}
