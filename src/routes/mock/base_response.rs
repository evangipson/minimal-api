use http::respond::Respond;
use std::collections::HashMap;

/// [`BaseMockResponse`] represents a response to a mock application.
pub struct BaseMockResponse {
    pub is_success: bool,
    pub message_type: String,
    pub message_severity: u8,
    pub message_severity_dictionary: Vec<String>,
    pub message_code: u8,
    pub message_code_dictionary: Vec<String>,
    pub message_exception: String,
    pub message_custom_values_piped: String,
    pub reason_code: u8,
}

/// [`DEFAULT_RESPONSE`] is a `const` [`BaseMockResponse`] instantiation
/// filled with some default values to send back to an application.
pub const DEFAULT_RESPONSE: BaseMockResponse = BaseMockResponse {
    is_success: true,
    message_type: String::new(),
    message_severity: 0,
    message_severity_dictionary: vec![],
    message_code: 0,
    message_code_dictionary: vec![],
    message_exception: String::new(),
    message_custom_values_piped: String::new(),
    reason_code: 0,
};

impl BaseMockResponse {
    /// [`BaseMockResponse::get_default_response`] returns a [`HashMap`] with [`str`]
    /// keys and values that implement the [`Respond`] trait.
    ///
    /// # Example
    /// [`BaseMockResponse::get_default_response`] can be used to build a response
    /// containing many types of values to send back to an application:
    /// ```rust
    /// use http::respond::Respond;
    /// use minimal_api::routes::mock::base_response::BaseMockResponse;
    /// use std::collections::HashMap;
    ///
    /// fn get_application_response() -> HashMap<&'static str, Box<dyn Respond>> {
    ///     BaseMockResponse::get_default_response()
    /// }
    /// ```
    pub fn get_default_response() -> HashMap<&'static str, Box<dyn Respond>> {
        HashMap::from([
            (
                "IsSuccess",
                Box::new(DEFAULT_RESPONSE.is_success) as Box<dyn Respond>,
            ),
            (
                "MessageType",
                Box::new(DEFAULT_RESPONSE.message_type) as Box<dyn Respond>,
            ),
            (
                "MessageSeverity",
                Box::new(DEFAULT_RESPONSE.message_severity) as Box<dyn Respond>,
            ),
            (
                "MessageSeverityDictionary",
                Box::new(DEFAULT_RESPONSE.message_severity_dictionary) as Box<dyn Respond>,
            ),
            (
                "MessageCode",
                Box::new(DEFAULT_RESPONSE.message_code) as Box<dyn Respond>,
            ),
            (
                "MessageCodeDictionary",
                Box::new(DEFAULT_RESPONSE.message_code_dictionary) as Box<dyn Respond>,
            ),
            (
                "MessageException",
                Box::new(DEFAULT_RESPONSE.message_exception) as Box<dyn Respond>,
            ),
            (
                "MessageCustomValuesPiped",
                Box::new(DEFAULT_RESPONSE.message_custom_values_piped) as Box<dyn Respond>,
            ),
            (
                "ReasonCode",
                Box::new(DEFAULT_RESPONSE.reason_code) as Box<dyn Respond>,
            ),
        ])
    }
}
