use crate::list::List;
use crate::pokemon::{Region, Selection};
use rand::Rng;

pub enum RandomType {
    Any,
    Region(Region),
    List(Vec<String>),
}
impl RandomType {
    pub fn parse_random(&self, list: &List) -> String {
        match self {
            RandomType::Any => {
                let mut rand = rand::thread_rng();
                let idx = rand.gen_range(0..list.ids.len());
                list.ids.get_by_left(&idx).unwrap().clone()
            }

            RandomType::Region(region) => {
                let mut rand = rand::thread_rng();
                let range = match region {
                    Region::Kanto => 0..=151,
                    Region::Johto => 152..=251,
                    Region::Hoenn => 252..=386,
                    Region::Sinnoh => 387..=493,
                    Region::Unova => 494..=649,
                    Region::Kalos => 650..=721,
                    Region::Alola => 722..=809,
                    Region::Galar => 810..=905,
                };
                let idx = rand.gen_range(range);
                list.ids.get_by_left(&idx).unwrap().clone()
            }

            RandomType::List(options) => {
                // lists can contain regions, so needs some extra parsing
                let mut rng = rand::thread_rng();
                let idx = rng.gen_range(0..options.len());

                let selection = Selection::parse(options[idx].clone());

                match selection {
                    Selection::Region(region) => RandomType::Region(region).parse_random(list),
                    _ => selection.eval(list),
                }
            }
        }
    }
}
