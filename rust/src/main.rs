trait Tick {}
impl PartialEq for dyn Tick {
    fn eq(&self, _: &dyn Tick) -> bool {
        true
    }
}

struct Item();
impl Tick for Item {}

fn main() {
    let item = Item {};
    let item2 = Item {};
    if item == item2 {
        println!("Equality check is a compile error")
    }
}
