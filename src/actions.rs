
#[derive(Debug)]
pub enum Action {
    Init,
    Exit,
    Tick,
    NoOp,
    Multi(Vec<Action>),

    // SwitchScreen(Screen),

}
