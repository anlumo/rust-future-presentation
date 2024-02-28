#![allow(dead_code, unused_variables, unused_mut)]

use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct ImageLoader {
    url: String,
}

impl ImageLoader {
    pub fn load(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}

impl Future for ImageLoader {
    type Output = Result<HtmlImageElement, JsValue>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        std::task::Poll::Pending
    }
}
