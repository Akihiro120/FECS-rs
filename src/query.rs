use crate::{FECS, component::Component, bitset::Bitset};
use std::any::TypeId;

pub trait Query {
    fn signature(fecs: &FECS) -> Bitset;
}

// have a case for an empty bit
impl <T>Query for T 
where 
    T: Component
{
    fn signature(fecs: &FECS) -> Bitset {
        // get the bits for the components, then bit or them        
        let type_id = TypeId::of::<T>();
        let h_bit_offset = fecs.get_c_signatures()
            .get(&type_id)
            .unwrap_or(&0);
        let mut h_bitset = Bitset::new(fecs.get_max_components() as usize);
        h_bitset.set(*h_bit_offset as usize);

        return h_bitset;
    }
}

impl <Head, Tail>Query for (Head, Tail) 
where 
    Head: Component,
    Tail: Query
{
    fn signature(fecs: &FECS) -> Bitset {
        // get the bits for the components, then bit or them        
        let type_id = TypeId::of::<Head>();
        let h_bit_offset = fecs.get_c_signatures()
            .get(&type_id)
            .unwrap_or(&0);
        let mut h_bitset = Bitset::new(fecs.get_max_components() as usize);
        h_bitset.set(*h_bit_offset as usize);

        return h_bitset | Tail::signature(fecs);
    }
}
