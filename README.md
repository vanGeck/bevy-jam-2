# README

## Config files

Every config file is present as a `ron` file in the `assets/config/default/` directory. If you need to change one of the
constants for yourself, for testing purposes, you can copy the relevant config file to `assets/config/dev/` and change
the values to your wishes. Those files wil not be added to source control, so you're not fucking up anyone else's
build.

### Adding new config files

- Add it as a struct to `src/config/`.
- Add it to the ConfigLoader.
- Add a ron file with default values to `assets/config/default`.

### Using the configs

You can request it as a resource in systems: `config: Res<GridConfig>,`.

