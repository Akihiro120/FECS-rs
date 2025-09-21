use std::any::Any;

pub trait Component: 'static + Any + Sized
{
}
