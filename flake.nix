{
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.latest.withComponents [
          "cargo"
          "rustc"
          "clippy"
        ];
        rust = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      in rec {
        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues packages;
          nativeBuildInputs = [ pkgs.bloaty pkgs.cargo-watch ];
        };

        packages.default =
          let toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in rust.buildRustPackage {
            pname = toml.package.name;
            version = toml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
      });
}
