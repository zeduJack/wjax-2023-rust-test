fn main() {
    println!("Hello, world!");
}


fn combine_names(name: &str, vorname: Option<&str>) -> String {
    match vorname {
        Some(s) => format!("{} {}", name, s),
        None => name.to_string()
    }
}


#[cfg(test)]
mod test {
    use crate::combine_names;

    #[test]
    fn test_combine_names() {
        assert_eq!("Dujmovic Zeljko", combine_names("Dujmovic", Some("Zeljko")));
        assert_eq!("Dujmovic", combine_names("Dujmovic", None));
    }
}
