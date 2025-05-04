# pokeget-rs

A fork of [pokeget-rs](https://github.com/talwat/pokeget-rs) that adds some extra functionality, primarily for
streamlining
the experience when using it in programmes such as fastfetch.

Adds the following features:

- Supports reading stdin as input.
- Extends pokeget into 2 subcommands: `pokeget pokemon` and `pokeget random`.
- `pokeget pokemon` will show the sprite of the given pokemon, as per default pokeget.
- `pokeget random` will show a random pokemon sprite, with the ability specify a region, or provide a custom list of
  favorites to pick from.

# Pokeget

Display Pokémon sprites in your terminal.

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

Choose one random Pokémon from your input options.

Options can be:

- Nothing (blank): Random Pokémon from entire Pokédex
- Region names: `kanto`, `johto`, `hoenn`, `sinnoh`, `unova`, `kalos`, `alola`, `galar`
- Specific Pokémon names or IDs
- Custom lists defined in your config

**Important:** When using `pokeget random`, all inputs are combined into a single pool from which one Pokémon is
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

## Additional Options

For more customization options, run:

```
pokeget --help
```

## .bashrc

If you're using pokeget on shell startup, such as in `.bashrc`,
then instead of running `pokeget <pokemon>`, you can write the output
to a file by doing: `pokeget <pokemon> > file.txt`
and then have something like `cat file.txt` in your bashrc.

This makes your shell initialization practically instant, but obviously
won't work with random pokemon. pokeget is already fairly fast,
so using it on shell initialization is also not a very large bottleneck.

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

### Adding a directory to $PATH

#### Bash & Zsh

Append this to your `.bashrc` or `.zshrc`:

```sh
export PATH="<path>:$PATH"
```

#### Fish

Run this in your CLI:

```sh
fish_add_path <path>
```

## Updating

Just rerun `git pull` on the repository and then recompile.

## Why?

Because the first pokeget was slow, bloated, and super complicated, so I decided to make a better version in rust.

Now, instead of precomputing all the sprites and uploading them to a repo, pokeget will
be able to compute them on-demand which makes everything much more flexible.
Rust enables that computation to be done much more quickly than something like python.

It will also draw the sprites 2x smaller by using half squares.

## What about other projects?

pokeget-rs has an edge over projects like the old pokeget, pokeshell, etc... since it's in rust.

It also is significantly (5.5x) faster than krabby which is another very similar project.

For more info, go to [OTHER_PROJECTS.md](OTHER_PROJECTS.md).

## What about big sprites?

Gone. Reduced to atoms.

In all seriousness, I've just decided to not deal with them since it's significantly
extra work that I don't want to deal with. They were rarely used, and looked ugly
in small terminal windows, so there was little use in keeping them.

## Credits

The original pokeget-rs was made by [talwat](https://github.com/talwat/pokeget-rs)

This time, the sprites are from [pokesprite](https://github.com/msikma/pokesprite) and pokeget uses them with a git
submodule.

Sprites are embedded into the binary, so pokeget won't download them. This is a good compromise,
since while the binary may be large, pokeget can execute almost instantly and while offline.

## License

pokeget uses the MIT license, so feel free to fork it and customize it as you please.
If you're unsure about any of the internal workings of
pokeget, [open an issue](https://github.com/talwat/pokeget-rs/issues),
and I'll answer whatever question you might have.
