with (import <nixpkgs> {});
mkShell {
    imports = [ ~/.config/nix/develop.nix ];

    buildInputs = [
        zig_0_12
    ];

    shellHook = ''
        printf "\tAhora a desarrollar en Zig 0.12\n" 
    '';
}
