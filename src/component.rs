use crate::FECS;

pub trait Component: 'static + Sized /* + Send + Sync */ {
    fn register(fecs: &mut FECS) {
        fecs.register_component::<Self>();
    }
}

impl Component for u32 {}
impl Component for u64 {}
impl Component for f32 {}
impl Component for f64 {}
impl Component for i32 {}
impl Component for i64 {}
