{
  description = "Packages for commit-email";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      nixpkgs,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems =
        f: (nixpkgs.lib.genAttrs systems) (system: f system nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (
        _: pkgs:
        let
          sharedPackageInfo = {
            pname = "commit-email";
            version = "0.3.0";
            meta = {
              description = "A tool that reminds you to commit with the correct email address";
              homepage = "https://github.com/mkqavi/commit-email";
              license = pkgs.lib.licenses.mit;
            };
          };
        in
        {
          default = pkgs.rustPlatform.buildRustPackage (
            finalAttrs:
            sharedPackageInfo
            // {
              src = pkgs.fetchFromGitHub {
                owner = "mkqavi";
                repo = "commit-email";
                tag = "v${finalAttrs.version}";
                hash = "sha256-fKN01zlQHeYZzXx6nz4iqEInGDITdPoC6uLx1guTNng=";
              };

              cargoHash = "sha256-TlMyDB1K+qPAHpjNWZWO8nJwdO/jndzCJqaapBqaYoo=";
            }
          );

          main = pkgs.rustPlatform.buildRustPackage (
            sharedPackageInfo
            // {
              version = sharedPackageInfo.version + "+main";
              src = pkgs.lib.cleanSource ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };
            }
          );
        }
      );
    };
}
