mod fecs;
mod bitset;
mod query;
mod component;
mod sparse_set;

use fecs::FECS;
use component::Component;

fn main() {
    let mut fecs = FECS::new();
    u32::register(&mut fecs);

    let e0 = fecs.add_entity();
    let e1 = fecs.add_entity();
    let e2 = fecs.add_entity();

    fecs.attach::<u32>(&e0, 32);
    fecs.attach::<u32>(&e1, 64);
    fecs.attach::<u32>(&e2, 128);

    fecs.query::<u32>()
        .iter()
        .for_each(|e| {
            let value = fecs.get::<u32>(&e).unwrap();
            println!("{}", value);
        });
}
