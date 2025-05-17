use std::collections::HashMap;

pub trait Respond {
    fn get_json(&self) -> String;
}

impl Respond for String {
    fn get_json(&self) -> String {
        "\"".to_string() + self + "\""
    }
}

impl<T: std::string::ToString, U: std::string::ToString> Respond for HashMap<T, U> {
    fn get_json(&self) -> String {
        "{".to_owned()
            + self
                .iter()
                .map(|(t, u)| "\"".to_string() + &t.to_string() + "\":\"" + &u.to_string() + "\", ")
                .collect::<String>()
                .trim_end_matches(",")
            + "}"
    }
}
