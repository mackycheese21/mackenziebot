use crate::dnd::{DropDirection, Drop, Dice, Component, Sign, Term};
use std::fmt::{Display, Formatter, Result};

impl Display for DropDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            DropDirection::Highest => '+',
            DropDirection::Lowest => '-'
        })
    }
}

impl Display for Drop {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.direction, self.value)
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.count > 1 {
            write!(f, "{}", self.count)?;
        }
        write!(f, "d{}", self.max)?;
        if let Some(drop) = &self.drop {
            write!(f, "d{}", drop)
        } else {
            Ok(())
        }
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Component::Dice(dice) => write!(f, "{}", dice),
            Component::Bonus(bonus) => write!(f, "{}", bonus)
        }
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Sign::Positive => '+',
            Sign::Negative => '-'
        })
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.sign, self.component)
    }
}