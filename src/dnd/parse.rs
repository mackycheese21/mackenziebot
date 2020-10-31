use crate::dnd::{Args, Bonus, Component, Cursor, Dice, Drop, DropDirection, Sign, Term};

pub trait Parse: Sized {
    fn parse(cursor: Cursor) -> Result<(Cursor, Self), usize>;
}

// Also for bonus since bonus = u32
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
        let mut value = parse_digit(ch).filter(|c| *c > 0).ok_or(cursor.index)?;
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
                cursor = next_cursor;
                Some(Drop {
                    direction: match modifier {
                        '+' => DropDirection::Highest,
                        '-' => DropDirection::Lowest,
                        _ => return Err(modifier_cursor.index)
                    },
                    value,
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

impl Parse for Component {
    fn parse(cursor: Cursor) -> Result<(Cursor, Self), usize> {
        let dice = Dice::parse(cursor);
        if dice.is_ok() {
            dice.map(|(cursor, dice)| (cursor, Component::Dice(dice)))
        } else {
            let bonus = Bonus::parse(cursor);
            if bonus.is_ok() {
                bonus.map(|(cursor, bonus)| (cursor, Component::Bonus(bonus)))
            } else {
                Err(cursor.index)
            }
        }
    }
}

impl Parse for Args {
    fn parse(cursor: Cursor) -> Result<(Cursor, Self), usize> {
        let (mut cursor, first_component) = Component::parse(cursor)?;
        let mut terms = vec![Term {
            sign: Sign::Positive,
            component: first_component,
        }];
        loop {
            cursor.flush_whitespace();
            if let Ok((next_cursor, ch)) = cursor.next() {
                cursor = next_cursor;
                let sign = match ch {
                    '+' => Sign::Positive,
                    '-' => Sign::Negative,
                    _ => return Err(cursor.index)
                };
                cursor.flush_whitespace();
                let (next_cursor, next_component) = Component::parse(cursor)?;
                cursor = next_cursor;
                terms.push(Term {
                    sign,
                    component: next_component,
                })
            } else {
                return Ok((cursor, Args { terms }));
            }
        }
    }
}