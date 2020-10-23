let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix{inherit sources;};
  pkgs = import sources.nixpkgs{};
  mingw = (import <nixpkgs>{ crossSystem = { config = "x86_64-pc-mingw32"; libc = "msvcrt"; crossThreads = true; }; }).buildPackages.gcc;
in
pkgs.mkShell {
    buildInputs = [ 
	rust 
	#mingw
	pkgs.pkgsCross.mingwW64.buildPackages.gcc
    ];
}
