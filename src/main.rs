#![allow(dead_code)]

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

const ERSTE: &str = "11111111111111111|Am Tiergarten";
const ZWEITE: &str = "21111111111111111|Am Tiergarten";

#[derive(Debug, PartialEq, Eq, Hash)]
struct Straße {
    id: String,
    name: String,
}

impl std::str::FromStr for Straße {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let teile: Vec<_> = s.split('|').collect();
        //check for 2 parts
        if teile.len() != 2 {
            return Err(format!(
                "Eintrag besteht aus {} Teilen, sollte aus 2 bestehen",
                teile.len()
            ));
        }
        let id = teile[0];
        let name = teile[1];
        //check id length
        if id.chars().count() != 17 {
            return Err(format!(
                "id hat Länge {}, sollte 17 lang sein, straßenname: {}",
                id.chars().count(),
                name
            ));
        }

        //create Straße
        Ok(Straße {
            id: id.to_string(),
            name: name.to_string(),
        })
    }
}

fn map_aus_datei(dateiname: &str) -> HashMap<Straße, bool> {
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
    map
}

fn datei_prüfen(map: &mut HashMap<Straße, bool>, dateiname: &str) {
    let file = File::open(dateiname).unwrap();
    let lines = io::BufReader::new(file).lines();

    println!("Straßen einlesen");
    for line in lines {
        let straße = Straße::from_str(line.unwrap().as_str());
        match straße {
            Ok(straße) => {
                if map.contains_key(&straße) {
                    map.insert(straße, true);
                } else {
                    println!(
                        "Straße existiert nicht. id: {}, name: {}",
                        straße.id, straße.name
                    );
                }
            }
            Err(fehler) => println!("Fehler beim Einlesen einer Straße: {fehler}"), //TODO hilfreiche informationen ausgeben
        };
    }
    println!("\nErgebnis");
    for (straße, gefunden) in map {
        if !*gefunden {
            println!("Straße fehlt. id: {}, name: {}", straße.id, straße.name)
        }
    }
}

fn main() {
    let mut map = map_aus_datei("moers.txt");
    datei_prüfen(&mut map, "osm.txt");
}
