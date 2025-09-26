use std::any::Any;

pub trait Component: 'static + Any + Sized
{
}

pub use fecs_derive::Component;
