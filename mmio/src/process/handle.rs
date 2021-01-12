
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum HandleType {
    NONE = 0,
    TIMER,
    FILE,
    SOCKET,
    IR,
}

#[derive(Copy, Clone)]
pub struct Handle {
    pub handle_type: HandleType,
}

pub struct TimerHandle {
    handler: extern "C" fn() -> !,
    period: usize // in second
}

impl TimerHandle {

    pub fn new(handler : extern "C" fn() -> !, period : usize) -> Self {
        TimerHandle {
            handler,
            period
        }
    }
}
