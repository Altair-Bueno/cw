pub mod byte;
pub mod line;
pub mod word;

pub trait Counter<Input> {
    type State: 'static;
    type Output;

    fn parse(&self, input: Input, state: Self::State) -> Self::State;
    fn terminate(&self, state: Self::State) -> Self::Output;
}

pub trait Collapse<Collapsable> {
    fn collapse(self, colapsable: Collapsable) -> Collapsable;
}

#[cfg(feature = "tower")]
impl<ANY> Counter<ANY> for tower_layer::Identity {
    type State = ();

    type Output = ();

    fn parse(&self, _: ANY, _: Self::State) -> Self::State {
        
    }

    fn terminate(&self, _: Self::State) -> Self::Output {
        
    }
}

#[cfg(feature = "anymap")]
impl Collapse<anymap::AnyMap> for () {
    fn collapse(self, colapsable: anymap::AnyMap) -> anymap::AnyMap {
        colapsable
    }
}
