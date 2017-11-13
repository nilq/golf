use super::*;

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub pos: TokenPosition,
    pub index: usize,
}

impl Snapshot {
    pub fn new(index: usize, pos: TokenPosition) -> Snapshot {
        Snapshot {
            index,
            pos,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tokenizer {
    pub pos:   TokenPosition,
    index:     usize,
    items:     Vec<char>,
    snapshots: Vec<Snapshot>,
}

impl Iterator for Tokenizer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.read().cloned()
    }
}

#[allow(dead_code)]
impl Tokenizer {
    pub fn new(items: &mut Iterator<Item = char>) -> Tokenizer {
        Tokenizer {
            index:     0,
            pos:       TokenPosition::default(),
            items:     items.collect(),
            snapshots: Vec::new(),
        }
    }

    pub fn end(&self) -> bool {
        self.end_n(0)
    }

    pub fn end_n(&self, lookahead: usize) -> bool {
        self.index + lookahead >= self.items.len()
    }

    pub fn peek(&self) -> Option<&char> {
        self.peek_n(0)
    }

    pub fn peek_n(&self, n: usize) -> Option<&char> {
        if self.end_n(n) {
            return None
        }
        Some(&self.items[self.index + n])
    }

    pub fn read(&mut self) -> Option<&char> {
        if self.end() {
            return None
        }
        self.advance(1);
        Some(&self.items[self.index - 1])
    }

    pub fn advance(&mut self, a: usize) {
        if self.index + a <= self.items.len() {
            for item in &self.items[self.index .. self.index + a] {
                match *item {
                    '\n' => {
                        self.pos.line += 1;
                        self.pos.col = 0;
                    }
                    _ => self.pos.col += 1
                }
            }
            self.index += a
        }
    }

    pub fn take_snapshot(&mut self) {
        self.snapshots.push(Snapshot::new(self.index, self.pos));
    }

    pub fn peek_snapshot(&self) -> Option<&Snapshot> {
        self.snapshots.last()
    }

    pub fn rollback_snapshot(&mut self) {
        let snapshot = self.snapshots.pop().unwrap();
        self.index = snapshot.index;
        self.pos = snapshot.pos;
    }

    pub fn commit_snapshot(&mut self) {
        self.snapshots.pop();
    }

    pub fn last_position(&self) -> TokenPosition {
        self.peek_snapshot().unwrap().pos
    }

    pub fn try_match_token(&mut self, matcher: &Matcher) -> Option<Token> {
        if self.end() {
            return Some(Token::new(TokenType::EOF,
                                   TokenPosition::new(self.index, self.index),
                                   String::new()));
        }

        self.take_snapshot();
        match matcher.try_match(self) {
            Some(t) => {
                self.commit_snapshot();
                Some(t)
            }

            None => {
                self.rollback_snapshot();
                None
            }
        }
    }

    pub fn index(&self) -> &usize {
        &self.index
    }
}
