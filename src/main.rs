use rand::Rng;
use std::convert::{TryFrom, TryInto};

struct Dice {
    count: u32,
    max: u32,
    drop: u32,
}

impl Dice {
    fn generate(&self) -> Vec<u32> {
        let mut v = vec![];
        for _ in 0..self.count {
            v.push(rand::thread_rng().gen_range(1, self.count + 1))
        }
        v.sort();
        if self.drop < 0 {
            v.drain(..-self.drop)
        } else if self.drop > 0 {
            v.drain(v.len() - self.drop..)
        }
        v
    }
}

type Bonus = u32;

enum Component {
    Dice(Dice),
    Bonus(Bonus),
}

type Args = Vec<Component>;

type ParseOutput<'a, T> = (&'a str, T);

impl<'a> TryFrom<&'a str> for ParseOutput<'a, Dice> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl<'a> TryFrom<&'a str> for ParseOutput<'a, Bonus> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl<'a> TryFrom<&'a str> for ParseOutput<'a, Component> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Ok((value, dice)) = value.try_into() {
            Ok((value, Component::Dice(dice)))
        } else if let Ok((value, bonus)) = value.try_into() {
            Ok((value, Component::Bonus(bonus)))
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a str> for ParseOutput<'a, Args> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

fn main() {
    let d = Dice {
        count: 10,
        max: 10,
        drop: 2,
    };
    println!("{:?}", d.generate())
}