#![feature(iter_intersperse)]

use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Straße {
    id: String,
    name: String,
}

impl std::str::FromStr for Straße {
    type Err = StraßeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let teile: Vec<_> = s.split('|').collect();
        //match on parts len
        match teile.len() {
            0 => Err(StraßeParseError::Leer),
            1 => Err(StraßeParseError::KeinName {
                id: teile[0].to_string(),
            }),
            2 => {
                let id = teile[0].to_string();
                let name = teile[1].to_string();
                //check id length
                if id.chars().count() != 17 {
                    return Err(StraßeParseError::FalscheIdLänge { id, name });
                }

                //create Straße
                Ok(Straße { id, name })
            }
            _ => {
                let mut teile = teile.into_iter();
                Err(StraßeParseError::ZuVieleTeile {
                    id: teile.next().unwrap().to_string(),
                    name: teile.next().unwrap().to_string(),
                    rest: teile.intersperse("|").collect(),
                })
            }
        }
    }
}

#[derive(Error, Debug)]
enum StraßeParseError {
    #[error("str ist leer")]
    Leer,
    #[error("Straßenname fehlt; id: {id}")]
    KeinName { id: String },
    #[error("Id hat falsche Länge (ist {}, erwarte 17); id: {id}, name: {name}", id.len())]
    FalscheIdLänge { id: String, name: String },
    #[error("Straße hat zu viele Teile, erwarte 2; id: {id}, name: {name}, rest: {rest}")]
    ZuVieleTeile {
        id: String,
        name: String,
        rest: String,
    },
}

fn map_aus_datei(dateiname: &str) -> HashMap<Straße, bool> {
    print!("Erstelle map aus {dateiname}...");

    let file = File::open(dateiname).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut map = HashMap::new();
    for line in lines {
        let straße = Straße::from_str(line.unwrap().as_str());
        match straße {
            Ok(straße) => map.insert(straße, false),
            Err(fehler) => panic!("Straße hat falsches Format:\n{fehler}"),
        };
    }
    println!("   Fertig!");
    map
}

fn datei_prüfen(map: &mut HashMap<Straße, bool>, dateiname: &str) {
    println!("Prüfe {dateiname}");
    let file = File::open(dateiname).unwrap();
    let lines = io::BufReader::new(file).lines();

    println!("Straßen einlesen");
    for line in lines {
        let straße = Straße::from_str(line.unwrap().as_str());
        match straße {
            Ok(straße) => match map.entry(straße) {
                Entry::Occupied(mut e) => {
                    e.insert(true);
                }
                Entry::Vacant(e) => {
                    let straße = e.into_key();
                    println!(
                        "Straße existiert nicht; id: {}, name: {}",
                        straße.id, straße.name
                    );
                }
            },
            Err(fehler) => println!("Fehler beim Einlesen einer Straße: {fehler}"),
        };
    }
    println!("Fertig!");
    println!("Ergebnis");
    for (straße, gefunden) in map {
        if !*gefunden {
            println!("Straße fehlt; id: {}, name: {}", straße.id, straße.name)
        }
    }
    println!("Fertig!");
}

fn main() {
    let mut map = map_aus_datei("moers.txt");
    datei_prüfen(&mut map, "osm.txt");
}
