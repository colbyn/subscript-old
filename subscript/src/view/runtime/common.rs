use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi, CallbackSettings, QueueCallback, VoidCallback};
use crate::model::incremental::{IVecSub};
use crate::view::dsl::{self as dsl, Dsl, View, SubComponent};
use crate::view::shared::*;
use crate::view::dom::*;


pub(crate) struct ElementEnv<'a> {
	pub tag: &'a str,
	pub dom_ref: &'a browser::Element,
	pub rightward: &'a RefCell<Option<Box<browser::NodeApi>>>,
}

impl<'a> ElementEnv<'a> {
	pub(crate) fn get_rightward(&self) -> Option<Box<browser::NodeApi>> {
		let inner: &Option<Box<browser::NodeApi>> = &self.rightward.borrow();
		match inner {
			Some(x) => Some(x.box_clone()),
			None => None
		}
	}
}

pub(crate) fn insert_child<'a>(new: &browser::NodeApi, env: &ElementEnv<'a>) {
	if let Some(rightward) = env.get_rightward() {
		env.dom_ref.insert_before(new, rightward.as_ref());
	} else {
		env.dom_ref.append_child(new);
	}
}

impl<Msg> Dom<Msg> {
	pub(crate) fn remove<'a>(self, env: &ElementEnv<'a>) {
		match self {
			Dom::Component(value) => {
				env.dom_ref.remove_child(&value.dom_ref);
			}
			Dom::Text(value) => {
				env.dom_ref.remove_child(&value.dom_ref);
			}
			Dom::Element(value) => {
				let new_env = ElementEnv {
					tag: &value.tag,
					dom_ref: &value.dom_ref,
					rightward: &RefCell::new(None),
				};
				for event in value.events.iter() {
					env.dom_ref.remove_event_listener(&event.event_type(), &event.backend_callback);
				}
				for child in value.children {
					child.remove(&new_env);
				}
				env.dom_ref.remove_child(&value.dom_ref);
			}
			Dom::Mixin(value) => {
				for key in value.attributes.keys() {
					env.dom_ref.remove_attribute(key.as_str());
				}
				for event in value.events.iter() {
					env.dom_ref.remove_event_listener(&event.event_type(), &event.backend_callback);
				}
				for child in value.children {
					child.remove(env);
				}
			}
			Dom::Control(Control::Toggle(toggle)) => {
				let inner: Option<Dom<Msg>> = toggle.dom.into_inner();
				if let Some(dom) = inner {
					dom.remove(env);
				}
			}
			Dom::Control(Control::Linked(sub)) => {
				if let Right(dom) = sub.terminate() {
					for child in dom.removed {
						child.remove(env);
					}
					for child in dom.active {
						if let Right(dom) = child.into_inner() {
							dom.remove(env);
						}
					}
				}
			}
		}
	}
}


pub(crate) fn set_attribute<'a>(key: &String, value: &Either<Value<String>, Value<bool>>, element: &ElementEnv<'a>) {
	let attribute = |key: &String, value: &Either<Value<String>, Value<bool>>| match value {
		Left(Value::Static(value)) => {
			element.dom_ref.set_attribute(key.as_str(), value.as_str());
		}
		Left(Value::Dynamic(cell)) => {
			let value: &str = &cell.value.borrow();
			element.dom_ref.set_attribute(key.as_str(), value);
		}
		Right(Value::Static(value)) => {
			if value.clone() {
				element.dom_ref.set_attribute(key.as_str(), "");
			} else {
				element.dom_ref.remove_attribute(key.as_str());
			}
		}
		Right(Value::Dynamic(cell)) => {
			if cell.value.borrow().clone() {
				element.dom_ref.set_attribute(key.as_str(), "");
			} else {
				element.dom_ref.remove_attribute(key.as_str());
			}
		}
	};
	let property = |key: &String, value: &Either<Value<String>, Value<bool>>| {
		unimplemented!()
	};
	match (element.tag, key.as_str()) {
		("input", "value") => property(key, value),
		("input", "checked") => property(key, value),
		_ => attribute(key, value),
	}
}

pub(crate) fn update_attribute<'a>(key: &String, value: &Either<Value<String>, Value<bool>>, element: &ElementEnv<'a>) {
	let attribute = |key: &String, value: &Either<Value<String>, Value<bool>>| match value {
		Left(string) => {
			string.if_changed(|new_value| {
				element.dom_ref.set_attribute(key.as_str(), new_value.as_str());
			});
		}
		Right(boolean) => {
			boolean.if_changed(|new_value| {
				if new_value.clone() {
					element.dom_ref.set_attribute(key.as_str(), "");
				} else {
					element.dom_ref.remove_attribute(key.as_str());
				}
			});	
		}
	};
	let property = |key: &String, value: &Either<Value<String>, Value<bool>>| {
		unimplemented!()
	};
	match (element.tag, key.as_str()) {
		("input", "value") => property(key, value),
		("input", "checked") => property(key, value),
		_ => attribute(key, value),
	}
}

impl<Msg> Dom<Msg> {
	pub(crate) fn get_before_dom_ref(&self) -> Option<Box<browser::NodeApi>> {
		fn check_children<Msg>(children: &Vec<Dom<Msg>>) -> Option<Box<browser::NodeApi>> {
			let mut result: Option<Box<browser::NodeApi>> = None;
			for child in children.iter() {
				if result.is_none() {
					result = child.get_before_dom_ref();
				}
			}
			result
		}
		match self {
			Dom::Control(Control::Linked(sub)) => {
				let mut result: Option<Box<browser::NodeApi>> = None;
				sub.inspect_dom(&mut |view| {
					if result.is_none() {
						result = view.get_before_dom_ref();
					}
				});
				result
			}
			Dom::Control(Control::Toggle(toggle)) => {
				let mut result = None;
				if toggle.pred.value.borrow().clone() {
					let inner: &Option<Dom<Msg>> = &toggle.dom.borrow();
					assert!(inner.is_some());
					if let Some(dom) = inner {
						result = dom.get_before_dom_ref();
					}
				}
				result
			}
			Dom::Mixin(x) => check_children(&x.children),
			Dom::Component(x) => Some(x.dom_ref.box_clone()),
			Dom::Text(x) => Some(x.dom_ref.box_clone()),
			Dom::Element(x) => Some(x.dom_ref.box_clone()),
		}
	}
}

