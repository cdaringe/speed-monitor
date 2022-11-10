#[derive(Debug)]
pub enum FastCliErr {
    General(String),
    ParseOutput(String),
}

impl std::fmt::Display for FastCliErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FastCliErr::General(s) => write!(f, "fast-cli failure: {}", s),
            FastCliErr::ParseOutput(s) => write!(f, "fast-cli output/parse error: {}", s),
        }
    }
}

#[derive(Debug)]
pub enum SMErr {
    DbError,
    FastCliErr(FastCliErr),
}

impl<T> From<SMErr> for Result<T, SMErr> {
    fn from(e: SMErr) -> Self {
        Err(e)
    }
}

impl From<FastCliErr> for SMErr {
    fn from(e: FastCliErr) -> Self {
        SMErr::FastCliErr(e)
    }
}

impl std::fmt::Display for SMErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SMErr::DbError => write!(f, "db error"),
            SMErr::FastCliErr(e) => write!(f, "{}", e),
        }
    }
}
