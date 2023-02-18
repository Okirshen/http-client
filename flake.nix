{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShell = pkgs.mkShell rec { 
          buildInputs = with pkgs; [ 
            rustc
            cargo
            rustPackages.clippy
            rustfmt
            xorg.libxcb
            pkg-config
            openssl
          ];
          libPath = with pkgs; lib.makeLibraryPath [
            libxkbcommon
            vulkan-loader
            libGL
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];

          LD_LIBRARY_PATH = libPath;
      };
    });
}
