// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

mod generated;
mod page;
mod urls;
mod utility;

use crate::urls::Urls;
use generated::css_classes::C;
use seed::{prelude::*, *};

// -------------------------
//          Init
// -------------------------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // 订阅消息，把subs的URL改变的消息反射到自己定义的Msg结构，
    // 后面update，使用Msg::UrlChanged(subs::UrlChanged(url)) =>的形式。
    orders.subscribe(Msg::UrlChanged).notify(subs::UrlChanged);

    // 初始化一个Model，默认是root，
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url, orders),
    }
}

struct Model {
    base_url: Url,
    page: Page,
}

/// 在这里添加界面
enum Page {
    Home,
    Html1(page::html1::Model),
    NotFound,
}

impl Page {
    /// 初始化page，按照当前url字符串信息，返回相应的界面
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            ["html1"] => Self::Html1(page::html1::init(
                url,
                &mut orders.proxy(Msg::Html1Url),
            )),
            _ => Self::NotFound,
        }
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // 根据url的改变信息，返回响应的page。
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url, orders);
        }

        // 处理子page的msg。
        Msg::Html1Url(msg) => {
            // 枚举模式匹配
            if let Page::Html1(model) = &mut model.page {
                // 调用html1子page的update。把消息反馈到传递到子page。
                page::html1::update(
                    msg,
                    model,
                    &mut orders.proxy(Msg::Html1Url),
                );
            }
        }
    }
}

enum Msg {
    UrlChanged(subs::UrlChanged),
    Html1Url(page::html1::Msg),
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    let mut ret = Vec::new();
    //ret.push(view_navbar(model));
    ret.extend_from_slice(&view_content(&model.page));
    ret
}

/// 处理界面的渲染，根据不用的子page，调用子page的view。
fn view_content(page: &Page) -> Vec<Node<Msg>> {
    match page {
        Page::Home => page::home::view(),
        Page::Html1(model) => page::html1::view(model).map_msg(Msg::Html1Url),
        Page::NotFound => vec![div![C![C.h_2], "没有找到网页"]],
    }
}

fn view_navbar(model: &Model) -> Node<Msg> {
    div![
        h1!["导航  "],
        a![
            C![C.h_5],
            attrs! { At::Href => Urls::new(&model.base_url).home() },
            "|Home"
        ],
        a![
            C![C.h_5],
            attrs! { At::Href => Urls::new(&model.base_url).html1() },
            "|Html1"
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::start("app", init, update, view);

    log!("App started.");
}
