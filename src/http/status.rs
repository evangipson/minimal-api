pub enum Status {
    OK,
    NOTFOUND,
}

impl Status {
    pub const OK: Status = Status::OK;
    pub const NOTFOUND: Status = Status::NOTFOUND;
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Status::OK => "200 OK",
            Status::NOTFOUND => "404 NOT FOUND",
        };
        write!(f, "{status}")
    }
}
