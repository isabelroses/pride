{
  description = "pride cli";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
          system: function nixpkgs.legacyPackages.${system}
        );
    in
    {
      packages = forAllSystems (
        pkgs:
        let
          pride-cli = pkgs.callPackage ./default.nix { };
        in
        {
          inherit pride-cli;
          default = pride-cli;
        }
      );

      devShells = forAllSystems (pkgs: {
        default = pkgs.callPackage ./shell.nix { };
      });

      overlays.default = final: _: { example = final.callPackage ./default.nix { }; };
    };
}
