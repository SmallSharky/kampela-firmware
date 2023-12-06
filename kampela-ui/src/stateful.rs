



pub trait Stateful {
    type StateType;

    fn get_state(&self) -> Self::StateType;
}