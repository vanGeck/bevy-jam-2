# README

## Hotkeys

- `Escape` will back out of the game to the main menu. If you're already on the main menu, it will instantly close the
  game.
- `F11` will toggle between `BorderlessFullscreen` and `Windowed` mode.

## Config files

### Adding new config files

- Add it as a struct to `src/config/`.
- Add it to the `src/loading/systems/load_configs()` function to load as a resource.
- Add a ron file with default values to `assets/config/default`.

### Using the configs in systems

You can request it as a resource in systems: `config: Res<GridConfig>,`.

### Overriding configs

Every config file is present as a `ron` file in the `assets/config/default/` directory. If you need to change one of the
constants for yourself, for testing purposes, you can copy the relevant config file to `assets/config/override/` and
change the values to your wishes. Those files wil not be added to source control, so you're not messing with anyone
else's build.

Some suggestions on how to use this:

- Override the `audio.ron` file to turn down the music and sound effects.
- Override the `debug.ron` file to skip past the main menu when testing.
- Override the `log.ron` file to tweak to log filter.
