use std::fmt::Display;

use tracing::instrument;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    name: String,
}

impl Atom {
    #[instrument]
    pub fn new(name: String) -> Self {
        return Self { name };
    }
}

impl<S: Into<String> + Sized> From<S> for Atom {
    fn from(value: S) -> Self {
        Atom::new(value.into())
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq<str> for Atom {
    fn eq(&self, other: &str) -> bool {
        self.name.eq(other)
    }
}
