use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum AttributeValue {
    Value(String),
    Toggle(bool),
}

pub trait AttributeValueInterface {
	fn to_attribute(self) -> AttributeValue;
}
impl AttributeValueInterface for String {
	fn to_attribute(self) -> AttributeValue {
		AttributeValue::Value(self)
	}
}
impl AttributeValueInterface for &str {
	fn to_attribute(self) -> AttributeValue {
		AttributeValue::Value(String::from(self))
	}
}
impl AttributeValueInterface for bool {
	fn to_attribute(self) -> AttributeValue {
		AttributeValue::Toggle(self)
	}
}

pub fn internal_normalize_attribute_value(x: impl AttributeValueInterface) -> AttributeValue {
	x.to_attribute()
}





