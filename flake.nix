{
  description = "Development environment for commit-email";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
    }:
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
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

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

              # Required by openssl-sys crate
              pkg-config
              openssl

              nil
              nixfmt
            ];

            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
        }
      );
    };
}
