{
  description = "Bevy Development environment";

  inputs = {
    # Retrieve current Nixpkgs unstable channel
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # Retrieve the Rust overlay

    rust-overlay.url = "github:oxalica/rust-overlay";

    # Retrieve the flake-utils library
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
# To allow cursor to be used

    flake-utils.lib.eachDefaultSystem (system:
      # Define the variables to use in the in { ... } section
      let
        # Use a pre-built rust environment as a wrapper
        overlays = [ (import rust-overlay) ];
        # Shorthand the nix packages library
        pkgs = import nixpkgs {
            inherit system overlays;
            config = {
                allowUnfree = true;
            };
        };
        # Define which tools in the rust tool chain to use
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };
      in {

        # When a dev shell is launched, create a package config with the following nixPackages.
        #
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [
            rustToolchain
            clang
            llvmPackages_latest.bintools
            glibc.dev
            glib.dev
            bacon
            openssl
            cargo-binstall
            # udev
            # vulkan-loader
            # xorg.libX11
            # xorg.libXcursor
            # xorg.libXi
            # xorg.libXrandr
            # wayland
            # libxkbcommon
          ];

         # The following is executed once after a shell is launched but before
         # The use can input a command
          shellHook = ''
            # Define cargo and library paths
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export LD_LIBRARY_PATH=${
              pkgs.lib.makeLibraryPath [
                pkgs.vulkan-loader
                pkgs.libxkbcommon
                pkgs.wayland
                pkgs.alsa-lib
                pkgs.udev
                pkgs.openssl
              ]
            }:$LD_LIBRARY_PATH;
            export LIBCLANG_PATH="${pkgs.llvmPackages_latest.libclang.lib}/lib";

            # Set up the build environment
            export BINDGEN_EXTRA_CLANG_ARGS="-I${pkgs.glibc.dev}/include -I${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include -I${pkgs.glib.dev}/include/glib-2.0 -I${pkgs.glib.out}/lib/glib-2.0/include/";
            export RUSTFLAGS="-C link-arg=-fuse-ld=lld";

            # Ensure we're on the nightly build
            rustup install nightly;
            rustup override set nightly;
            rustup target install wasm32-unknown-unknown;
          '';
        };
      });
}
