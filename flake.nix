{
  description = "CRDT Zoo development shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          tlaPackages =
            if pkgs ? tlaplus then
              [ pkgs.tlaplus ]
            else if pkgs ? tla2tools then
              [ pkgs.tla2tools ]
            else
              [ ];
          # TLAPS 1.4.x sometimes invokes z3 without forwarding the SMT file
          # argument. Keep the proof command reproducible with current nixpkgs.
          tlapsZ3 = pkgs.writeShellScriptBin "z3" ''
            if [ "$#" -eq 0 ]; then
              smt_file="$(ls -t ./*.tlaps/tlapm_*.smt 2>/dev/null | head -n 1)"
              if [ -n "$smt_file" ]; then
                exec ${pkgs.z3}/bin/z3 -smt2 -v:0 AUTO_CONFIG=false smt.MBQI=true "$smt_file"
              fi
            fi

            exec ${pkgs.z3}/bin/z3 "$@"
          '';
          tlapsPackages =
            if pkgs ? tlaps then
              [ pkgs.tlaps tlapsZ3 pkgs.z3 ]
            else
              [ ];
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.clippy
              pkgs.just
              pkgs.lean4
              pkgs.mdbook
              pkgs.rust-analyzer
              pkgs.rustc
              pkgs.rustfmt
            ] ++ tlaPackages ++ tlapsPackages;

            RUST_BACKTRACE = "1";
          };
        });
    };
}
