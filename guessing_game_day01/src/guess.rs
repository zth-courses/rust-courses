pub struct Guess {}

use rand::{thread_rng, Rng};

impl Guess {
    pub fn gen_random() -> u32 {
        let mut rng = thread_rng();
        let mut y: u32 = 0;
        if rng.gen() {
            y = rng.gen_range(1..101); // [1, 101)
        }
        return y;
    }
}
