{
  description = "Development environment for commit-email";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems =
        f: (nixpkgs.lib.genAttrs systems) (system: f system nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forAllSystems (
        system: pkgs:
        let
          rustToolchain = fenix.packages.${system}.stable.withComponents [
            "rustc"
            "rust-src"
            "rustfmt"
            "cargo"
            "clippy"
            "rust-analyzer"
          ];
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain

              nil
              nixfmt
            ];

            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
        }
      );
    };
}
