{sources ? import ./sources.nix}:

let 
  pkgs = import sources.nixpkgs{ overlays=[ (import sources.nixpkgs-mozilla) ]; };
  channel = "nightly";
  date = "2020-10-14";
  targets = [ "x86_64-pc-windows-gnu" ];
  chan = (pkgs.rustChannelOfTargets channel date targets);
in chan
