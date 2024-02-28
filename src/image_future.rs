use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

#[allow(dead_code)]
struct ImageData {
    image: HtmlImageElement,
    resolve: Closure<dyn FnMut()>,
    reject: Closure<dyn FnMut(JsValue)>,
}

pub struct ImageLoader {
    url: String,
    data: Option<ImageData>,
    error: Rc<RefCell<Option<JsValue>>>,
}

impl ImageLoader {
    pub fn load(url: &str) -> Self {
        Self {
            url: url.to_owned(),
            data: None,
            error: Rc::default(),
        }
    }
}

impl Future for ImageLoader {
    type Output = Result<HtmlImageElement, JsValue>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(ImageData { image, .. }) = self.data.take() {
            if image.complete() {
                if let Some(err) = self.error.borrow_mut().take() {
                    return std::task::Poll::Ready(Err(err));
                } else {
                    return std::task::Poll::Ready(Ok(image));
                }
            }
        } else {
            let waker = cx.waker().clone();
            let resolve = Closure::once(move || {
                waker.wake();
            });
            let waker = cx.waker().clone();
            let error = Rc::downgrade(&self.error);
            let reject = Closure::once(move |err: JsValue| {
                if let Some(error) = error.upgrade() {
                    *error.borrow_mut() = Some(err);
                }
                waker.wake();
            });

            let image = HtmlImageElement::new().unwrap();
            image.set_onload(Some(resolve.as_ref().unchecked_ref()));
            image.set_onerror(Some(reject.as_ref().unchecked_ref()));
            image.set_src(&self.url);
            self.data = Some(ImageData {
                image,
                resolve,
                reject,
            });
        }
        std::task::Poll::Pending
    }
}
