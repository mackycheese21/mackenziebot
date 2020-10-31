mod dnd;
use dnd::{Args, Cursor, Parse, Component};
use crate::dnd::Sign;

fn evaluate(args: Args) -> String {
    let mut result = "".to_string();
    let mut sum = 0;
    let mut total_bonus = 0;
    for arg in args.terms {
        let mul = match arg.sign {
            Sign::Positive => 1,
            Sign::Negative => -1
        };
        let (sum_change, bonus_change) = match arg.component {
            Component::Dice(dice) => {
                if let Some(err) = dice.validate() {
                    return format!("Error validating dice {}{}: {}", match arg.sign {
                        Sign::Positive => "",
                        Sign::Negative => "-"
                    }, dice, err)
                }
                let values = dice.generate(&mut rand::thread_rng());
                result = format!("{}\n{}: {:?}", result, dice, values);
                (values.iter().sum::<u32>(), 0)
            },
            Component::Bonus(bonus) => {
                (bonus, bonus)
            }
        };
        sum += mul * sum_change as i32;
        total_bonus += mul * bonus_change as i32;
    }
    format!("{}\nTotal bonus: {}\nSum: {}", &result[1..], total_bonus, sum)
}

macro_rules! t {
    ($str: literal) => {
        let cursor = Cursor::new($str);
        let args = Args::parse(cursor).unwrap().1;
        println!("{}:\n{}\n", $str, evaluate(args))
    };
}

fn main() {
    t!("5d10d+2 + 2d5 - 2");
    t!("5d15d+20");
    t!("69d420 - 666");
}