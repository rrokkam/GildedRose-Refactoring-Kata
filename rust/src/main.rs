trait Tick {
    fn days_remaining(&self) -> i32;
}

impl PartialEq for dyn Tick {
    fn eq(&self, other: &dyn Tick) -> bool {
        self.days_remaining() == other.days_remaining()
    }
}

struct Item {
    days_remaining: i32,
}

impl Tick for Item {
    fn days_remaining(&self) -> i32 {
        self.days_remaining
    }
}

fn main() {
    let item = Item { days_remaining: 3 };
    let item2 = Item { days_remaining: 3 };
    if item == item2 {
        println!("They're equal!")
    }
}
