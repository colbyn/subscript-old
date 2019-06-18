use std::collections::*;
use std::cell::*;
use std::marker::*;
use std::any::*;
use std::rc::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::backend::browser;
use crate::view_sys::dsl::View;
use crate::program_sys::instances::TickEnv;
use crate::program_sys::shell::*;
use crate::program_sys::effect::nav::Url;

pub use crate::program_sys::shell::{Shell};
pub use crate::program_sys::effect::nav::{UrlPath, UrlParser, UrlChanged};
pub use crate::program_sys::effect::sub::Subscriptions;


pub trait Spec where Self: Clone {
	type Msg;
    type Model;
	
	fn init(&self, startup: StartupInfo<Self>) -> Init<Self>;
	fn update(&self, model: &mut Self::Model, msg: Self::Msg, sh: &mut Shell<Self>);
	fn view(&self, model: &Self::Model) -> View<Self::Msg>;
}


pub struct StartupInfo<S: Spec> {
	pub saved_model: Option<S::Model>,
    pub current_url: Url,
}

pub struct Init<S: Spec> {
	pub model: S::Model,
	pub subs: Subscriptions<S::Msg>
}

impl<S: Spec> Default for Init<S>
where
    S::Model: Default,
{
    fn default() -> Self {
        Init {
            model: Default::default(),
            subs: Subscriptions::default(),
        }
    }
}





