# pokeget-rs
Display Pokémon sprites in your terminal.

A fork of [pokeget-rs](https://github.com/talwat/pokeget-rs) that adds some extra functionality, primarily for
streamlining the experience when using it in fastfetch.

Adds the following features to the original pokeget-rs:

- Supports reading stdin as input.
- Extends pokeget into 2 subcommands: `pokeget pokemon` and `pokeget random`.
- `pokeget pokemon` will show the sprite of the given pokemon, as per default pokeget.
- `pokeget random` will show a random pokemon sprite, with the ability specify a region, or provide a custom list of
  favorites to pick from.

## Basic Usage

### Display Specific Pokémon

```
pokeget pokemon <pokemon>
```

Show the sprite of specified Pokémon. You can use either names or Pokédex IDs.

Example: `pokeget pokemon pikachu bulbasaur 150`  
*Displays sprites for Pikachu, Bulbasaur, and Mewtwo together*

### Display Random Pokémon

```
pokeget random [options]
```
Options can be:

- Nothing (blank): Random Pokémon from entire Pokédex
- Region names: `kanto`, `johto`, `hoenn`, `sinnoh`, `unova`, `kalos`, `alola`, `galar`
- Specific Pokémon names or IDs
- Custom lists defined in your config

When using `pokeget random`, all inputs are combined into a single pool from which one Pokémon is
randomly selected. This allows you to mix:

- Individual Pokémon names/IDs
- Entire regions
- Custom lists from your config

Example: `pokeget random kanto bulbasaur charizard favorites`
*Will randomly select ONE from:*

- Any Kanto Pokémon (151 possibilities)
- Bulbasaur
- Charizard
- Any Pokémon defined in your "favorites" list

This makes it easy to create custom random pools with exactly the Pokémon you want.

## Custom Lists

You can create your own Pokémon lists by adding them to the config file:

File location: `~/.config/pokeget/config.toml`

Example config:

```toml
[starters]
pokemon = ["1", "4", "7", "152", "155", "158"]

[legendaries]
pokemon = ["144", "145", "146", "150", "151"]

[favorites]
pokemon = ["25", "149", "sinnoh"]
```

Using your custom lists:

- `pokeget pokemon starters` - Display all Pokémon in your "starters" list
- `pokeget random favorites` - Display one random Pokémon from your "favorites" list
- `pokeget pokemon starters legendaries` - Display all Pokémon from both lists
- `pokeget random starters kanto 25` - Choose randomly from starters list, any Kanto Pokémon, or Pikachu

## Stdin Support

Pokeget supports reading input from stdin using the `-` argument:

```bash
# Pipe Pokémon names to pokeget
echo "pikachu charizard" | pokeget pokemon -

# Pipe a list of Pokémon for random selection
cat my_pokemon_list.txt | pokeget random -
```

The stdin input is treated the same as command-line arguments, so it can include Pokémon names, IDs, region names, or
custom list names.

## Installation

### Manual

You can clone the repository and compile manually by doing:

```sh
git clone --recurse-submodules https://github.com/sam-sims/pokeget-rs.git
cd pokeget-rs
cargo build --release
mv target/release/pokeget ~/.local/bin
```

and making sure `$HOME/.local/bin` is added to `$PATH`.

## Credits

The original pokeget-rs was made by [talwat](https://github.com/talwat/pokeget-rs)

The sprites are from [pokesprite](https://github.com/msikma/pokesprite) and pokeget uses them with a git
submodule.

Sprites are embedded into the binary, so pokeget won't download them.

## License

pokeget uses the MIT license, so feel free to fork it and customize it as you please.
If you're unsure about any of the internal workings of
pokeget, [open an issue](https://github.com/talwat/pokeget-rs/issues),
and I'll answer whatever question you might have.
