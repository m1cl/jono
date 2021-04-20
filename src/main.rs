#![recursion_limit = "10000"]
use keyframe::{ease, functions::EaseInOut};
use vgtk::ext::*;
use vgtk::lib::gdk_pixbuf::Pixbuf;
use vgtk::lib::gio::{ActionExt, ApplicationFlags, Cancellable, MemoryInputStream, SimpleAction};
use vgtk::lib::glib::Bytes;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, run, Callback, Component, UpdateAction, VNode};

mod weechat;

static THUMBNAIL: &[u8] = include_bytes!("../wysiwyg.png");

static WIN_HEIGHT: i32 = 800;
static WIN_WIDTH: i32 = 600;

pub struct AboutDialog {
    thumbnail: Pixbuf,
}

impl Default for AboutDialog {
    fn default() -> Self {
        let data_stream = MemoryInputStream::new_from_bytes(&Bytes::from_static(THUMBNAIL));
        let thumbnail =
            Pixbuf::new_from_stream(&data_stream, None as Option<&Cancellable>).unwrap();
        AboutDialog { thumbnail }
    }
}

impl AboutDialog {
    #[allow(unused_must_use)]
    fn run() {
        vgtk::run_dialog::<AboutDialog>(vgtk::current_window().as_ref());
    }
}
impl Component for AboutDialog {
    type Message = ();
    type Properties = ();

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Dialog::new_with_buttons(
                Some("About the tuber app"),
                None as Option<&Window>,
                DialogFlags::MODAL,
                &[("Ok", ResponseType::Ok)]
                )>
                <Box spacing=10 orientation=Orientation::Vertical>
                    <Image pixbuf=Some(self.thumbnail.clone())></Image>
                    <Label markup="<big><b> my tuber application </b></big>" />
                    <Label markup="made with <a href=\"littleendianrecords\" />" />
                </Box>
            </Dialog>
        }
    }
}
#[derive(Debug, Clone, Default)]
struct Radio {
    labels: &'static [&'static str],
    active: usize,
    on_changed: Callback<usize>,
}

#[derive(Clone, Debug)]
pub enum RadioAction {
    Changed(usize),
}

impl Component for Radio {
    type Message = RadioAction;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }
    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            RadioAction::Changed(index) => {
                self.on_changed.send(index);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box spacing=10>
                {
                    self.labels.iter().enumerate().map(|(index,label)| gtk! {
                        <ToggleButton label={*label} active={index == self.active}
                            on toggled=|_| RadioAction::Changed(index)
                        />
                    })
                }
            </Box>
        }
    }
}

#[derive(Clone, Debug)]
struct Task {
    text: String,
    done: bool,
}

impl Task {
    fn new<S: ToString>(text: S, done: bool) -> Self {
        Self {
            text: text.to_string(),
            done,
        }
    }

    fn label(&self) -> String {
        if self.done {
            format!(
                "<span strikethrough=\"true\" alpha=\"50%\">{}</span>",
                self.text
            )
        } else {
            self.text.clone()
        }
    }

    fn render(&self, index: usize) -> VNode<Model> {
        gtk! {
            <ListBoxRow>
                <Box>
                    <CheckButton active=self.done on toggled=|_| Action::Toggle {index}/>
                    <Label label=self.label() use_markup=true />
                    <Button Box::pack_type=PackType::End
                        relief=ReliefStyle::None
                        // label="Delete" // text
                        image="edit-delete"
                        on clicked=|_| Action::Delete {index}
                    />
                </Box>
            </ListBoxRow>
        }
    }
}

#[derive(Clone, Debug)]
struct Model {
    tasks: Vec<Task>,
    // tells you which task should be visible
    // that could improved by using an enum for clarity
    filter: usize,
    height: i32,
    width: i32,
}

#[derive(Clone, Debug)]
enum Action {
    About,
    ChangeWindowSize,
    Exit,
    Toggle { index: usize },
    Add { task: String },
    Delete { index: usize },
    Filter { filter: usize },
    Cleanup,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task::new("Rock/Pop", true),
                Task::new("Post Punk", true),
                Task::new("New Wave", false),
                Task::new("metal", false),
            ],
            filter: 0,
            width: 6,
            height: 8,
        }
    }
}

impl Model {
    fn filter_task(&self, task: &Task) -> bool {
        // for now this is o k. But an enum would be better,
        // because it would make it cleaner. E.g:
        // Filter::ALL, Filter::COMPLETED, Filter::ACTIVE
        // I tried this before but I got some error. Their is an
        // example online on the vgtk github page.
        match self.filter {
            0 => true,
            1 => !task.done,
            2 => task.done,
            _ => unreachable!(),
        }
    }
    fn count_comleted(&self) -> usize {
        self.tasks.iter().filter(|task| task.done).count()
    }

    fn animate_window(&mut self) {
        let animation = ease(EaseInOut, 700.0, 200.0, 0.5);
        self.height = animation as i32;
        self.height = animation as i32;
        println!("{:?}", animation);
    }
    fn items_left(&self) -> String {
        let left = self.count_comleted();
        let plural = if left == 1 { "item" } else { "items" };
        format!("{} {} left", left, plural)
    }
}
impl Component for Model {
    type Message = Action;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Action::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Action::Toggle { index } => {
                self.tasks[index].done = !self.tasks[index].done;
                UpdateAction::Render
            }
            Action::Add { task } => {
                self.tasks.push(Task::new(task, false));
                UpdateAction::Render
            }
            Action::Delete { index } => {
                self.tasks.remove(index);
                UpdateAction::Render
            }
            Action::Filter { filter } => {
                self.filter = filter;

                self.animate_window();
                UpdateAction::Render
            }
            Action::Cleanup => {
                self.tasks.retain(|task| !task.done);
                UpdateAction::Render
            }
            Action::About => {
                AboutDialog::run();
                UpdateAction::None
            }
            Action::ChangeWindowSize => {
                self.height = 3;
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        // let wee = weechat::new("m1cl");
        let main_menu = vgtk::menu()
            .section(vgtk::menu().item("About...", "app.about"))
            .section(vgtk::menu().item("Quit", "app.quit"))
            .build();
        gtk! {
            <Application::new_unwrap(Some("com.example.tuber"), ApplicationFlags::empty())>
                <SimpleAction::new("quit", None)
                    Application::accels=["<Ctrl>q"].as_ref()
                    enabled=true on activate=|a, _| Action::Exit
                />
                <SimpleAction::new("about", None)
                    enabled=true on activate=|a, _| Action::About
                />

                <Window default_width=WIN_WIDTH default_height=WIN_HEIGHT border_width=20 on destroy=|_| Action::Exit>
                    <HeaderBar title="Erlang: The Todo List" show_close_button=true>
                        <MenuButton HeaderBar::pack_type=PackType::End @MenuButtonExt::direction=ArrowType::Down relief=ReliefStyle::None image="open-menu-symbolic">
                            <Menu::new_from_model(&main_menu)></Menu>
                        </MenuButton>
                    </HeaderBar>
                    <Box orientation=Orientation::Vertical spacing=10 >
                        <ScrolledWindow Box::fill=true Box::expand=true>
                            <ListBox selection_mode=SelectionMode::None>
                                {
                                    self.tasks.iter().filter(|task| self.filter_task(task))
                                        .enumerate()
                                        .map(|(index, task)| task.render(index))
                                }
                            </ListBox>
                        </ScrolledWindow>
                        <Box>
                            <Label label=self.items_left()/>
                            <@Radio Box::center_widget=true active=self.filter
                                labels=["All", "Active", "Completed"].as_ref()
                                on changed=|filter| Action::Filter {filter}
                            />
                            { gtk_if!(self.count_comleted() > 0 => {
                               <Button label="Clear completed" Box::pack_type=PackType::End
                                   on clicked=|_| Action::Cleanup />
                           })}
                        </Box>
                        <Box orientation=Orientation::Vertical spacing=70 >
                            <Entry placeholder_text="What needs to be done?" on activate=|entry| {
                                entry.select_region(0, -1);
                                Action::Add {
                                    task: entry.get_text().unwrap().to_string()
                                }
                            }/>
                        </Box>
                    </Box>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
