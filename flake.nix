{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        inherit (pkgs) lib;
        fs = lib.fileset;

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # compile dependencies once
        vendoredCrates = craneLib.vendorCargoDeps {
          cargoLock = ./Cargo.lock;
        };

        srcForCrate = cratePath: fs.toSource {
          root = ./.;
          fileset = fs.unions [
            ./Cargo.lock
            # keep all Cargo.tomls as scaffolding for workspace
            (fs.fileFilter (file: file.name == "Cargo.toml") ./.)
            # add the source of the crate we want to compile
            (./. + ("/" + cratePath))
          ];
        };

        manifestFromPath = cratePath: (lib.importTOML (./. + ("/" + cratePath + "/Cargo.toml")));
        crateNameFromPath = cratePath: (manifestFromPath cratePath).package.name;
        crateVersionFromPath = cratePath: (manifestFromPath cratePath).package.version;

        workspaceDeps = cratePath: craneLib.buildDepsOnly {
          src = ./.;
          pname = crateNameFromPath cratePath;
          cargoVendorDir = vendoredCrates;
          cargoExtraArgs = "-p ${crateNameFromPath cratePath}";
        };

        buildWorkspaceCrate = cratePath: craneLib.buildPackage {
          src = srcForCrate cratePath;
          pname = crateNameFromPath cratePath;
          version = crateVersionFromPath cratePath;

          # create a src/lib.rs next to every Cargo.toml
          postUnpack = "${lib.getExe pkgs.fd} Cargo.toml -tf -x mkdir -p {//}/src ';' -x touch {//}/src/lib.rs";
          cargoArtifacts = workspaceDeps cratePath;
          cargoVendorDir = vendoredCrates;
          cargoExtraArgs = "-p ${crateNameFromPath cratePath}";
          doNotLinkInheritedArtifacts = true;

          buildInputs = with pkgs; [ pkg-config libiconv ]
            ++ (lib.optionals stdenv.isLinux [ xorg.libX11 xorg.libXi xorg.libXtst ])
            ++ (lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Cocoa ]);
        };

        workspaceCratePaths = (lib.importTOML ./Cargo.toml).workspace.members;

        # attrSet of crateName -> derivation for this workspace
        workspaceMembers = lib.listToAttrs (lib.forEach workspaceCratePaths (path: {
          name = crateNameFromPath path;
          value = buildWorkspaceCrate path;
        }));

        # builds every crate in the workspace
        workspace = pkgs.symlinkJoin {
          name = "kare-workspace";
          paths = lib.attrValues workspaceMembers;
        };
      in
      rec
      {
        checks = {
          inherit workspace;
        };

        apps = {
          kare = flake-utils.lib.mkApp { drv = self.outputs.packages."${system}".kare; };
          gkare = flake-utils.lib.mkApp { drv = self.outputs.packages."${system}".gkare; };
          default = apps.kare;
        };

        packages = rec {
          inherit workspace;
          default = workspace;
        } // workspaceMembers;

        devShells.default = pkgs.mkShell {
          name = "kare";
          inputsFrom = builtins.attrValues checks;
          nativeBuildInputs = with pkgs; [
            rustToolchain
          ];
          # RUST_SRC_PATH = "${toolchain.rust-src}/lib/rustlib/src/rust/library";
        };
      });
}
