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
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {

        # Define the default package for building the application
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "dioxus-web";
          version = "0.1.0";
          
          src = ./.;
          
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
            clang
            llvmPackages_latest.bintools
            cargo-binstall
            nodejs
            nodePackages.npm
          ];
          
          buildInputs = with pkgs; [
            glibc.dev
            glib.dev
            openssl
            sqlite
            vulkan-loader
            libxkbcommon
            wayland
            alsa-lib
            udev
          ];
          
          buildPhase = ''
            export CARGO_HOME=$TMPDIR/cargo
            export PATH=$PATH:$CARGO_HOME/bin
            
            # Set up environment variables for building
            export LIBCLANG_PATH="${pkgs.llvmPackages_latest.libclang.lib}/lib"
            export BINDGEN_EXTRA_CLANG_ARGS="-I${pkgs.glibc.dev}/include -I${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include -I${pkgs.glib.dev}/include/glib-2.0 -I${pkgs.glib.out}/lib/glib-2.0/include/"
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
              pkgs.vulkan-loader
              pkgs.libxkbcommon
              pkgs.wayland
              pkgs.alsa-lib
              pkgs.udev
              pkgs.openssl
            ]}:$LD_LIBRARY_PATH
            
            # Set up database URL for build
            export DATABASE_URL="sqlite:main.db"
            
            # Install dioxus-cli
            cargo install dioxus-cli --version 0.6.3 --locked
            
            # Build the fullstack application
            cd source
            
            # Build the server binary with server features
            cargo build --release --features server
            
            # Build the web assets
            dx build --release --platform web
          '';
          
          installPhase = ''
            mkdir -p $out/bin
            mkdir -p $out/share/dioxus-web
            
            # Copy the server binary
            cp source/target/release/source $out/bin/dioxus-web
            
            # Copy the web assets
            cp -r source/dist/* $out/share/dioxus-web/
            
            # Copy the database file if it exists
            if [ -f source/main.db ]; then
              cp source/main.db $out/share/dioxus-web/
            fi
            
            # Copy any other necessary files
            if [ -f source/Dioxus.toml ]; then
              cp source/Dioxus.toml $out/share/dioxus-web/
            fi
          '';
        };

        # Load packages used for build time
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [
            # Build dependencies
            rustup
            rustToolchain
            clang
            llvmPackages_latest.bintools
            glibc.dev
            glib.dev
            openssl
            cargo-binstall
            sqlite
            # Load bacon, a rust CLI tool for convience
            bacon
          ];

         # The following is executed once after a shell is launched but before
         # The use can input a command
          shellHook = ''
            # Define cargo and library paths for pkgconfig
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

            # Start ZSH shell instead of bash shell
            zsh
          '';
        };
      });
}
