pub mod byte;
pub mod line;
pub mod word;

pub trait Counter {
    type State: 'static;
    type Output;

    fn parse(&self, input: &[u8], state: Self::State) -> Self::State;
    fn terminate(&self, state: Self::State) -> Self::Output;
}

pub trait Collapse<Collapsable> {
    fn collapse(self, colapsable: Collapsable) -> Collapsable;
}

#[cfg(feature = "tower")]
impl Counter for tower_layer::Identity {
    type State = ();

    type Output = ();

    fn parse(&self, _: &[u8], _: Self::State) -> Self::State {}

    fn terminate(&self, _: Self::State) -> Self::Output {}
}

#[cfg(feature = "anymap")]
impl Collapse<anymap::AnyMap> for () {
    fn collapse(self, colapsable: anymap::AnyMap) -> anymap::AnyMap {
        colapsable
    }
}
