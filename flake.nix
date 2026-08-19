{
  description = "Packages for commit-email";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust_1_79_0.url = "github:NixOS/nixpkgs/c3392ad349a5227f4a3464dce87bcc5046692fce";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust_1_79_0,
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
      packages = forAllSystems (
        system: pkgs: {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "commit-email";
            version = "0.2.2+main";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };

          stable =
            let
              rustPkgs = rust_1_79_0.legacyPackages.${system};
            in
            rustPkgs.rustPlatform.buildRustPackage rec {
              pname = "commit-email";
              version = "0.2.1";
              src = rustPkgs.fetchFromGitHub {
                owner = "mkqavi";
                repo = "commit-email";
                rev = "v${version}";
                hash = "sha256-xMUulFLYW+txcb0pjaME4mMs+jaCigIi9bcghELfph8=";
              };

              buildInputs =
                with rustPkgs;
                [
                  zlib
                  openssl
                ]
                ++ lib.optionals stdenv.isDarwin [
                  rustPkgs.darwin.apple_sdk.frameworks.Security
                  rustPkgs.darwin.apple_sdk.frameworks.SystemConfiguration
                ];

              nativeBuildInputs = with rustPkgs; [
                pkg-config
              ];

              cargoHash = "sha256-ND/F0qpqoUFsejMekO07RigTbr1SNFdb/2CNa+8KbJI=";
            };
        }
      );
    };
}
