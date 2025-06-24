use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Source<'c> {
    pub source: Cow<'c, str>,
    pub filename: Option<Cow<'c, str>>,
}
impl<'c> Source<'c> {
    pub fn new(source: &'c str, filename: Option<&'c str>) -> Source<'c> {
        Source {
            source: Cow::from(source),
            filename: filename.map(|filename| Cow::from(filename)),
        }
    }

    pub fn without_filename<T: std::fmt::Display>(source: T) -> Source<'c> {
        Source {
            source: Cow::from(source.to_string()),
            filename: None,
        }
    }

    pub fn filename(&self) -> Option<String> {
        self.filename.clone().map(String::from)
    }
}

impl<'c> From<&'c str> for Source<'c> {
    fn from(source: &'c str) -> Source<'c> {
        Source::without_filename(source)
    }
}

impl<'c> From<String> for Source<'c> {
    fn from(source: String) -> Source<'c> {
        Source::without_filename(source)
    }
}
