{
  pkgs,
  rustToolchain,
}:
pkgs.mkShell {
  packages = with pkgs; [
    rustToolchain
    rust-analyzer
    alejandra
    nil
  ];
}
