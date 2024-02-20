{
  description = "Flake for working on NixOS.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { system = system; };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustup
            pkg-config
            gcc
            alsa-lib
            udev
            vulkan-loader
            xorg.libX11
            xorg.libXrandr
            xorg.libXcursor
            xorg.libXi
            libxkbcommon
            clang
            libclang
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.alsa-lib
              pkgs.udev
              pkgs.vulkan-loader
              pkgs.xorg.libX11
              pkgs.xorg.libXrandr
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.libxkbcommon
              pkgs.clang
              pkgs.libclang
          ];
        };
      });
    };
}
