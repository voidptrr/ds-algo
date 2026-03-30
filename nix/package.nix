{
  pkgs,
  craneLib,
  src,
  cargoArtifacts,
  rustToolchain,
}: let
  cargoToml = builtins.fromTOML (builtins.readFile (src + "/Cargo.toml"));
in
  craneLib.buildPackage {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;
    inherit src cargoArtifacts;
    strictDeps = true;
    doCheck = false;

    nativeBuildInputs = [pkgs.makeWrapper];

    postFixup = ''
      wrapProgram "$out/bin/ds-algo" --prefix PATH : ${pkgs.lib.makeBinPath [rustToolchain]}
    '';

    meta = {
      mainProgram = "ds-algo";
    };
  }
