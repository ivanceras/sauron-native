use sauron_native::widget::attribute::*;
use sauron_native::widget::event::*;
use sauron_native::widget::*;
use sauron_native::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    entries: Vec<Entry>,
    visibility: Visibility,
    value: String,
    uid: usize,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
    id: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Visibility {
    All,
    Active,
    Completed,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Msg {
    Add,
    EditingEntry(usize, bool),
    Update(String),
    UpdateEntry(usize, String),
    Delete(usize),
    ChangeVisibility(Visibility),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    NoOp,
}

impl Component<Msg> for Model {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Add => {
                self.entries.push(Entry::new(&self.value, self.uid));
                self.uid += 1;
                self.value = "".to_string();
            }
            Msg::EditingEntry(id, is_editing) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.editing = is_editing;
                    }
                });
            }
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::UpdateEntry(id, new_description) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.description = new_description.clone();
                    }
                });
            }
            Msg::Delete(id) => {
                self.entries.retain(|entry| entry.id != id);
            }
            Msg::ChangeVisibility(visibility) => {
                self.visibility = visibility;
            }
            Msg::ToggleEdit(id) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.editing = !entry.editing;
                    }
                });
            }
            Msg::ToggleAll => {
                let is_all_completed = !self.is_all_completed();
                self.entries
                    .iter_mut()
                    .for_each(|entry| entry.completed = is_all_completed);
            }
            Msg::Toggle(id) => {
                self.entries.iter_mut().for_each(|entry| {
                    if entry.id == id {
                        entry.completed = !entry.completed;
                        log::debug!(
                            "Toggle entry: {} to {}",
                            entry.id,
                            entry.completed,
                        );
                    }
                });
            }
            Msg::ClearCompleted => {
                self.entries.retain(|entry| !entry.completed);
            }
            Msg::NoOp => {}
        }
    }

    fn view(&self) -> Node<Msg> {
        column(
            vec![name("todomvc-wrapper")],
            vec![
                column(
                    vec![name("todoapp")],
                    vec![
                        self.view_input(),
                        self.view_entries(),
                        self.view_controls(),
                    ],
                ),
                self.info_footer(),
            ],
        )
    }
}

impl Entry {
    fn new(description: &str, id: usize) -> Self {
        Entry {
            description: description.to_string(),
            completed: false,
            editing: false,
            id,
        }
    }
}

impl Model {
    pub fn new() -> Self {
        Model {
            entries: vec![],
            visibility: Visibility::All,
            value: "".into(),
            uid: 0,
        }
    }

    fn is_all_completed(&self) -> bool {
        self.entries.iter().all(|entry| entry.completed)
    }
    fn view_entries(&self) -> Node<Msg> {
        column(
            vec![name("main")],
            vec![
                checkbox(vec![
                    name("toggle-all"),
                    checked(self.is_all_completed()),
                    on_click(|_| Msg::ToggleAll),
                ]),
                column(vec![name("todo-list")], {
                    self.entries
                        .iter()
                        .filter(|entry| match self.visibility {
                            Visibility::All => true,
                            Visibility::Active => !entry.completed,
                            Visibility::Completed => entry.completed,
                        })
                        .map(|entry| self.view_entry(entry))
                        .collect::<Vec<Node<Msg>>>()
                }),
            ],
        )
    }

    fn view_filter(&self, visibility: Visibility) -> Node<Msg> {
        let visibility_str = visibility.to_string();
        column(
            vec![],
            vec![link(vec![
                name(if self.visibility == visibility {
                    "selected"
                } else {
                    "not-selected"
                }),
                uri(visibility.to_uri()),
                on_click(move |_| Msg::ChangeVisibility(visibility.clone())),
                label(visibility_str),
            ])],
        )
    }

    fn view_input(&self) -> Node<Msg> {
        column(
            vec![],
            vec![
                text_label(vec![value("todos")]),
                text_input(vec![
                    name("new-todo"),
                    placeholder("What needs to be done?"),
                    value(self.value.to_string()),
                    on_input(|v: InputEvent| Msg::Update(v.value.to_string())),
                    on_keypress(|event: KeyEvent| {
                        if event.key_code == KeyCode::Enter {
                            Msg::Add
                        } else {
                            Msg::NoOp
                        }
                    }),
                ]),
            ],
        )
    }

    fn view_entry(&self, entry: &Entry) -> Node<Msg> {
        let mut class_name = "todo".to_string();
        if entry.editing {
            class_name.push_str(" editing");
        }
        if entry.completed {
            class_name.push_str(" completed");
        }
        let entry_id = entry.id;
        row(
            vec![name(class_name), key(format!("todo-{}", entry.id))],
            vec![
                row(
                    vec![name("view")],
                    vec![
                        checkbox(vec![
                            name("toggle"),
                            checked(entry.completed),
                            on_click(move |_| Msg::Toggle(entry_id)),
                        ]),
                        text_label(vec![
                            on_doubleclick(move |_| Msg::ToggleEdit(entry_id)),
                            value(format!("{}", entry.description)),
                        ]),
                        button(vec![
                            name("destroy"),
                            label("x"),
                            on_click(move |_| Msg::Delete(entry_id)),
                        ]),
                    ],
                ),
                text_input(vec![
                    name("edit"),
                    value(&entry.description),
                    on_input(move |input: InputEvent| {
                        Msg::UpdateEntry(entry_id, input.value.to_string())
                    }),
                    on_blur(move |_| Msg::EditingEntry(entry_id, false)),
                    on_keypress(move |event: KeyEvent| {
                        if event.key_code == KeyCode::Enter {
                            Msg::EditingEntry(entry_id, false)
                        } else {
                            Msg::NoOp
                        }
                    }),
                ]),
            ],
        )
    }

    fn view_controls(&self) -> Node<Msg> {
        let entries_completed =
            self.entries.iter().filter(|entry| entry.completed).count();

        let entries_left = self.entries.len() - entries_completed;
        let item = if entries_left == 1 { " item" } else { " items" };

        row(
            vec![name("footer")],
            vec![
                row(
                    vec![name("todo-count")],
                    vec![
                        text_label(vec![value(entries_left.to_string())]),
                        text_label(vec![value(format!(" {} left", item))]),
                    ],
                ),
                row(
                    vec![name("filters")],
                    vec![
                        self.view_filter(Visibility::All),
                        self.view_filter(Visibility::Active),
                        self.view_filter(Visibility::Completed),
                    ],
                ),
                button(vec![
                    name("clear-completed"),
                    on_click(|_| Msg::ClearCompleted),
                    label(format!("Clear completed ({})", entries_completed)),
                ]),
            ],
        )
    }

    fn info_footer(&self) -> Node<Msg> {
        column(
            vec![name("info")],
            vec![
                text_label(vec![value("Double-click to edit a todo")]),
                column(
                    vec![],
                    vec![
                        text_label(vec![value("Written by ")]),
                        link(vec![
                            uri("https://github.com/ivanceras/"),
                            label("Jovansonlee Cesar".to_string()),
                        ]),
                    ],
                ),
                column(
                    vec![],
                    vec![
                        text_label(vec![value("Part of ")]),
                        link(vec![
                            uri("http://todomvc.com/"),
                            label("TodoMVC"),
                        ]),
                    ],
                ),
            ],
        )
    }
}

impl ToString for Visibility {
    fn to_string(&self) -> String {
        match self {
            Visibility::All => "All".to_string(),
            Visibility::Active => "Active".to_string(),
            Visibility::Completed => "Completed".to_string(),
        }
    }
}

impl Visibility {
    fn to_uri(&self) -> String {
        match self {
            Visibility::All => "#/".to_string(),
            Visibility::Active => "#/active".to_string(),
            Visibility::Completed => "#/completed".to_string(),
        }
    }
}
