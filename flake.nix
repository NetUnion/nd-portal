{
  description = "login portal";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" ];
    perSystem = { pkgs, ... }: {
      packages = rec {
        nd-portal = with pkgs; pkgsStatic.rustPlatform.buildRustPackage rec {
          name = "nd-portal";

          src = lib.cleanSource ./.;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [ pkg-config ];

          buildInputs = with pkgsStatic; [ openssl ];
        };
        default = nd-portal;
      };
    };
  };
}
