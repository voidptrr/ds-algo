{
  pkgs,
  craneLib,
  src,
  cargoArtifacts,
  rustToolchain,
  root,
}: let
  rustFmtCheck =
    pkgs.runCommand "ds-algo-rust-fmt-check"
    {
      nativeBuildInputs = [rustToolchain];
    } ''
      cd ${src}
      cargo fmt --all --check
      touch "$out"
    '';

  nixFmtCheck =
    pkgs.runCommand "ds-algo-nix-fmt-check"
    {
      nativeBuildInputs = [pkgs.alejandra];
    } ''
      cd ${root}
      shopt -s globstar nullglob
      nix_files=(./flake.nix ./**/*.nix)
      alejandra --check "''${nix_files[@]}"
      touch "$out"
    '';
in {
  fmt = pkgs.runCommand "ds-algo-fmt-check" {} ''
    test -e ${rustFmtCheck}
    test -e ${nixFmtCheck}
    touch "$out"
  '';

  clippy = craneLib.cargoClippy {
    inherit src cargoArtifacts;
    cargoClippyExtraArgs = "-p leetcode --all-targets --all-features -- -D warnings";
  };

  test = craneLib.cargoTest {
    inherit src cargoArtifacts;
    cargoExtraArgs = "-p leetcode";
  };
}
