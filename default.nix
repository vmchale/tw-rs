let
  pkgs = import <nixpkgs> {};
in
  pkgs.rustUnstable.cargo
