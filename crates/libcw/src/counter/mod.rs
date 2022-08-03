use anymap::AnyMap;
use tower_layer::Identity;

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

impl<ANY> Counter<ANY> for Identity {
    type State = ();

    type Output = ();

    fn parse(&self, _: ANY, _: Self::State) -> Self::State {
        ()
    }

    fn terminate(&self, _: Self::State) -> Self::Output {
        ()
    }
}

impl Collapse<AnyMap> for () {
    fn collapse(self, colapsable: AnyMap) -> AnyMap {
        colapsable
    }
}
