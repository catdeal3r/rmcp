{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          clang
          pkg-config
          openssl
        ];

        # Tell Rust and pkg-config where the OpenSSL headers and libraries are
        OPENSSL_NO_VENDOR = 1; # Optional: forces cargo to use system OpenSSL instead of building its own

        shellHook = ''
          # Append OpenSSL and other package libraries to LD_LIBRARY_PATH
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [ pkgs.openssl ]}:$LD_LIBRARY_PATH"
        '';
      };
    };
}
