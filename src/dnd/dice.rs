use rand::Rng;
use crate::dnd::{DropDirection, Dice};

impl Dice {
    pub fn validate(&self) -> Option<String> {
        if let Some(drop) = &self.drop {
            if drop.value >= self.count {
                Some("the drop is too large for this many rolls".to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn generate<R: Rng>(&self, rng: &mut R) -> Vec<u32> {
        let mut v = vec![];
        for _ in 0..self.count {
            v.push(rng.gen_range(1, self.max + 1))
        }
        v.sort();
        if let Some(drop) = &self.drop {
            match drop.direction {
                DropDirection::Highest => v.drain(v.len() - drop.value as usize..),
                DropDirection::Lowest => v.drain(0..drop.value as usize)
            };
        }
        v
    }
}