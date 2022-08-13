use std::fmt::Display;

// Only contains Operations that are in use. Not exhaustive.
pub enum FilterOps {
    Gte,
    Lte,
}

impl FilterOps {
    pub fn as_str(&self) -> &'static str {
        match self {
            FilterOps::Gte => "$gte",
            FilterOps::Lte => "$lte",
        }
    }
}

impl Display for FilterOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
