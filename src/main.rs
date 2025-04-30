//! Display pokemon sprites in your terminal.

use std::io::{BufReader, Read};
use clap::Parser;
use pokeget::cli::Args;
use pokeget::list::List;
use pokeget::pokemon::{Attributes, Pokemon};
use pokeget::sprites::combine_sprites;
use std::process::exit;

fn main() {
    let list = List::read();
    let args = Args::parse();

    let attributes = Attributes::new(&args);

    let input_pokemon = if args.pokemon[0] == "-" {
        let stdin = std::io::stdin();
        let mut reader = BufReader::new(stdin.lock());
        let mut buf = String::new();

        if let Err(e) = reader.read_to_string(&mut buf) {
            eprintln!("Error reading from stdin: {}", e);
            exit(1);
        }
        buf.split_whitespace().map(|x| x.to_string()).collect()
    } else {
        args.pokemon
    };

    if input_pokemon.is_empty() {
        eprintln!("you must specify the pokemon you want to display");
        exit(1);
    }

    let pokemons: Vec<Pokemon> = input_pokemon
        .into_iter()
        .map(|x| Pokemon::new(x, &list, &attributes))
        .collect();

    let combined = combine_sprites(&pokemons);

    if !args.hide_name {
        let names: Vec<&str> = pokemons.iter().map(|x| x.name.as_ref()).collect();

        eprintln!("{}", names.join(", "));
    }

    println!("{}", showie::to_ascii(&combined));
}
