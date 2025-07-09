use std::collections::HashMap;

/// [`Respond`] is a trait for formatting types to repond to web server requests easily.
pub trait Respond {
    /// [`Respond::get_json`] gets a JSON-style [`String`] representation of the type
    /// that calls it.
    /// # Example
    /// [`Respond::get_json`] can be used to return a JSON response for any implementations:
    /// ```rust
    /// use http::respond::Respond;
    ///
    /// fn return_json_response(message: impl Respond) -> String {
    ///     message.get_json()
    /// }
    /// ```
    fn get_json(&self) -> String;
}

/// Implement [`Respond`] for [`String`]
impl Respond for String {
    fn get_json(&self) -> String {
        "\"".to_string() + self + "\""
    }
}

/// Implement [`Respond`] for [`str`]
impl Respond for str {
    fn get_json(&self) -> String {
        "\"".to_string() + self + "\""
    }
}

/// Implement [`Respond`] for `static` [`str`]
impl Respond for &str {
    fn get_json(&self) -> String {
        "\"".to_string() + self + "\""
    }
}

/// Implement [`Respond`] for [`bool`]
impl Respond for bool {
    fn get_json(&self) -> String {
        self.to_string()
    }
}

/// Implement [`Respond`] for [`u8`]
impl Respond for u8 {
    fn get_json(&self) -> String {
        self.to_string()
    }
}

/// Implement [`Respond`] for [`Vec<String>`]
impl Respond for Vec<std::string::String> {
    fn get_json(&self) -> String {
        "[".to_string()
            + &self
                .into_iter()
                .map(|x| x.to_string() + ",")
                .collect::<String>()
                .strip_suffix(",")
                .unwrap_or("")
            + "]"
    }
}

/// Implement [`Respond`] for a dynamic [`HashMap`] containing a response of any type
impl Respond for HashMap<&str, Box<dyn Respond>> {
    fn get_json(&self) -> String {
        "{".to_string()
            + &self
                .into_iter()
                .map(|x| "\"".to_owned() + x.0 + "\":" + &x.1.get_json() + ",")
                .collect::<String>()
                .strip_suffix(",")
                .unwrap_or("") // handle empty maps gracefully
            + "}"
    }
}
