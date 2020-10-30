use rand::Rng;
use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use serenity::static_assertions::_core::fmt::Formatter;

#[derive(Debug)]
enum Drop {
    Highest(usize),
    Lowest(usize),
}

impl Display for Drop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", match self {
            Drop::Highest(_) => '+',
            Drop::Lowest(_) => '-'
        }, match self {
            Drop::Highest(x) => x,
            Drop::Lowest(x) => x
        })
    }
}

#[derive(Debug)]
struct Dice {
    count: u32,
    max: u32,
    drop: Option<Drop>,
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.count > 1 {
            write!(f, "{}", self.count)?;
        }
        write!(f, "d{}", self.max)?;
        if let Some(drop) = &self.drop {
            write!(f, "{}", drop)?
        }
        Ok(())
    }
}

impl Dice {
    fn generate(&self) -> Vec<u32> {
        let mut v = vec![];
        for _ in 0..self.count {
            v.push(rand::thread_rng().gen_range(1, self.count + 1))
        }
        v.sort();
        if let Some(drop) = &self.drop {
            match drop {
                Drop::Highest(value) => v.drain(v.len() - value..),
                Drop::Lowest(value) => v.drain(0..*value)
            };
        }
        v
    }
}

enum Bonus {
    Positive(u32),
    Negative(u32),
}

enum Component {
    Dice(Dice),
    Bonus(Bonus),
}

type Args = Vec<Component>;

#[derive(Clone, Copy, Debug)]
struct Cursor<'a> {
    value: &'a str,
    index: usize,
}

impl<'a> Cursor<'a> {
    fn current(&self) -> char {
        self.value.bytes().nth(self.index).unwrap() as char;
        self.value.as_bytes()[self.index] as char;
        self.value.chars().nth(self.index).unwrap()
    }
    fn next(&self) -> Result<(Self, char), usize> {
        if self.index < self.value.len() {
            Ok((Cursor { value: self.value, index: self.index + 1 }, self.current()))
        } else {
            Err(self.index)
        }
    }
    fn expect(&self, ch: char) -> Result<Self, usize> {
        let (next, nch) = self.next()?;
        if nch != ch {
            Err(self.index)
        } else {
            Ok(next)
        }
    }
    fn flush_whitespace(&mut self) {
        while self.index < self.value.len() && self.current().is_whitespace() {
            self.index += 1
        }
    }
}

trait Parse: Sized {
    fn parse(cursor: Cursor) -> Result<(Cursor, Self), usize>;
}

impl Parse for u32 {
    fn parse(cursor: Cursor) -> Result<(Cursor, Self), usize> {
        fn parse_digit(value: char) -> Option<u32> {
            match value {
                '0' => Some(0),
                '1' => Some(1),
                '2' => Some(2),
                '3' => Some(3),
                '4' => Some(4),
                '5' => Some(5),
                '6' => Some(6),
                '7' => Some(7),
                '8' => Some(8),
                '9' => Some(9),
                _ => None
            }
        }
        let (mut cursor, ch) = cursor.next()?;
        let mut value = parse_digit(ch).filter(|c| *c > 0).ok_or(cursor.index)?; // TODO: should this return err if the first character is zero?
        loop {
            let next = cursor.next();
            if let Ok((next_cursor, ch)) = next {
                if let Some(digit) = parse_digit(ch) {
                    value = value * 10 + digit;
                    cursor = next_cursor;
                    continue;
                }
            }
            return if value > 0 {
                Ok((cursor, value))
            } else {
                Err(cursor.index)
            };
        }
    }
}

impl Parse for Dice {
    fn parse(mut cursor: Cursor) -> Result<(Cursor, Self), usize> {
        let mut count = 1;
        if let Ok((next_cursor, actual_count)) = u32::parse(cursor) {
            cursor = next_cursor;
            count = actual_count;
        }
        cursor = cursor.expect('d')?;
        let (next_cursor, max) = u32::parse(cursor)?;
        cursor = next_cursor;
        let drop = if let Ok((next_cursor, next_ch)) = cursor.next() {
            if next_ch == 'd' {
                let (modifier_cursor, modifier) = next_cursor.next()?;
                let (next_cursor, value) = u32::parse(modifier_cursor)?;
                Some(match modifier {
                    '+' => Drop::Highest(value as usize),
                    '-' => Drop::Lowest(value as usize),
                    _ => return Err(modifier_cursor.index)
                })
            } else {
                None
            }
        } else {
            None
        };
        Ok((cursor, Dice { count, max, drop }))
    }
}

impl Parse for Bonus {
    fn parse(cursor: Cursor) -> Result<(Cursor, Self), usize> {
        unimplemented!()
    }
}

impl Parse for Component {
    fn parse(cursor: Cursor) -> Result<(Cursor, Self), usize> {
        unimplemented!()
    }
}

impl Parse for Args {
    fn parse(cursor: Cursor) -> Result<(Cursor, Self), usize> {
        unimplemented!()
    }
}

macro_rules! test_dice {
    ($str: literal) => {
        let str = $str;
        let cursor = Cursor { value: str, index: 0 };
        let dice = Dice::parse(cursor).unwrap().1;
        println!("{:?}", dice)
    };
}

fn main() {
    test_dice!("d2");
    test_dice!("320d324");
    test_dice!("1d3d+4");
    test_dice!("4d8d+3");
    // let value = "1234 5432";
    // let cursor = Cursor { value, index: 0 };
    // let (mut cursor, uint) = cursor.parse_uint().unwrap();
    // assert_eq!(uint, 1234);
    // cursor.flush_whitespace();
    // let (cursor, uint) = cursor.parse_uint().unwrap();
    // assert_eq!(uint, 5432);
    // assert_eq!(cursor.next().err().unwrap(), 9);
    // let d = Dice {
    //     count: 10,
    //     max: 10,
    //     drop: 2,
    // };
    // println!("{:?}", d.generate())
}