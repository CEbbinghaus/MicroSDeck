{ pkgs ? import <nixpkgs> {} }:
	let
	overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
in
pkgs.mkShellNoCC rec {
	# Kanidm dependencies
	buildInputs = with pkgs; [
		pkg-config
		
		cargo
		rustc

		clang
		llvmPackages.bintools

    pnpm
    nodejs
		
		openssl
	];
	
	RUSTC_VERSION = overrides.toolchain.channel;
	# https://github.com/rust-lang/rust-bindgen#environment-variables
	LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
}