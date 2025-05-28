{
  description = "OCaml Template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        ocamlPackages = pkgs.ocamlPackages;

        buildInputs = [
          # Add library dependencies here
          ocamlPackages.findlib
          ocamlPackages.ppx_deriving
          ocamlPackages.ppx_inline_test
          ocamlPackages.re
        ];

        nativeBuildInputs = with pkgs; [
          ocamlPackages.ocaml
          ocamlPackages.dune_3
          ocamlPackages.findlib
          ocamlPackages.utop
          ocamlPackages.ocamlformat
          ocamlPackages.ocaml-lsp
        ];
      in
      {
        packages = {
          default = ocamlPackages.buildDunePackage {
            pname = "template";
            version = "0.0.0";
            duneVersion = "3";
            src = ./.;

            strictDeps = true;

            inherit nativeBuildInputs buildInputs;
          };
        };

        devShells.default = pkgs.mkShell { inherit nativeBuildInputs buildInputs; };
      }
    );
}
