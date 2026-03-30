{
  pkgs,
  rustToolchain,
}:
pkgs.mkShell {
  packages = with pkgs; [
    rustToolchain
    rust-analyzer
    alejandra
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
  '';
}
