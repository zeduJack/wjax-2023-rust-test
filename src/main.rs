use std::collections::btree_map::Entry;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Bytes, Read, Seek};
use std::str::Split;

fn main() -> std::io::Result<()>  {
    println!("Hello, world!");
    wc("./.gitignore").expect("TODO: panic message");
    file_lines("./.gitignore");
    file_bytes("./.gitignore")
}


fn combine_names(name: &str, vorname: Option<&str>) -> String {
    match vorname {
        Some(s) => format!("{} {}", name, s),
        None => name.to_string()
    }
}

fn create_string_map(s: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let saetze: Vec<&str> = s.split('.').collect();

    for satz in saetze {
        let worte: Vec<&str> = satz.trim().split(' ').collect();
        for wort in worte {
            map.insert(wort.to_string(), satz.to_string());
        }
    }
    map
}

// Übung 3: Baue einen command wc der die anzal bytes & anzal Zeilen ermittelt
fn wc(path: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;

    let mut line_count = 0;
    let mut byte_count = 0;


    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        byte_count += line.len();
        line_count += 1;
    }

    println!("Lines: {}", line_count);
    println!("Bytes: {}", byte_count);
    Ok(())
}

fn file_lines(path: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;

    let reader = BufReader::new(file);
    let count = reader.lines().count();

    println!("File lines: {}", count);
    Ok(())
}

fn file_bytes(path: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;

    let mut byte_count = 0;

    let reader = BufReader::new(file);

    let byte_count = reader.bytes().count();
    println!("File bytes: {}", byte_count);
    Ok(())
}

// // todo: nicht nicht fertig. Die methode sollte eine Map zurückgeben wo jedes Wort der Key ist und
// // eine Liste von Sätzen die das wort enthalten
// fn suchmaschine(s: String) -> HashMap<String, String> {
//     let mut map: HashMap<String, Vec<String>> = HashMap::new();
//     let saetze: Vec<&str> = s.split('.').collect();
//
//     for satz in saetze {
//         let worte: Vec<&str> = satz.trim().split(' ').collect();
//         for wort in worte {
//             match map.entry(wort.to_string()) {
//                 Entry::Occupied(mut e) => {
//                     e.get_mut().push(7);
//                 }
//                 Entry::Vacant(e) => {
//                     e.insert(vec![7]);
//                 }
//             }
//         }
//     }
//     map
// }


#[cfg(test)]
mod test {
    use crate::{combine_names, create_string_map};

    #[test]
    fn test_combine_names() {
        assert_eq!("Dujmovic Zeljko", combine_names("Dujmovic", Some("Zeljko")));
        assert_eq!("Dujmovic", combine_names("Dujmovic", None));
    }

    #[test]
    fn test_create_string_map() {
        create_string_map("Heute ist Montag. Morgen ist Dienstag".to_string());
    }
}
