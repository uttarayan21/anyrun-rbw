## anyrun-rbw

Plugin for anyrun that adds support for bitwarden using rbw

To install compile with `cargo build` and paste the resulting `.so` file to any static place  
and then add the full path to it to `~/.config/anyrun/config.ron`'s plugin array

For nix users

Add it to your flake.nix as a dependency and then add it to the anyrun home-manager  module like so

```nix
  programs.anyrun = {
    enable = device.hasGui;
    config = {
      plugins = with inputs.anyrun.packages.${pkgs.system}; [
        inputs.anyrun-rbw.packages.${pkgs.system}.default
        inputs.anyrun-nixos-options.packages.${pkgs.system}.default
        inputs.anyrun-hyprwin.packages.${pkgs.system}.default
        inputs.anyrun-rink.packages.${pkgs.system}.default
        applications
        websearch
        shell
        translate
        symbols
        kidex
      ];
      x = {fraction = 0.5;};
      y = {fraction = 0.3;};
      height = {absolute = 0;};
      width = {absolute = 1000;};
      showResultsImmediately = true;
      maxEntries = 10;
      layer = "overlay";
    };
};
```
