#![feature(try_trait)]

use std::io::Read;
use std::iter::Peekable;

#[derive(Debug)]
enum Error {
    UnexpectedEof,
    UnexpectedChar(char, String),
}

impl From<std::option::NoneError> for Error {
    fn from(_e: std::option::NoneError) -> Error {
        Error::UnexpectedEof
    }
}

type Result<T> = std::result::Result<T, Error>;

struct Parser<'a> {
    stream: &'a mut Peekable<std::str::Chars<'a>>,
    group_level: u32,
    group_score: u32,
}

impl<'a> Parser<'a> {
    fn new(stream: &'a mut Peekable<std::str::Chars<'a>>) -> Parser<'a> {
        Parser {
            stream: stream,
            group_level: 0,
            group_score: 0,
        }
    }

    fn parse(&mut self) -> Result<()> {
        self.parse_group()?;
        Ok(())
    }

    fn parse_compound(&mut self) -> Result<()> {
        loop {
            match self.peek_char("{<")? {
                '{' => { self.parse_group()?; }
                '<' => { self.parse_garbage()?; }
                _ => { assert!(false); }
            }

            if self.accept_char(",").is_none() {
                return Ok(())
            }
        }
    }

    fn parse_group(&mut self) -> Result<()> {
        self.group_level += 1;
        self.group_score += self.group_level;

        self.expect_char("{")?;
        if self.peek_char("}").is_none() {
            self.parse_compound()?;
        }
        self.expect_char("}")?;

        self.group_level -= 1;
        Ok(())
    }

    fn parse_garbage(&mut self) -> Result<()> {
        self.expect_char("<")?;
        loop {
            match self.stream.next()? {
                '>' => { return Ok(()) }
                '!' => { self.stream.next()?; }
                _ => {}
            }
        }
    }

    fn expect_char(&mut self, expected_chars: &str) -> Result<char> {
        if let Some(ch) = self.expect_char_or_eof(expected_chars)? {
            Ok(ch)
        } else {
            Err(Error::UnexpectedEof)
        }
    }

    fn expect_char_or_eof(&mut self, expected_chars: &str) -> Result<Option<char>> {
        let ch = self.stream.next()?;
        if expected_chars.contains(ch) {
            Ok(Some(ch))
        } else {
            Err(Error::UnexpectedChar(ch, String::from(expected_chars)))
        }
    }

    fn accept_char(&mut self, accepted_chars: &str) -> Option<char> {
        let ch = self.peek_char(accepted_chars);
        if ch.is_some() {
            self.stream.next();
        }
        ch
    }

    fn peek_char(&mut self, accepted_chars: &str) -> Option<char> {
        if let Some(ch) = self.stream.peek().map(|ch| *ch) {
            if accepted_chars.contains(ch) {
                return Some(ch);
            }
        }
        None
    }
}

fn count_groups(stream: &String) -> Result<u32> {
    let chars = stream.chars();
    let mut peekable = chars.peekable();
    let mut parser = Parser::new(&mut peekable);
    parser.parse()?;
    Ok(parser.group_score)
}

fn main() {
    let mut stream = String::new();
    std::io::stdin().read_to_string(&mut stream).expect("input error");
    let num_groups = count_groups(&stream).expect("parse error");
    println!("{}", num_groups);
}
