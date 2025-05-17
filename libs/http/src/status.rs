#[derive(Clone)]
pub enum Status {
    Ok,
    BadRequest,
    NotFound,
    UnprocessableEntity,
    ServerError,
}

impl Status {
    pub const OK: Status = Status::Ok;
    pub const BAD_REQUEST: Status = Status::BadRequest;
    pub const NOT_FOUND: Status = Status::NotFound;
    pub const UNPROCESSABLE_ENTITY: Status = Status::UnprocessableEntity;
    pub const SERVER_ERROR: Status = Status::ServerError;
}

impl std::fmt::Display for Status {
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
