//! Display pokemon sprites in your terminal.

use clap::Parser;
use pokeget::cli::Args;
use pokeget::cli::Commands;
use pokeget::list::List;
use pokeget::pokemon::RandomType;
use pokeget::pokemon::{Attributes, Pokemon};
use pokeget::sprites::combine_sprites;
use std::io::{BufReader, Read};
use std::process::exit;

fn read_from_stdin() -> Vec<String> {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buf = String::new();

    if let Err(e) = reader.read_to_string(&mut buf) {
        eprintln!("Error reading from stdin: {}", e);
        exit(1);
    }

    buf.split_whitespace().map(|x| x.to_string()).collect()
}

fn main() {
    let list = List::read();
    let args = Args::parse();

    let attributes = Attributes::new(&args);

    let pokemons = match &args.command {
        // handle random subcommand
        Some(Commands::Random { pokemon }) => {
            // if - then read from stdin
            let pokemon_list = if pokemon.contains(&"-".to_string()) {
                read_from_stdin()
            } else {
                pokemon.clone()
            };
            // if pokemon list is empty - then get any random pokemon, if not - then get random from list
            let random_type = match pokemon_list.as_slice() {
                [] => RandomType::Any,
                _ => RandomType::List(pokemon_list),
            };

            vec![Pokemon::new_from_random(&random_type, &list, &attributes)]
        }
        // handle pokemon subcommand
        Some(Commands::Pokemon { pokemon }) => {
            // if - then read from stdin
            let pokemon_list = if pokemon.contains(&"-".to_string()) {
                read_from_stdin()
            } else {
                pokemon.clone()
            };
            pokemon_list
                .iter()
                .map(|x| Pokemon::new(x.to_string(), &list, &attributes))
                .collect()
        }
        // handle no subcommand
        None => {
            eprintln!("No pokemon specified. Use `pokeget random` to get a random pokemon.");
            exit(1);
        }
    };

    let combined = combine_sprites(&pokemons);

    if !args.hide_name {
        let names: Vec<&str> = pokemons.iter().map(|x| x.name.as_ref()).collect();

        eprintln!("{}", names.join(", "));
    }

    println!("{}", showie::to_ascii(&combined));
}
