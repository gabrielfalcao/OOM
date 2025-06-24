use crate::Span;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position { line, column }
    }

    pub fn from_tuple(line_col: (usize, usize)) -> Position {
        let (line, column) = line_col;
        Position { line, column }
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    pub fn span_to(&self, input: &str) -> Span {
        let lines = input.lines().map(String::from).collect::<Vec<String>>();
        let end = Position::new(
            self.line + lines.len(),
            self.column + lines.iter().map(|line| line.len()).sum::<usize>(),
        );
        Span::new(input.to_string(), self.to_tuple(), end.to_tuple())
    }
}

impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        self.to_tuple()
    }
}
