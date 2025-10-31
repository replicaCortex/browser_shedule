{
  pkgs ? import <nixpkgs> { },
}:
let
  rust = with pkgs; [
    cargo
    rustc

    rustfmt
    clippy

    gdb
    rust-analyzer
  ];

  aliases = ''
    alias cr="cargo run"
    alias cb="cargo build"
    alias ct="cargo test"
    alias cc="cargo-clippy"
    alias g="rust-gdb -tui ./target/debug/browser_shedule"
    alias mv="mv -n"
  '';
in
pkgs.mkShell {
  buildInputs = rust;

  shellHook = aliases;
}
