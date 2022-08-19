use std::fmt::Display;

pub enum SortOrder {
    Asc,
    Desc,
}

// Only contains Operations that are in use. Not exhaustive.
pub enum FilterOps {
    Gte,
    Lte,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Asc => "SortOrder::Asc",
            SortOrder::Desc => "SortOrder::Desc",
        }
    }
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FilterOps {
    pub fn as_str(&self) -> &'static str {
        match self {
            FilterOps::Gte => "FilterOps::Gte",
            FilterOps::Lte => "FilterOps::Lte",
        }
    }
}

impl Display for FilterOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
