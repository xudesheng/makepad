use crate::event::{TimerEvent, WindowGeomChangeEvent};

#[derive(Debug, Clone)]
pub enum TvosEvent {
    Init,
    AppGotFocus,
    AppLostFocus,
    WindowGeomChange(WindowGeomChangeEvent),
    Paint,
    Timer(TimerEvent),
}
