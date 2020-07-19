use enum_dispatch::enum_dispatch;
use std::fmt::{self, Display};

#[enum_dispatch]
trait Tick {
    fn tick(&mut self);
    fn name(&self) -> String;
    fn days_remaining(&self) -> i32;
    fn quality(&self) -> u32;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Brie {
    days_remaining: i32,
    quality: u32,
}

impl Tick for Brie {
    fn tick(&mut self) {
        self.days_remaining -= 1;
        if self.quality == 50 {
            return;
        }
        self.quality += 1;
        if self.days_remaining < 0 && self.quality < 50 {
            self.quality += 1;
        }
    }
    fn name(&self) -> String {
        "Aged Brie".to_string()
    }
    fn days_remaining(&self) -> i32 {
        self.days_remaining
    }
    fn quality(&self) -> u32 {
        self.quality
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Backstage {
    days_remaining: i32,
    quality: u32,
}

impl Tick for Backstage {
    fn tick(&mut self) {
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
    fn name(&self) -> String {
        "Backstage passes to a TAFKAL80ETC concert".to_string()
    }
    fn days_remaining(&self) -> i32 {
        self.days_remaining
    }
    fn quality(&self) -> u32 {
        self.quality
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Sulfuras {
    days_remaining: i32,
    quality: u32,
}

impl Tick for Sulfuras {
    fn tick(&mut self) {}
    fn name(&self) -> String {
        "Sulfuras, Hand of Ragnaros".to_string()
    }
    fn days_remaining(&self) -> i32 {
        self.days_remaining
    }
    fn quality(&self) -> u32 {
        self.quality
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Ordinary {
    name: String,
    days_remaining: i32,
    quality: u32,
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

#[enum_dispatch(Tick)]
#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Brie,
    Backstage,
    Sulfuras,
    Ordinary,
}

impl Item {
    pub fn new(name: impl AsRef<str>, days_remaining: i32, quality: u32) -> Self {
        match name.as_ref() {
            "Aged Brie" => Brie {
                days_remaining,
                quality,
            }
            .into(),
            "Backstage passes to a TAFKAL80ETC concert" => Backstage {
                days_remaining,
                quality,
            }
            .into(),
            "Sulfuras, Hand of Ragnaros" => Sulfuras {
                days_remaining,
                quality,
            }
            .into(),
            _ => Ordinary {
                name: name.as_ref().to_string(),
                days_remaining,
                quality,
            }
            .into(),
        }
    }

    #[cfg(test)]
    pub fn ticked_once(name: impl AsRef<str>, days_remaining: i32, quality: u32) -> Self {
        let mut item = Item::new(name, days_remaining, quality);
        item.tick();
        item
    }
}

impl Display for Item {
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

#[derive(Debug, PartialEq, Eq)]
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
