{
  nixConfig.bash-prompt-prefix = "(nix) ";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
          ];
        };
      });
}
