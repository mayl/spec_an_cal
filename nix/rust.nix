{sources ? import ./sources.nix}:

let 
  pkgs = import sources.nixpkgs{ overlays=[ (import sources.nixpkgs-mozilla) ]; };
  channel = "nightly";
  date = "2020-10-14";
  targets = [ "x86_64-pc-windows-msvc" ];
  chan = (pkgs.rustChannelOf{ channel = channel; date = date; }).rust;
in chan
