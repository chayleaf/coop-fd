{
  # internet is super slow for me right now, so I'm using the branch I have extensively cached
  #inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
  inputs.nixpkgs.url = "github:chayleaf/nixpkgs/e70edbbc30bca7d90c4a1e8c653ceb1607cc2858";
  inputs.crane.url = "github:ipetkov/crane";
  inputs.crane.inputs.nixpkgs.follows = "nixpkgs";
  inputs.flake-compat = {
    url = "github:edolstra/flake-compat";
    flake = false;
  };

  outputs = { crane, nixpkgs, ... }: let
    inherit (nixpkgs) lib;
    mkPkgs = pkgs: let
      craneLib = (crane.mkLib pkgs);
      clang = pkgs.clang;
      libclang = pkgs.libclang.lib;
    in {
      default = craneLib.buildPackage {
        pname = "coop-ofd";
        version = "0.1.0";
        src = nixpkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            lib.hasSuffix ".js" path
            || lib.hasSuffix ".map" path
            || craneLib.filterCargoSources path type;
        };
        # pkgs/build-support/rust/hooks/rust-bindgen-hook.sh
        # seemingly crane doesn't use it by default?
        preBuild = ''
          export LIBCLANG_PATH="${libclang}/lib";
          export BINDGEN_EXTRA_CLANG_ARGS="$(< ${clang}/nix-support/cc-cflags) $(< ${clang}/nix-support/libc-cflags) $(< ${clang}/nix-support/libcxx-cxxflags) $NIX_CFLAGS_COMPILE"
        '';
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = [ libclang ] ++ (with pkgs; [ tesseract leptonica ]);
      };
    };
  in {
    devShells = lib.genAttrs ["x86_64-linux" "aarch64-linux"] (system: let
      pkgs = import nixpkgs { inherit system; };
    in {
      default = import ./shell.nix { inherit pkgs; };
    });
    nixosModules.default = { pkgs, ... }: {
      imports = [ ./module.nix ];
      services.coop-ofd.package = lib.mkDefault (mkPkgs pkgs).default;
    };
    overlays.default = final: prev: mkPkgs final;
    packages = lib.genAttrs ["x86_64-linux" "aarch64-linux"] (system: mkPkgs (import nixpkgs { inherit system; }));
  };
}
