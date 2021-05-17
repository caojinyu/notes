use crate::generated::css_classes::C;
use seed::{prelude::*, *};

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        value: 0,
    }
}

pub struct Model {
    value: i32,
}

pub enum Msg {
    AddOne,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddOne => {
            model.value += 1;
        }
    }
}

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![div![
        C![C.p_2],
        "这是Html1:",
        input![
            C![C.h_4],
            attrs! { At::Value => model.value },
            ev(Ev::Click, |_event| { Msg::AddOne }),
        ]
    ]]
}
