use crate::event::{
    DragEvent, DropEvent, KeyEvent, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ScrollEvent,
    TextClipboardEvent, TextInputEvent, TimerEvent, WindowCloseRequestedEvent, WindowClosedEvent,
    WindowDragQueryEvent, WindowGeomChangeEvent,
};

#[derive(Debug)]
pub enum XlibEvent {
    AppGotFocus,
    AppLostFocus,
    WindowGeomChange(WindowGeomChangeEvent),
    WindowClosed(WindowClosedEvent),
    Paint,

    MouseDown(MouseDownEvent),
    MouseUp(MouseUpEvent),
    MouseMove(MouseMoveEvent),
    Scroll(ScrollEvent),

    WindowDragQuery(WindowDragQueryEvent),
    WindowCloseRequested(WindowCloseRequestedEvent),
    TextInput(TextInputEvent),
    Drag(DragEvent),
    Drop(DropEvent),
    DragEnd,
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    TextCopy(TextClipboardEvent),
    TextCut(TextClipboardEvent),
    Timer(TimerEvent),
}
