use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::any::Any;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use either::Either;
use wasm_bindgen::JsValue;

use ss_web_utils::js;
use ss_web_utils::dom;


///////////////////////////////////////////////////////////////////////////////
// DOM EVENT-HANDLERS
///////////////////////////////////////////////////////////////////////////////

pub fn on_click<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnClick(OnClick(Rc::new(cb))))
}
pub fn on_mouse_down<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnMouseDown(OnMouseDown(Rc::new(cb))))
}
pub fn on_mouse_up<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnMouseUp(OnMouseUp(Rc::new(cb))))
}
pub fn on_mouse_enter<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnMouseEnter(OnMouseEnter(Rc::new(cb))))
}
pub fn on_mouse_leave<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnMouseLeave(OnMouseLeave(Rc::new(cb))))
}
pub fn on_mouse_over<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnMouseOver(OnMouseOver(Rc::new(cb))))
}
pub fn on_mouse_out<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnMouseOut(OnMouseOut(Rc::new(cb))))
}
pub fn on_input<Msg>(cb: impl Fn(String) -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnInput(OnInput(Rc::new(cb))))
}
pub fn on_check<Msg>(cb: impl Fn(bool) -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnCheck(OnCheck(Rc::new(cb))))
}
pub fn on_submit<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnSubmit(OnSubmit(Rc::new(cb))))
}
pub fn on_blur<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnBlur(OnBlur(Rc::new(cb))))
}
pub fn on_focus<Msg>(cb: impl Fn() -> Msg + 'static) -> EventHandler<Msg> {
    EventHandler(IEventHandler::OnFocus(OnFocus(Rc::new(cb))))
}


///////////////////////////////////////////////////////////////////////////////
// INTERNAL DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub struct EventHandlers<Msg>(pub(crate) BTreeMap<EventType, EventHandler<Msg>>);

#[derive(Debug, Clone, PartialEq)]
pub struct EventHandler<Msg>(pub(crate) IEventHandler<Msg>);

impl<Msg> EventHandler<Msg> {
    pub fn event_name(&self) -> EventType {
        self.0.event_name()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// DOM events enum.
pub enum EventType {
    OnClick,
    OnMouseDown,
    OnMouseUp,
    OnMouseEnter,
    OnMouseLeave,
    OnMouseOver,
    OnMouseOut,
    OnInput,
    OnCheck,
    OnSubmit,
    OnBlur,
    OnFocus,
}

impl EventType {
    /// Gets the event name as a string.
    pub fn as_str(&self) -> &str {
        match self {
            EventType::OnClick => "click",
            EventType::OnMouseDown => "mousedown",
            EventType::OnMouseUp => "mouseup",
            EventType::OnMouseEnter => "mouseenter",
            EventType::OnMouseLeave => "mouseenter",
            EventType::OnMouseOver => "mouseover",
            EventType::OnMouseOut => "mouseout",
            EventType::OnInput => "change",
            EventType::OnCheck => "click",
            EventType::OnSubmit => "submit",
            EventType::OnBlur => "blur",
            EventType::OnFocus => "focus",
        }
    }
}

#[derive(Clone)]
pub enum IEventHandler<Msg> {
    OnClick(OnClick<Msg>),
    OnMouseDown(OnMouseDown<Msg>),
    OnMouseUp(OnMouseUp<Msg>),
    OnMouseEnter(OnMouseEnter<Msg>),
    OnMouseLeave(OnMouseLeave<Msg>),
    OnMouseOver(OnMouseOver<Msg>),
    OnMouseOut(OnMouseOut<Msg>),
    OnInput(OnInput<Msg>),
    OnCheck(OnCheck<Msg>),
    OnSubmit(OnSubmit<Msg>),
    OnBlur(OnBlur<Msg>),
    OnFocus(OnFocus<Msg>),
}

impl<Msg> IEventHandler<Msg> {
    pub fn event_name(&self) -> EventType {
        match self {
            IEventHandler::OnClick(_) =>  EventType::OnClick,
            IEventHandler::OnMouseDown(_) =>  EventType::OnMouseDown,
            IEventHandler::OnMouseUp(_) =>  EventType::OnMouseUp,
            IEventHandler::OnMouseEnter(_) =>  EventType::OnMouseEnter,
            IEventHandler::OnMouseLeave(_) =>  EventType::OnMouseLeave,
            IEventHandler::OnMouseOver(_) =>  EventType::OnMouseOver,
            IEventHandler::OnMouseOut(_) =>  EventType::OnMouseOut,
            IEventHandler::OnInput(_) =>  EventType::OnInput,
            IEventHandler::OnCheck(_) =>  EventType::OnCheck,
            IEventHandler::OnSubmit(_) =>  EventType::OnSubmit,
            IEventHandler::OnBlur(_) =>  EventType::OnBlur,
            IEventHandler::OnFocus(_) =>  EventType::OnFocus,
        }
    }
}

impl<Msg> Hash for IEventHandler<Msg> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.event_name().hash(state);
    }
}
impl<Msg> PartialEq for IEventHandler<Msg> {
    fn eq(&self, other: &IEventHandler<Msg>) -> bool {
        self.event_name() == other.event_name()
    }
}
impl<Msg> Debug for IEventHandler<Msg> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IEventHandler")
    }
}



// MOUSE
#[derive(Clone)]
pub struct OnClick<Msg>(pub Rc<Fn()->Msg>);
#[derive(Clone)]
pub struct OnMouseDown<Msg>(pub Rc<Fn()->Msg>);
#[derive(Clone)]
pub struct OnMouseUp<Msg>(pub Rc<Fn()->Msg>);
#[derive(Clone)]
pub struct OnMouseEnter<Msg>(pub Rc<Fn()->Msg>);
#[derive(Clone)]
pub struct OnMouseLeave<Msg>(pub Rc<Fn()->Msg>);
#[derive(Clone)]
pub struct OnMouseOver<Msg>(pub Rc<Fn()->Msg>);
#[derive(Clone)]
pub struct OnMouseOut<Msg>(pub Rc<Fn()->Msg>);


// FORMS
#[derive(Clone)]
pub struct OnInput<Msg>(pub Rc<Fn(String)->Msg>);
#[derive(Clone)]
pub struct OnCheck<Msg>(pub Rc<Fn(bool)->Msg>);
#[derive(Clone)]
pub struct OnSubmit<Msg>(pub Rc<Fn()->Msg>);


// FOCUS
#[derive(Clone)]
pub struct OnBlur<Msg>(pub Rc<Fn()->Msg>);
#[derive(Clone)]
pub struct OnFocus<Msg>(pub Rc<Fn()->Msg>);


///////////////////////////////////////////////////////////////////////////////
// INTERNAL - NODE EVENT-HANDLERS
///////////////////////////////////////////////////////////////////////////////

impl<Msg: Ord> EventHandlers<Msg> {
    pub fn new() -> Self {
        EventHandlers(BTreeMap::new())
    }
    pub fn into_inner(self) -> Vec<EventHandler<Msg>> {
        let mut results = Vec::new();
        for (_, v) in self.0 {
            results.push(v);
        }
        results
    }
    pub fn add(&mut self, handler: EventHandler<Msg>) {
        self.0.insert(handler.event_name(), handler);
    }
}


///////////////////////////////////////////////////////////////////////////////
// INTERNAL - INSTANCES
///////////////////////////////////////////////////////////////////////////////

// ALL
impl<Msg> js::Handler<Msg> for &EventHandler<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0.handler(event)
    }
}
impl<Msg> js::Handler<Msg> for EventHandler<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0.handler(event)
    }
}
impl<Msg> js::Handler<Msg> for IEventHandler<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        match self {
            IEventHandler::OnClick(x) => x.handler(event),
            IEventHandler::OnMouseDown(x) => x.handler(event),
            IEventHandler::OnMouseUp(x) => x.handler(event),
            IEventHandler::OnMouseEnter(x) => x.handler(event),
            IEventHandler::OnMouseLeave(x) => x.handler(event),
            IEventHandler::OnMouseOver(x) => x.handler(event),
            IEventHandler::OnMouseOut(x) => x.handler(event),
            IEventHandler::OnInput(x) => x.handler(event),
            IEventHandler::OnCheck(x) => x.handler(event),
            IEventHandler::OnSubmit(x) => x.handler(event),
            IEventHandler::OnBlur(x) => x.handler(event),
            IEventHandler::OnFocus(x) => x.handler(event),
        }
    }
}

// MOUSE
impl<Msg> js::Handler<Msg> for OnClick<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseDown<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseUp<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseEnter<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseLeave<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseOver<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseOut<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}


// FORMS
impl<Msg> js::Handler<Msg> for OnInput<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0(dom::event::get_oninput_value(&event))
    }
}
impl<Msg> js::Handler<Msg> for OnCheck<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        let event: web_sys::Event = From::from(event.clone());
        let target: web_sys::EventTarget = event
            .target()
            .expect("target failed");
        let target: JsValue = From::from(target);
        let target: web_sys::HtmlInputElement = From::from(target);
        let value: bool = target.checked();
        self.0(value)
    }
}
impl<Msg> js::Handler<Msg> for OnSubmit<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}

// FOCUS
impl<Msg> js::Handler<Msg> for OnBlur<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnFocus<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}




