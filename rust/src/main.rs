trait Tick {
    fn tick(&mut self);
    fn days_remaining(&self) -> i32;
}

impl PartialEq for dyn Tick {
    fn eq(&self, other: &dyn Tick) -> bool {
        self.days_remaining() == other.days_remaining()
    }
}

impl Eq for dyn Tick {}

pub struct Item {
    name: String,
    days_remaining: i32,
    quality: u32,
    item: Option<Box<dyn Tick>>,
}

impl Tick for Item {
    fn tick(&mut self) {
        self.item.as_mut().unwrap().tick();
    }

    fn days_remaining(&self) -> i32 {
        self.days_remaining
    }
}

impl Item {
    pub fn new(name: impl AsRef<str>, days_remaining: i32, quality: u32) -> Item {
        let name = name.as_ref().to_string();
        Item {
            name,
            days_remaining,
            quality,
            item: None,
        }
    }
}

fn main() {
    let item = Item::new("name", 5, 3);
    let item2 = Item::new("blah", 3, 2);
    if item == item2 {
        println!("They're equal!")
    }
}
