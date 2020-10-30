use rand::Rng;

struct Dice {
    count: u32,
    max: u32,
    drop: u32,
}

impl Dice {
    fn generate(&self) -> Vec<u32> {
        unimplemented!()
    }
}

type Bonus = u32;

enum Component {
    Dice(Dice),
    Bonus(Bonus),
}

type Args = Vec<Component>;

impl From<&str> for Option<Dice> {
    fn from(_: &str) -> Self {
        unimplemented!()
    }
}

impl From<&str> for Option<Bonus> {
    fn from(_: &str) -> Self {
        unimplemented!()
    }
}

impl From<&str> for Option<Component> {
    fn from(_: &str) -> Self {
        unimplemented!()
    }
}

impl From<&str> for Option<Args> {
    fn from(_: &str) -> Self {
        unimplemented!()
    }
}

fn main() {}