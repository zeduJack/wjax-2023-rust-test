use std::collections::btree_map::Entry;
use std::collections::HashMap;
use std::str::Split;

fn main() {
    println!("Hello, world!");
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

// todo: nicht nicht fertig. Die methode sollte eine Map zurückgeben wo jedes Wort der Key ist und
// eine Liste von Sätzen die das wort enthalten
fn suchmaschine(s: String) -> HashMap<String, String> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let saetze: Vec<&str> = s.split('.').collect();

    for satz in saetze {
        let worte: Vec<&str> = satz.trim().split(' ').collect();
        for wort in worte {
            match map.entry(wort.to_string()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(7);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![7]);
                }
            }
        }
    }
    map
}


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
