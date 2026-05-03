{
  description = "A Sudoku solver using CryptoMiniSat and Rust";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          common = {
            version = "0.1.0";
            src = ./.;
            cargoHash = "sha256-OwqSLeHmZb8/oLIXERsYHprmq5bwJgRM5pbMZRK2ZiQ=";

            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [
              cryptominisat
              boost
              zlib
              gmp
            ];

            preBuild = ''
              for f in $(find /build -name build.rs | grep cryptominisat); do
                echo "Patching $f to use system cryptominisat"
                echo 'fn main() { println!("cargo:rustc-link-lib=dylib=cryptominisat5"); }' > "$f"
              done
            '';
          };
        in
        {
          default = pkgs.rustPlatform.buildRustPackage (
            common // { 
              pname = "sudoku-sat"; 
            }
          );
          test = pkgs.rustPlatform.buildRustPackage (
            common
            // {
              pname = "sudoku-sat-test";
              buildFeatures = [ "test-bin" ];
            }
          );
        }
      );

      checks = forAllSystems (system: {
        test-run = nixpkgs.legacyPackages.${system}.runCommand "sudoku-sat-test-run" { } ''
          ${self.packages.${system}.test}/bin/sudoku-sat
          touch $out
        '';
      });

      apps = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          descriptions = {
            default = "Sudoku Solver using CryptoMiniSat and Rust";
            test = "Sudoku Solver Tests";
          };
        in
        pkgs.lib.mapAttrs (name: pkg: {
          type = "app";
          program = "${pkg}/bin/sudoku-sat";
          meta.description = descriptions.${name} or "";
        }) self.packages.${system}
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              pkg-config
              cargo
              rustc
              rust-analyzer
              clippy
              rustfmt
            ];
            buildInputs = with pkgs; [
              cryptominisat
              boost
              zlib
              gmp
            ];
          };
        }
      );
    };
}
