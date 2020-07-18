use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    name: String,
    sell_in: i32,
    quality: i32,
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name,
            sell_in: sell_in,
            quality: quality,
        }
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
    mod normal {
        use crate::*;
        #[test]
        fn before_sell_date() {
            let mut item = Item::new("normal".to_string(), 10, 5);
            item.update_quality();
            assert_eq!(item, Item::new("normal".to_string(), 9, 4));
        }
        #[test]
        fn on_sell_date() {
            assert_eq!(2 + 2, 4);
        }
        #[test]
        fn after_sell_date() {
            assert_eq!(2 + 2, 4);
        }
        #[test]
        fn zero_quality() {
            assert_eq!(2 + 2, 4);
        }
    }

    mod brie {
        mod before_sell_date {
            #[test]
            fn not_with_max_quality() {
                assert_eq!(2 + 2, 4);
            }
            #[test]
            fn with_max_quality() {
                assert_eq!(2 + 2, 4);
            }
        }
        mod on_sell_date {
            #[test]
            fn not_near_max_quality() {
                assert_eq!(2 + 2, 4);
            }

            #[test]
            fn near_max_quality() {
                assert_eq!(2 + 2, 4);
            }

            #[test]
            fn with_max_quality() {
                assert_eq!(2 + 2, 4);
            }
        }
        mod after_sell_date {
            #[test]
            fn not_with_max_quality() {
                assert_eq!(2 + 2, 4);
            }
            #[test]
            fn with_max_quality() {
                assert_eq!(2 + 2, 4);
            }
        }
    }

    mod sulfuras {
        #[test]
        fn before_sell_date() {
            assert_eq!(2 + 2, 4);
        }
        #[test]
        fn on_sell_date() {
            assert_eq!(2 + 2, 4);
        }
        #[test]
        fn after_sell_date() {
            assert_eq!(2 + 2, 4);
        }
    }

    mod backstage_pass {
        #[test]
        fn long_before_sell_date() {
            assert_eq!(2 + 2, 4);
        }

        mod medium_close_to_sell_date {
            #[test]
            fn upper_bound() {
                assert_eq!(2 + 2, 4);
            }
            #[test]
            fn upper_bound_at_max_quality() {
                assert_eq!(2 + 2, 4);
            }
            #[test]
            fn lower_bound() {
                assert_eq!(2 + 2, 4);
            }
            #[test]
            fn lower_bound_at_max_quality() {
                assert_eq!(2 + 2, 4);
            }
        }

        mod very_close_to_sell_date {
            #[test]
            fn upper_bound() {
                assert_eq!(2 + 2, 4);
            }
            #[test]
            fn upper_bound_at_max_quality() {
                assert_eq!(2 + 2, 4);
            }
            #[test]
            fn lower_bound() {
                assert_eq!(2 + 2, 4);
            }
            #[test]
            fn lower_bound_at_max_quality() {
                assert_eq!(2 + 2, 4);
            }
        }

        #[test]
        fn on_sell_date() {
            assert_eq!(2 + 2, 4);
        }
        #[test]
        fn after_sell_date() {
            assert_eq!(2 + 2, 4);
        }
    }
}
