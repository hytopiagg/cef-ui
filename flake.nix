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
              xorg.libXdamage
              xorg.libXext
              xorg.libXfixes
              xorg.libXcomposite
              xorg.libxcb
              libxkbcommon
              clang
              libclang
              glib
              pango
              libdrm
              dbus
              cups
              cairo
              nss
              nspr
              atk
              expat
              mesa
            ];

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.rustup
              pkgs.alsa-lib
              pkgs.udev
              pkgs.vulkan-loader
              pkgs.xorg.libX11
              pkgs.xorg.libXrandr
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.xorg.libXdamage
              pkgs.xorg.libXext
              pkgs.xorg.libXfixes
              pkgs.xorg.libXcomposite
              pkgs.xorg.libxcb
              pkgs.libxkbcommon
              pkgs.clang
              pkgs.libclang
              pkgs.glib
              pkgs.pango
              pkgs.libdrm
              pkgs.dbus              
              pkgs.cups
              pkgs.cairo
              pkgs.nss
              pkgs.nspr
              pkgs.atk
              pkgs.expat
              pkgs.mesa
            ];

            shellHook = ''
              export PATH=$HOME/.cargo/bin:$PATH
          '';
          };
        });
      };
}
