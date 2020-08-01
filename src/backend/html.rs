//! html backend where all the functionalities is offloaded into sauron
use crate::{widget::layout::compute_node_layout, Backend, Component};
use sauron::prelude::*;
use std::{fmt::Debug, marker::PhantomData};
use stretch::geometry::Size;
use stretch::number::Number;

mod convert_event;
mod convert_widget;

/// We wrap the App's Msg with this such that we can add high level behavior of the app
/// such as automatically computing the layout when the window is resized
#[derive(Clone)]
pub enum BackendMsg<MSG> {
    /// the Wrapped MSG meant for the app
    AppMsg(MSG),
    /// the app container is resized
    Resize(i32, i32),
}

/// holds the user application,
/// this just wraps the app, so we can implement the Component trait for it
pub struct HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    app: APP,
    browser_size: (i32, i32),
    _phantom_data: PhantomData<MSG>,
}

impl<APP, MSG> HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn new(app: APP) -> Self {
        let browser_size = Browser::get_size();
        HtmlApp {
            app,
            browser_size,
            _phantom_data: PhantomData,
        }
    }
}

impl<APP, MSG> sauron::Component<BackendMsg<MSG>> for HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn init(
        &self,
    ) -> sauron::cmd::Cmd<sauron::Program<Self, BackendMsg<MSG>>, BackendMsg<MSG>>
    {
        log::debug!("init in HtmlApp..");
        Browser::on_resize(BackendMsg::Resize)
    }

    fn update(
        &mut self,
        msg: BackendMsg<MSG>,
    ) -> sauron::cmd::Cmd<sauron::Program<Self, BackendMsg<MSG>>, BackendMsg<MSG>>
    {
        match msg {
            BackendMsg::AppMsg(msg) => {
                self.app.update(msg);
            }
            BackendMsg::Resize(w, h) => {
                log::debug!("window is resizing..");
                self.browser_size = (w, h);
            }
        }
        sauron::cmd::Cmd::none()
    }

    fn view(&self) -> sauron::Node<BackendMsg<MSG>> {
        let t1 = sauron::now();

        let mut view = self.app.view();
        let (w, h) = self.browser_size;
        let (adjusted_w, adjusted_h) = (w as f32 - 100.0, h as f32 - 20.0);
        compute_node_layout(
            &mut view,
            Size {
                width: Number::Defined(adjusted_w),
                height: Number::Defined(adjusted_h),
            },
        );

        let t2 = sauron::now();
        log::warn!("layout computation took: {}ms", t2 - t1);

        let html_view = convert_widget::widget_tree_to_html_node(&view, &mut 0);
        html_view.map_msg(BackendMsg::AppMsg)
    }
}

impl<APP, MSG> Backend<APP, MSG> for HtmlApp<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn init(app: APP) {
        log::trace!("Html app started..");
        let html_app = HtmlApp::new(app);
        sauron::Program::mount_to_body(html_app);
    }
}
