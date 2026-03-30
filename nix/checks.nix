{
  pkgs,
  craneLib,
  src,
  cargoArtifacts,
  package,
  root,
}: let
  rustFmtCheck = craneLib.cargoFmt {
    inherit src;
  };

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
  ds-algo = package;

  fmt = pkgs.runCommand "ds-algo-fmt-check" {} ''
    test -e ${rustFmtCheck}
    test -e ${nixFmtCheck}
    touch "$out"
  '';

  clippy = craneLib.cargoClippy {
    inherit src cargoArtifacts;
    cargoClippyExtraArgs = "--workspace --all-targets --all-features -- -D warnings";
  };

  test = craneLib.cargoTest {
    inherit src cargoArtifacts;
    cargoExtraArgs = "--workspace";
  };
}
