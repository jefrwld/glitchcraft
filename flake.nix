{
  description = "FFglitch Glitch Art Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        ffglitch = pkgs.callPackage ./ffglitch.nix {};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            ffmpeg-full
            ffglitch
          ];
          
          shellHook = ''
            echo "=== FFglitch Art Environment ==="
            echo "FFmpeg: $(ffmpeg -version | head -n1)"
            echo "FFglitch: $(ffglitch --version 2>&1 || echo 'FFglitch available')"
            echo "=========================="
          '';
        };
      });
}
