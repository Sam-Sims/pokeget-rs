pub use crate::random::RandomType;
use std::process::exit;

use image::DynamicImage;

use rand::Rng;

use crate::{cli::Args, list::List, Data};

const DEFAULT_SHINY_RATE: u32 = 8192;

/// Enum used to store regions
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Region {
    Kanto,
    Johto,
    Hoenn,
    Sinnoh,
    Unova,
    Kalos,
    Alola,
    Galar,
}

impl Region {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kanto" => Some(Region::Kanto),
            "johto" => Some(Region::Johto),
            "hoenn" => Some(Region::Hoenn),
            "sinnoh" => Some(Region::Sinnoh),
            "unova" => Some(Region::Unova),
            "kalos" => Some(Region::Kalos),
            "alola" => Some(Region::Alola),
            "galar" => Some(Region::Galar),
            _ => None,
        }
    }
}

/// Enum used to assist parsing user input.
///
/// It can sort all types of inputs, and then evaluate them to a filename.
#[derive(PartialEq, Eq)]
pub enum Selection {
    /// When a DexID is selected (number larger than 0).
    DexId(usize),

    /// When a pokemon name/id is selected.
    Name(String),

    /// When a region pokemon is selected.
    Region(Region),
}

impl Selection {
    /// Parses a raw argument into a [`Selection`].
    pub fn parse(arg: String) -> Self {
        if let Ok(dex_id) = arg.parse::<usize>() {
            match dex_id {
                // If it's zero, then change it to random.
                0 => Selection::DexId(0),

                // If it's not zero and in the range of the list, then it's a dex id.
                id if (id > 0) => Selection::DexId(id),

                // This shouldn't normally fire, but it's here to give the proper error message.
                _ => Selection::Name(arg),
            }
        } else {
            if let Some(region) = Region::from_str(&arg) {
                Selection::Region(region)
            } else {
                Selection::Name(arg)
            }
        }
    }

    /// Evaluates the selection and returns a pokemon filename.
    pub fn eval(self, list: &List) -> String {
        match self {
            Selection::DexId(0) => {
                let random_type = RandomType::Any;
                random_type.parse_random(list)
            }
            Selection::DexId(id) => list
                .get_by_id(id - 1)
                .unwrap_or_else(|| {
                    eprintln!("{} is not a valid pokedex ID", id);
                    exit(1)
                })
                .clone(),
            Selection::Name(name) => name,
            Selection::Region(region) => {
                let random_type = RandomType::Region(region);
                random_type.parse_random(list)
            }
        }
    }
}

/// The struct used to represent a Pokemon's data.
/// This includes it's file path, formatted name, sprite, and attributes.
pub struct Pokemon<'a> {
    /// The path of the Pokemon in pokesprite.
    /// Eg. `regular/abra.png`
    pub path: String,

    /// The formatted name of the pokemon, usually gotten from a [List].
    pub name: String,

    /// The sprite of the Pokemon, as a [DynamicImage].
    pub sprite: DynamicImage,

    /// Data, like the form and whether a pokemon is shiny or not.
    pub attributes: &'a Attributes,
}

impl<'a> Pokemon<'a> {
    /// Creates a new pokemon.
    /// This also fetches the sprite & formats the name.
    pub fn new(arg: String, list: &List, attributes: &'a Attributes) -> Self {
        let selection = Selection::parse(arg);
        let name = selection.eval(list);

        let path = attributes.path(&name);
        let bytes = Data::get(&path)
            .unwrap_or_else(|| {
                if Region::from_str(&name).is_some() {
                    eprintln!("{} is a region name. To get a random pokemon from this region, use `pokeget random {}`", name, name);
                } else {
                    eprintln!("pokemon not found");
                }
                exit(1)
            })
            .data
            .into_owned();

        let img = image::load_from_memory(&bytes).unwrap();
        let trimmed = showie::trim(&img);

        Self {
            path,
            name: list.format_name(&name),
            sprite: trimmed,
            attributes,
        }
    }

    pub fn new_from_random(
        random_type: &RandomType,
        list: &List,
        attributes: &'a Attributes,
    ) -> Self {
        let name = random_type.parse_random(list);

        let path = attributes.path(&name);
        let bytes = Data::get(&path)
            .unwrap_or_else(|| {
                eprintln!("pokemon not found");
                exit(1)
            })
            .data
            .into_owned();

        let img = image::load_from_memory(&bytes).unwrap();
        let trimmed = showie::trim(&img);

        Self {
            path,
            name: list.format_name(&name),
            sprite: trimmed,
            attributes,
        }
    }
}

/// Handles parsing the form, as well as whether a pokemon is female or shiny.
pub struct Attributes {
    pub form: String,
    pub female: bool,
    pub shiny: bool,
}

/// Pokemon attribues, like whether it's shiny, female, and it's form.
impl Attributes {
    /// Determine whether a pokemon should be shiny, based on a random rate (`DEFAULT_SHINY_RATE`).
    ///
    /// If the user specified that they want a shiny pokemon, then this function is irrelevant.
    fn rate_is_shiny() -> bool {
        let rate = match std::env::var("POKEGET_SHINY_RATE")
            .map_err(|_| false)
            .and_then(|x| x.parse::<u32>().map_err(|_| true))
        {
            Ok(rate) => rate.max(1), // No zero please
            Err(should_notify) => {
                if should_notify {
                    eprintln!("POKEGET_SHINY_RATE was improperly formatted, using default rate")
                }

                DEFAULT_SHINY_RATE
            }
        };

        0 == rand::thread_rng().gen_range(0..rate)
    }
    /// Make a new [`Attributes`] by parsing the command line arguments.
    pub fn new(args: &Args) -> Self {
        let mut form = match args {
            Args { mega: true, .. } => "mega",
            Args { mega_x: true, .. } => "mega-x",
            Args { mega_y: true, .. } => "mega-y",
            Args { alolan: true, .. } => "alola",
            Args { gmax: true, .. } => "gmax",
            Args { hisui: true, .. } => "hisui",
            Args { galar: true, .. } => "galar",
            _ => &args.form,
        }
        .to_string();

        if args.noble {
            form.push_str("-noble");
        }

        Self {
            form,
            female: args.female,
            shiny: args.shiny || Self::rate_is_shiny(),
        }
    }

    /// Formats the attributes and a filename from a [Selection] into a completed path.
    pub fn path(&self, name: &str) -> String {
        let mut filename = name.to_owned();

        // The form shouldn't be applied to random pokemon.
        if !self.form.is_empty() {
            filename.push_str(&format!("-{}", self.form));
        }

        // I hate Mr. Mime and Farfetch'd.
        filename = filename
            .replace([' ', '_'], "-")
            .replace(['.', '\'', ':'], "")
            .to_lowercase();

        let path = format!(
            "{}/{}{}.png",
            if self.shiny { "shiny" } else { "regular" },
            if self.female { "female/" } else { "" }, // Random pokemon also shouldn't follow the female rule.
            filename.trim()
        );

        path
    }
}
