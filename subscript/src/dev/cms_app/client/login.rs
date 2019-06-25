use std::marker::*;
use std::rc::*;
use std::collections::*;
use std::any::*;
use serde::{Serialize, Deserialize};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi};
use crate::reactive_sys::*;
use crate::view_sys::runtime::common::ElementEnv;
use crate::view_sys::shared::*;
use crate::view_sys::{dom, dsl, runtime, dom::{Dom, Element}, dsl::{View, Dsl}};
use crate::view_sys::adapters::*;
use crate::program_sys::instances::Component;
use crate::program_sys::spec::*;
use crate::program_sys::{self, Program};

use crate::dev::cms_app::client::data::*;
use crate::dev::cms_app::client::ui_utils::{self, text_theme};


///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct LoginSpec {}

pub enum Msg {
    NoOp,
    Input {
        to: FormField,
        value: String,
    }
}

#[derive(Default)]
pub struct Model {
    name: String,
    password: String,
}

#[derive(Clone)]
pub enum FormField {
    Name,
    Password,
}

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
// VIEWABLE DATA TYPES
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Spec for LoginSpec {
    type Msg = Msg;
    type Model = Model;

    fn init(&self, startup: &Shell<Self>) -> Init<Self> {
        Default::default()
    }
    fn update(&self, model: &mut Model, msg: Msg, sh: &mut Shell<Self>) {
        match msg {
            Msg::NoOp => {}
            Msg::Input{to, value} => {
                match to {
                    FormField::Name => {
                        model.name = value;
                    }
                    FormField::Password => {
                        model.password = value;
                    }
                }
            }
        }
    }
    fn view(&self, model: &Model) -> View<Msg> {
        let name_id: String = format!("id-{}", rand::random::<u16>());
        let password_id: String = format!("id-{}", rand::random::<u16>());
        v1!{
            h1 !{
                "LoginSpec";
            };
            form !{
                // NAME
                div !{
                    label !{
                        text_theme();
                        for = name_id.clone();
                        "Name";
                    };
                    input !{
                        text_theme();
                        id = name_id;
                    };
                };
                // PASSWORD
                div !{
                    label !{
                        text_theme();
                        for = password_id.clone();
                        "Password";
                    };
                    input !{
                        text_theme();
                        id = password_id;
                    };
                };
            };
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// VIEW HELPERS
///////////////////////////////////////////////////////////////////////////////


