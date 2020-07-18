use std::fmt::{self, Debug, Display};

trait Tick {
    fn tick(&mut self);
    fn name(&self) -> String;
    fn days_remaining(&self) -> i32;
    fn quality(&self) -> u32;
}

impl Display for dyn Tick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.name(),
            self.days_remaining(),
            self.quality()
        )
    }
}

impl Debug for dyn Tick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Item")
            .field("name", &self.name())
            .field("days_remaining", &self.days_remaining())
            .field("quality", &self.quality())
            .finish()
    }
}

impl PartialEq for dyn Tick {
    fn eq(&self, other: &dyn Tick) -> bool {
        self.name() == other.name()
            && self.days_remaining() == other.days_remaining()
            && self.quality() == other.quality()
    }
}

impl Eq for dyn Tick {}

struct Ordinary {
    name: String,
    days_remaining: i32,
    quality: u32,
}

impl Ordinary {
    pub fn new(name: impl AsRef<str>, days_remaining: i32, quality: u32) -> Ordinary {
        Ordinary {
            name: name.as_ref().to_string(),
            days_remaining,
            quality,
        }
    }
}

impl Tick for Ordinary {
    fn tick(&mut self) {
        self.days_remaining -= 1;
        self.quality = self.quality.saturating_sub(1);
        if self.days_remaining < 0 {
            self.quality = self.quality.saturating_sub(1);
        }
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn days_remaining(&self) -> i32 {
        self.days_remaining
    }
    fn quality(&self) -> u32 {
        self.quality
    }
}
pub struct Item {
    name: String,
    days_remaining: i32,
    quality: u32,
    item: Option<Box<dyn Tick>>,
}

impl Tick for Item {
    fn tick(&mut self) {
        match self.name.as_ref() {
            "Aged Brie" => self.brie_tick(),
            "Sulfuras, Hand of Ragnaros" => self.sulfuras_tick(),
            "Backstage passes to a TAFKAL80ETC concert" => self.backstage_tick(),
            _ => self.ordinary_tick(),
        }
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn days_remaining(&self) -> i32 {
        self.days_remaining
    }
    fn quality(&self) -> u32 {
        self.quality
    }
}

impl Item {
    pub fn new(name: impl AsRef<str>, days_remaining: i32, quality: u32) -> Item {
        let name = name.as_ref().to_string();
        if name == "Aged Brie"
            || name == "Sulfuras, Hand of Ragnaros"
            || name == "Backstage passes to a TAFKAL80ETC concert"
        {
            Item {
                name,
                days_remaining,
                quality,
                item: None,
            }
        } else {
            Item {
                name: name.clone(),
                days_remaining,
                quality,
                item: Some(Box::new(Ordinary::new(name, days_remaining, quality))),
            }
        }
    }

    #[cfg(test)]
    fn ticked_once(name: impl AsRef<str>, days_remaining: i32, quality: u32) -> Item {
        let mut item = Item::new(name, days_remaining, quality);
        item.tick();
        item
    }

    fn ordinary_tick(&mut self) {
        self.item.as_mut().unwrap().tick();
    }

    fn brie_tick(&mut self) {
        self.days_remaining -= 1;
        if self.quality == 50 {
            return;
        }
        self.quality += 1;
        if self.days_remaining < 0 && self.quality < 50 {
            self.quality += 1;
        }
    }

    fn sulfuras_tick(&mut self) {}

    fn backstage_tick(&mut self) {
        self.days_remaining -= 1;
        if self.days_remaining < 0 {
            self.quality = 0;
            return;
        }

        if self.quality < 50 {
            self.quality += 1;
        }

        if self.days_remaining < 10 && self.quality < 50 {
            self.quality += 1
        }

        if self.days_remaining < 5 && self.quality < 50 {
            self.quality += 1
        }
    }
}

pub struct GildedRose {
    items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn tick(&mut self) {
        for item in &mut self.items {
            item.tick();
        }
    }
}

impl Display for GildedRose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "name, sellIn, quality")?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ORDINARY: &str = "Ordinary";
    const AGED_BRIE: &str = "Aged Brie";
    const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";
    const BACKSTAGE_PASS: &str = "Backstage passes to a TAFKAL80ETC concert";
    const CONJURED: &str = "Conjured Mana Cake";

    mod ordinary {
        use super::*;

        #[test]
        fn before_sell_date() {
            assert_eq!(
                Item::ticked_once(ORDINARY, 10, 5),
                Item::new(ORDINARY, 9, 4)
            );
        }

        #[test]
        fn on_sell_date() {
            assert_eq!(
                Item::ticked_once(ORDINARY, 0, 5),
                Item::new(ORDINARY, -1, 3)
            );
        }

        #[test]
        fn after_sell_date() {
            assert_eq!(
                Item::ticked_once(ORDINARY, -10, 5),
                Item::new(ORDINARY, -11, 3)
            );
        }

        #[test]
        fn zero_quality() {
            assert_eq!(
                Item::ticked_once(ORDINARY, 10, 0),
                Item::new(ORDINARY, 9, 0)
            );
        }
    }

    mod brie {
        use super::*;

        mod before_sell_date {
            use super::*;

            #[test]
            fn not_with_max_quality() {
                assert_eq!(
                    Item::ticked_once(AGED_BRIE, 10, 5),
                    Item::new(AGED_BRIE, 9, 6)
                );
            }

            #[test]
            fn with_max_quality() {
                assert_eq!(
                    Item::ticked_once(AGED_BRIE, 10, 50),
                    Item::new(AGED_BRIE, 9, 50)
                );
            }
        }

        mod on_sell_date {
            use super::*;

            #[test]
            fn not_near_max_quality() {
                assert_eq!(
                    Item::ticked_once(AGED_BRIE, 0, 5),
                    Item::new(AGED_BRIE, -1, 7)
                );
            }

            #[test]
            fn near_max_quality() {
                assert_eq!(
                    Item::ticked_once(AGED_BRIE, 0, 49),
                    Item::new(AGED_BRIE, -1, 50)
                );
            }

            #[test]
            fn with_max_quality() {
                assert_eq!(
                    Item::ticked_once(AGED_BRIE, 0, 50),
                    Item::new(AGED_BRIE, -1, 50)
                );
            }
        }

        mod after_sell_date {
            use super::*;

            #[test]
            fn not_with_max_quality() {
                assert_eq!(
                    Item::ticked_once(AGED_BRIE, -10, 5),
                    Item::new(AGED_BRIE, -11, 7)
                );
            }

            #[test]
            fn with_max_quality() {
                assert_eq!(
                    Item::ticked_once(AGED_BRIE, -10, 50),
                    Item::new(AGED_BRIE, -11, 50)
                );
            }
        }
    }

    mod sulfuras {
        use super::*;

        #[test]
        fn before_sell_date() {
            assert_eq!(
                Item::ticked_once(SULFURAS, 10, 5),
                Item::new(SULFURAS, 10, 5)
            );
        }

        #[test]
        fn on_sell_date() {
            assert_eq!(Item::ticked_once(SULFURAS, 0, 5), Item::new(SULFURAS, 0, 5));
        }

        #[test]
        fn after_sell_date() {
            assert_eq!(
                Item::ticked_once(SULFURAS, -10, 5),
                Item::new(SULFURAS, -10, 5)
            );
        }
    }

    mod backstage_pass {
        use super::*;

        #[test]
        fn long_before_sell_date() {
            assert_eq!(
                Item::ticked_once(BACKSTAGE_PASS, 11, 5),
                Item::new(BACKSTAGE_PASS, 10, 6)
            );
        }

        mod medium_close_to_sell_date {
            use super::*;

            #[test]
            fn upper_bound() {
                assert_eq!(
                    Item::ticked_once(BACKSTAGE_PASS, 10, 5),
                    Item::new(BACKSTAGE_PASS, 9, 7)
                );
            }

            #[test]
            fn upper_bound_at_max_quality() {
                assert_eq!(
                    Item::ticked_once(BACKSTAGE_PASS, 10, 50),
                    Item::new(BACKSTAGE_PASS, 9, 50)
                );
            }

            #[test]
            fn lower_bound() {
                assert_eq!(
                    Item::ticked_once(BACKSTAGE_PASS, 6, 5),
                    Item::new(BACKSTAGE_PASS, 5, 7)
                );
            }

            #[test]
            fn lower_bound_at_max_quality() {
                assert_eq!(
                    Item::ticked_once(BACKSTAGE_PASS, 6, 50),
                    Item::new(BACKSTAGE_PASS, 5, 50)
                );
            }
        }

        mod very_close_to_sell_date {
            use super::*;

            #[test]
            fn upper_bound() {
                assert_eq!(
                    Item::ticked_once(BACKSTAGE_PASS, 5, 5),
                    Item::new(BACKSTAGE_PASS, 4, 8)
                );
            }

            #[test]
            fn upper_bound_at_max_quality() {
                assert_eq!(
                    Item::ticked_once(BACKSTAGE_PASS, 5, 50),
                    Item::new(BACKSTAGE_PASS, 4, 50)
                );
            }

            #[test]
            fn lower_bound() {
                assert_eq!(
                    Item::ticked_once(BACKSTAGE_PASS, 1, 5),
                    Item::new(BACKSTAGE_PASS, 0, 8)
                );
            }

            #[test]
            fn lower_bound_at_max_quality() {
                assert_eq!(
                    Item::ticked_once(BACKSTAGE_PASS, 1, 50),
                    Item::new(BACKSTAGE_PASS, 0, 50)
                );
            }
        }

        #[test]
        fn on_sell_date() {
            assert_eq!(
                Item::ticked_once(BACKSTAGE_PASS, 0, 5),
                Item::new(BACKSTAGE_PASS, -1, 0)
            );
        }

        #[test]
        fn after_sell_date() {
            assert_eq!(
                Item::ticked_once(BACKSTAGE_PASS, -10, 50),
                Item::new(BACKSTAGE_PASS, -11, 0)
            );
        }
    }

    mod conjured {
        use super::*;

        mod before_sell_date {
            use super::*;

            #[test]
            #[ignore]
            fn not_with_zero_quality() {
                assert_eq!(
                    Item::ticked_once(CONJURED, 10, 5),
                    Item::new(CONJURED, 9, 3)
                );
            }

            #[test]
            #[ignore]
            fn with_zero_quality() {
                assert_eq!(
                    Item::ticked_once(CONJURED, 10, 0),
                    Item::new(CONJURED, 9, 0)
                );
            }
        }

        mod on_sell_date {
            use super::*;

            #[test]
            #[ignore]
            fn not_with_zero_quality() {
                assert_eq!(
                    Item::ticked_once(CONJURED, 0, 5),
                    Item::new(CONJURED, -1, 1)
                );
            }

            #[test]
            #[ignore]
            fn with_zero_quality() {
                assert_eq!(
                    Item::ticked_once(CONJURED, 0, 0),
                    Item::new(CONJURED, -1, 0)
                );
            }
        }

        mod after_sell_date {
            use super::*;

            #[test]
            #[ignore]
            fn not_with_zero_quality() {
                assert_eq!(
                    Item::ticked_once(CONJURED, -10, 5),
                    Item::new(CONJURED, -11, 1)
                );
            }

            #[test]
            #[ignore]
            fn with_zero_quality() {
                assert_eq!(
                    Item::ticked_once(CONJURED, -10, 0),
                    Item::new(CONJURED, -11, 0)
                );
            }
        }
    }

    #[test]
    fn several_items() {
        let mut rose = GildedRose::new(vec![
            Item::new("a ordinary item", 5, 10),
            Item::new("Aged Brie", 3, 10),
        ]);
        rose.tick();

        assert_eq!(
            rose.items,
            vec![
                Item::new("a ordinary item", 4, 9),
                Item::new("Aged Brie", 2, 11)
            ]
        );
    }
}
