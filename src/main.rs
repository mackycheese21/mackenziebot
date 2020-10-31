mod dnd;
use dnd::{Args, Cursor, Parse};

macro_rules! test_dice {
    ($str: literal) => {
        let cursor = Cursor::new($str);
        let args = Args::parse(cursor).unwrap().1;
        println!("{:?}", args)
    };
}

fn main() {
    test_dice!("d2");
    test_dice!("320d324");
    test_dice!("1d3d+4");
    test_dice!("3 - 4d8d+3");
}