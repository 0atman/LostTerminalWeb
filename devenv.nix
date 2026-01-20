{ pkgs, lib, config, ... }:

{
  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  packages = with pkgs; [
    openssl
  ];

  scripts.watcher= {
    exec = ''
      RUSTFLAGS=-Awarnings watchexec -r --clear=reset -e rs --wrap-process=none "cargo run -q"
    '';
    packages = [ pkgs.watchexec ];
    description = "Rebuilds and runs app with supressed warnings";
  };

  # enterShell = ''
  #   echo
  #   echo 🦾 Helper scripts you can run to make your development richer:
  #   echo 🦾
  #   ${pkgs.gnused}/bin/sed -e 's| |••|g' -e 's|=| |' <<EOF | ${pkgs.util-linuxMinimal}/bin/column -t | ${pkgs.gnused}/bin/sed -e 's|^|🦾 |' -e 's|••| |g'
  #   ${lib.generators.toKeyValue {} (lib.mapAttrs (name: value: value.description) config.scripts)}
  #   EOF
  #   echo
  # '';

  # env.LD_LIBRARY_PATH = lib.makeLibraryPath [
  #   pkgs.zlib
  #   pkgs.libcxx.out
  #   pkgs.llvmPackages.libunwind.out
  # ];

}
