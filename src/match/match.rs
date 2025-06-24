use crate::{Matcher, Position, Production, Span, StackRange, State};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub matcher: Production,
    pub span: Span,
    pub inner: Vec<Match>,
}
impl Matcher for Match {
    fn name(&self) -> &str {
        self.matcher.name()
    }

    fn to_str(&self) -> String {
        self.matcher.to_str()
    }

    fn as_production(&self) -> Production {
        self.matcher.as_production()
    }

    fn is_match(&self, state: &mut State, input: &str, start: &Position) -> Option<Match> {
        self.matcher.is_match(state, input, start)
    }
}
impl Match {
    pub fn to_tuple(&self) -> (Production, Span) {
        (self.matcher.clone(), self.span.clone())
    }

    pub fn matcher(&self) -> Production {
        self.matcher.clone()
    }

    pub fn inner(&self) -> Vec<Match> {
        self.inner.clone()
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn with_inner(&self, matches: Vec<Match>) -> Match {
        let mut r#match = self.clone();
        r#match.inner.extend(matches);
        r#match
    }
}
impl<T: Matcher> From<(T, Span)> for Match {
    fn from(ms: (T, Span)) -> Match {
        let (matcher, span) = ms;
        Match {
            matcher: matcher.as_production(),
            span,
            inner: Vec::new(),
        }
    }
}

impl<T: Matcher> From<&(T, Span)> for Match {
    fn from(ms: &(T, Span)) -> Match {
        let (matcher, span) = ms;
        Match {
            matcher: (*matcher).as_production(),
            span: (*span).clone(),
            inner: Vec::new(),
        }
    }
}

impl Into<(Production, Span)> for Match {
    fn into(self) -> (Production, Span) {
        self.to_tuple()
    }
}

impl Into<Option<(Production, Span)>> for Match {
    fn into(self) -> Option<(Production, Span)> {
        Some((self.matcher.as_production(), self.span.clone()))
    }
}

impl Into<Production> for Match {
    fn into(self) -> Production {
        self.matcher.clone()
    }
}

impl Into<Span> for Match {
    fn into(self) -> Span {
        self.span.clone()
    }
}

impl Into<((usize, usize), (usize, usize))> for Match {
    fn into(self) -> ((usize, usize), (usize, usize)) {
        (self.span.start.to_tuple(), self.span.end.to_tuple())
    }
}

impl Into<String> for Match {
    fn into(self) -> String {
        self.span.to_string()
    }
}
