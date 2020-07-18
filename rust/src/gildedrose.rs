use std::string;
use std::vec;

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name,
            sell_in: sell_in,
            quality: quality,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GildedRose {
    pub items: vec::Vec<Item>,
}

impl GildedRose {
    pub fn new(items: vec::Vec<Item>) -> GildedRose {
        GildedRose { items: items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality = item.quality - 1;
                    }
                }
            } else {
                if item.quality < 50 {
                    item.quality = item.quality + 1;

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in - 1;
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                        if item.quality > 0 {
                            if item.name != "Sulfuras, Hand of Ragnaros" {
                                item.quality = item.quality - 1;
                            }
                        }
                    } else {
                        item.quality = item.quality - item.quality;
                    }
                } else {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod normal {
        use crate::*;
        #[test]
        fn before_sell_date() {
            let item = Item::new("normal".to_string(), 10, 5);
            let mut rose = GildedRose::new(vec![item]);
            rose.update_quality();
            assert_eq!(rose.items.len(), 1);
            assert_eq!(rose.items[0], Item::new("normal".to_string(), 9, 4));
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
