use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    name: String,
    sell_in: i32,
    quality: i32,
}

impl Item {
    pub fn new(name: impl AsRef<str>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.as_ref().to_string(),
            sell_in: sell_in,
            quality: quality,
        }
    }

    #[cfg(test)]
    fn updated_once(name: impl AsRef<str>, sell_in: i32, quality: i32) -> Item {
        let mut item = Item::new(name, sell_in, quality);
        item.update_quality();
        item
    }

    pub fn update_quality(&mut self) {
        if self.name != "Aged Brie" && self.name != "Backstage passes to a TAFKAL80ETC concert" {
            if self.quality > 0 {
                if self.name != "Sulfuras, Hand of Ragnaros" {
                    self.quality = self.quality - 1;
                }
            }
        } else {
            if self.quality < 50 {
                self.quality = self.quality + 1;

                if self.name == "Backstage passes to a TAFKAL80ETC concert" {
                    if self.sell_in < 11 {
                        if self.quality < 50 {
                            self.quality = self.quality + 1;
                        }
                    }

                    if self.sell_in < 6 {
                        if self.quality < 50 {
                            self.quality = self.quality + 1;
                        }
                    }
                }
            }
        }

        if self.name != "Sulfuras, Hand of Ragnaros" {
            self.sell_in = self.sell_in - 1;
        }

        if self.sell_in < 0 {
            if self.name != "Aged Brie" {
                if self.name != "Backstage passes to a TAFKAL80ETC concert" {
                    if self.quality > 0 {
                        if self.name != "Sulfuras, Hand of Ragnaros" {
                            self.quality = self.quality - 1;
                        }
                    }
                } else {
                    self.quality = self.quality - self.quality;
                }
            } else {
                if self.quality < 50 {
                    self.quality = self.quality + 1;
                }
            }
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items: items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            item.update_quality();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const NORMAL: &'static str = "Normal";
    const BRIE: &'static str = "Aged Brie";
    const SULFURAS: &'static str = "Sulfuras, Hand of Ragnaros";
    const PASS: &'static str = "Backstage passes to a TAFKAL80ETC concert";
    const CONJURED: &'static str = "Conjured Mana Cake";

    mod normal {
        use super::*;

        #[test]
        fn before_sell_date() {
            assert_eq!(Item::updated_once(NORMAL, 10, 5), Item::new(NORMAL, 9, 4));
        }

        #[test]
        fn on_sell_date() {
            assert_eq!(Item::updated_once(NORMAL, 0, 5), Item::new(NORMAL, -1, 3));
        }

        #[test]
        fn after_sell_date() {
            assert_eq!(
                Item::updated_once(NORMAL, -10, 5),
                Item::new(NORMAL, -11, 3)
            );
        }

        #[test]
        fn zero_quality() {
            assert_eq!(Item::updated_once(NORMAL, 10, 0), Item::new(NORMAL, 9, 0));
        }
    }

    mod brie {
        use super::*;

        mod before_sell_date {
            use super::*;

            #[test]
            fn not_with_max_quality() {
                assert_eq!(Item::updated_once(BRIE, 10, 5), Item::new(BRIE, 9, 6));
            }

            #[test]
            fn with_max_quality() {
                assert_eq!(Item::updated_once(BRIE, 10, 50), Item::new(BRIE, 9, 50));
            }
        }

        mod on_sell_date {
            use super::*;

            #[test]
            fn not_near_max_quality() {
                assert_eq!(Item::updated_once(BRIE, 0, 5), Item::new(BRIE, -1, 7));
            }

            #[test]
            fn near_max_quality() {
                assert_eq!(Item::updated_once(BRIE, 0, 49), Item::new(BRIE, -1, 50));
            }

            #[test]
            fn with_max_quality() {
                assert_eq!(Item::updated_once(BRIE, 0, 50), Item::new(BRIE, -1, 50));
            }
        }

        mod after_sell_date {
            use super::*;

            #[test]
            fn not_with_max_quality() {
                assert_eq!(Item::updated_once(BRIE, -10, 50), Item::new(BRIE, -11, 50));
            }

            #[test]
            fn with_max_quality() {
                assert_eq!(Item::updated_once(BRIE, -10, 50), Item::new(BRIE, -11, 50));
            }
        }
    }

    mod sulfuras {
        use super::*;

        #[test]
        fn before_sell_date() {
            assert_eq!(
                Item::updated_once(SULFURAS, 10, 5),
                Item::new(SULFURAS, 10, 5)
            );
        }

        #[test]
        fn on_sell_date() {
            assert_eq!(
                Item::updated_once(SULFURAS, 0, 5),
                Item::new(SULFURAS, 0, 5)
            );
        }

        #[test]
        fn after_sell_date() {
            assert_eq!(
                Item::updated_once(SULFURAS, -10, 5),
                Item::new(SULFURAS, -10, 5)
            );
        }
    }

    mod backstage_pass {
        use super::*;

        #[test]
        fn long_before_sell_date() {
            assert_eq!(Item::updated_once(PASS, 11, 5), Item::new(PASS, 10, 6));
        }

        mod medium_close_to_sell_date {
            use super::*;

            #[test]
            fn upper_bound() {
                assert_eq!(Item::updated_once(PASS, 10, 5), Item::new(PASS, 9, 7));
            }

            #[test]
            fn upper_bound_at_max_quality() {
                assert_eq!(Item::updated_once(PASS, 10, 50), Item::new(PASS, 9, 50));
            }

            #[test]
            fn lower_bound() {
                assert_eq!(Item::updated_once(PASS, 6, 5), Item::new(PASS, 5, 7));
            }

            #[test]
            fn lower_bound_at_max_quality() {
                assert_eq!(Item::updated_once(PASS, 6, 50), Item::new(PASS, 5, 50));
            }
        }

        mod very_close_to_sell_date {
            use super::*;

            #[test]
            fn upper_bound() {
                assert_eq!(Item::updated_once(PASS, 5, 5), Item::new(PASS, 4, 8));
            }

            #[test]
            fn upper_bound_at_max_quality() {
                assert_eq!(Item::updated_once(PASS, 5, 50), Item::new(PASS, 4, 50));
            }

            #[test]
            fn lower_bound() {
                assert_eq!(Item::updated_once(PASS, 1, 5), Item::new(PASS, 0, 8));
            }

            #[test]
            fn lower_bound_at_max_quality() {
                assert_eq!(Item::updated_once(PASS, 1, 50), Item::new(PASS, 0, 50));
            }
        }

        #[test]
        fn on_sell_date() {
            assert_eq!(Item::updated_once(PASS, 0, 5), Item::new(PASS, -1, 0));
        }

        #[test]
        fn after_sell_date() {
            assert_eq!(Item::updated_once(PASS, -10, 50), Item::new(PASS, -11, 0));
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
                    Item::updated_once(CONJURED, 10, 5),
                    Item::new(CONJURED, 9, 3)
                );
            }

            #[test]
            #[ignore]
            fn with_zero_quality() {
                assert_eq!(
                    Item::updated_once(CONJURED, 10, 0),
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
                    Item::updated_once(CONJURED, 0, 5),
                    Item::new(CONJURED, -1, 1)
                );
            }

            #[test]
            #[ignore]
            fn with_zero_quality() {
                assert_eq!(
                    Item::updated_once(CONJURED, 0, 0),
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
                    Item::updated_once(CONJURED, -10, 5),
                    Item::new(CONJURED, -11, 1)
                );
            }

            #[test]
            #[ignore]
            fn with_zero_quality() {
                assert_eq!(
                    Item::updated_once(CONJURED, -10, 0),
                    Item::new(CONJURED, -11, 0)
                );
            }
        }
    }

    #[test]
    fn several_items() {
        let mut rose = GildedRose::new(vec![
            Item::new("a normal item", 5, 10),
            Item::new("Aged Brie", 3, 10),
        ]);
        rose.update_quality();

        assert_eq!(
            rose.items,
            vec![
                Item::new("a normal item", 4, 9),
                Item::new("Aged Brie", 2, 11)
            ]
        );
    }
}
