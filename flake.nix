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
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.clippy
              pkgs.just
              pkgs.lean4
              pkgs.rust-analyzer
              pkgs.rustc
              pkgs.rustfmt
            ] ++ tlaPackages;

            RUST_BACKTRACE = "1";
          };
        });
    };
}
