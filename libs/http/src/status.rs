/// [`Status`] represents an HTTP response message status code.
#[derive(Clone)]
pub enum Status {
    /// [`Status::Ok`] represents a `200 OK` HTTP response status code.
    Ok,
    /// [`Status::BadRequest`] represents a `400 BAD REQUEST` HTTP response
    /// status code.
    BadRequest,
    /// [`Status::NotFound`] represents a `404 NOT FOUND` HTTP response
    /// status code.
    NotFound,
    /// [`Status::UnprocessableEntity`] represents a `422 UNPROCESSABLE ENTITY`
    /// HTTP response status code.
    UnprocessableEntity,
    /// [`Status::ServerError`] represents a `500 INTERNAL SERVER ERROR` HTTP
    /// response status code.
    ServerError,
}

/// Implement [`std::fmt::Display`] for [`Status`].
impl std::fmt::Display for Status {
    /// [`Status::fmt`] will [`write!`] a [`String`] representation of the
    /// [`Status`] that invokes it.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Status::Ok => "200 OK",
            Status::BadRequest => "400 BAD REQUEST",
            Status::NotFound => "404 NOT FOUND",
            Status::UnprocessableEntity => "422 UNPROCESSABLE ENTITY",
            Status::ServerError => "500 INTERNAL SERVER ERROR",
        };
        write!(f, "{status}")
    }
}
