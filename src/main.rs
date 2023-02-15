pub trait Between {
    fn between(self, min: Self, max: Self) -> bool where Self: PartialOrd<Self> + Sized {
        self > min && self < max
    }
}

impl Between for u32 {
    fn between(self, min: u32, max: u32) -> bool {
        self > min && self < max
    }
}

fn main() {
    println!("Hello, world!");
    assert!(10_u32.between(9, 11))
}
