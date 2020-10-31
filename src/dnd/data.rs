#[derive(Debug)]
pub enum DropDirection {
    Highest,
    Lowest,
}

#[derive(Debug)]
pub struct Drop {
    pub direction: DropDirection,
    pub value: u32,
}

#[derive(Debug)]
pub struct Dice {
    pub count: u32,
    pub max: u32,
    pub drop: Option<Drop>,
}

pub type Bonus = u32;

#[derive(Debug)]
pub enum Component {
    Dice(Dice),
    Bonus(Bonus),
}

#[derive(Debug)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug)]
pub struct Term {
    pub component: Component,
    pub sign: Sign,
}

#[derive(Debug)]
pub struct Args {
    pub terms: Vec<Term>
}