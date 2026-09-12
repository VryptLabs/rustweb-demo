use rustweb_core::{Component, Context, Props, RenderError, VNode};
use rustweb_macro::html;
use rustweb_router::{Route, Router};
use rustweb_ui::{render_button, render_dialog, render_input, render_tabs, ButtonProps, ButtonVariant, DialogProps, InputProps, TabsProps};

#[derive(Clone, PartialEq, Debug)]
struct Task {
    id: usize,
    title: String,
    done: bool,
}

#[derive(Clone, PartialEq, Debug)]
struct AppProps {
    tasks: Vec<Task>,
    filter: usize,
    dialog_open: bool,
    draft: String,
    children: Vec<VNode>,
}
impl Props for AppProps {}

struct App {
    tasks: Vec<Task>,
    next_id: usize,
    filter: usize,
    dialog_open: bool,
    draft: String,
}

#[derive(Debug)]
enum Msg {
    Toggle(usize),
    SetDraft(String),
    Add,
    OpenDialog,
    CloseDialog,
    SetFilter(usize),
}

impl Component for App {
    type Props = AppProps;
    type Msg = Msg;

    fn create(ctx: &Context<Self>) -> Result<Self, rustweb_core::ComponentError> {
        Ok(Self {
            tasks: ctx.props.tasks.clone(),
            next_id: ctx.props.tasks.len(),
            filter: ctx.props.filter,
            dialog_open: ctx.props.dialog_open,
            draft: ctx.props.draft.clone(),
        })
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> Result<rustweb_core::Cmd<Self>, rustweb_core::ComponentError> {
        match msg {
            Msg::Toggle(id) => {
                if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
                    t.done = !t.done;
                }
            }
            Msg::SetDraft(s) => self.draft = s,
            Msg::Add => {
                let title = self.draft.trim().to_owned();
                if !title.is_empty() {
                    self.tasks.push(Task { id: self.next_id, title, done: false });
                    self.next_id += 1;
                    self.draft.clear();
                    self.dialog_open = false;
                }
            }
            Msg::OpenDialog => self.dialog_open = true,
            Msg::CloseDialog => self.dialog_open = false,
            Msg::SetFilter(i) => self.filter = i,
        }
        Ok(rustweb_core::Cmd::Render)
    }

    fn view(&self, _ctx: &Context<Self>) -> Result<VNode, RenderError> {
        let filtered: Vec<&Task> = self.tasks.iter().filter(|t| match self.filter {
            1 => !t.done,
            2 => t.done,
            _ => true,
        }).collect();

        let rows: Vec<VNode> = filtered.iter().map(|t| html! {
            <li key={ t.id } aria-selected={ t.done }>
                <span>{ t.title.clone() }</span>
                <span>{ if t.done { " ✓" } else { "" } }</span>
            </li>
        }).collect();

        let input = render_input(InputProps {
            id: "new-task".into(),
            value: self.draft.clone(),
            placeholder: Some("What needs to be done?".into()),
            label: Some("New task".into()),
            error: None,
            disabled: false,
            children: vec![],
        });

        let tabs = render_tabs(TabsProps {
            tabs: vec!["All".into(), "Active".into(), "Done".into()],
            selected: self.filter,
            children: vec![
                html! { <ul>{ rows.clone() }</ul> },
                html! { <div>{ VNode::text(format!("{} active", self.tasks.iter().filter(|t| !t.done).count())) }</div> },
                html! { <div>{ VNode::text(format!("{} done", self.tasks.iter().filter(|t| t.done).count())) }</div> },
            ],
        });

        let add_btn = render_button(ButtonProps {
            label: "Add task".into(),
            variant: ButtonVariant::Primary,
            disabled: false,
            loading: false,
            aria_label: None,
            children: vec![],
        });

        let dialog = render_dialog(DialogProps {
            open: self.dialog_open,
            title: "Add task".into(),
            children: vec![input.clone()],
        });

        let stats = format!("{} tasks · {} done", self.tasks.len(), self.tasks.iter().filter(|t| t.done).count());

        Ok(html! {
            <div class="rw-shell">
                <aside class="rw-side">
                    <h2>rustweb demo</h2>
                    <nav class="rw-nav" aria-label="Primary">
                        <a href="/" aria-current="page">Board</a>
                        <a href="/settings">Settings</a>
                    </nav>
                </aside>
                <main class="rw-main" aria-label="TaskBoard">
                    <div class="rw-card">
                        <h1>{ "TaskBoard" }</h1>
                        <p>{ stats }</p>
                        { tabs }
                        <div style="display:flex;gap:8px;margin-top:12px">
                            { add_btn }
                            <button class="rw-btn rw-btn--secondary">{ "New" }</button>
                        </div>
                        { dialog }
                    </div>
                </main>
            </div>
        })
    }
}

fn router() -> Router {
    Router::new()
        .route(Route::new("", "Board"))
        .route(Route::new("settings", "Settings"))
        .route(Route::new("settings/profile", "Profile"))
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn start() {
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let mount = doc.get_element_by_id("app").unwrap();
        let tasks = vec![
            Task { id: 0, title: "Buy milk".into(), done: false },
            Task { id: 1, title: "Write docs".into(), done: true },
            Task { id: 2, title: "Ship v1.1".into(), done: false },
        ];
        let props = AppProps { tasks, filter: 0, dialog_open: false, draft: "".into(), children: vec![] };
        let link = rustweb_core::Link::new(|_: Msg| {});
        let ctx = Context::new(props, link, Default::default());
        let app = App::create(&ctx).unwrap();
        let vnode = app.view(&ctx).unwrap();
        let html = rustweb_ssr::render_to_string(&vnode);
        mount.set_inner_html(&html);
        web_sys::console::log_1(&"rustweb demo mounted".into());
        let _ = router();
    }
}

fn main() {
    let tasks = vec![
        Task { id: 0, title: "Buy milk".into(), done: false },
        Task { id: 1, title: "Write docs".into(), done: true },
    ];
    let props = AppProps { tasks, filter: 0, dialog_open: false, draft: "".into(), children: vec![] };
    let link = rustweb_core::Link::new(|_: Msg| {});
    let ctx = Context::new(props, link, Default::default());
    let app = App::create(&ctx).unwrap();
    let vnode = app.view(&ctx).unwrap();
    let html = rustweb_ssr::render_to_string(&vnode);
    println!("{html}");
    rustweb_ssr::verify_hydration(&html, &vnode).unwrap();
    println!("router / -> {:?}", router().resolve("/").unwrap().view);
}
