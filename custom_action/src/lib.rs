// 1. Make a window where script can be put and by default it
//   we put this script there
// 2. Upon finishing to edit Save a script

use serde::Deserialize;
use toml;

use fltk::{
    app,
    button::Button,
    dialog,
    enums::{CallbackTrigger, Color, Event, Font, FrameType, Shortcut},
    frame::Frame,
    menu,
    menu::Choice,
    prelude::*,
    text, window,
};
use std::process::Command;
use std::{
    error,
    fs::File,
    io::prelude::*,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

#[derive(Copy, Clone)]
pub enum Message {
    ChoiceChanged,
    Changed,
    Save,
    Quit,
    Cut,
    Copy,
    Paste,
}

pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

pub struct ScriptEditor {
    editor: text::TextEditor,
}

impl ScriptEditor {
    pub fn new(buf: text::TextBuffer) -> Self {
        let mut editor = text::TextEditor::new(5, 35, 790, 560, "");
        editor.set_buffer(Some(buf));

        #[cfg(target_os = "macos")]
        editor.resize(5, 5, 790, 590);

        editor.set_scrollbar_size(15);
        editor.set_text_font(Font::Courier);
        editor.set_linenumber_width(32);
        editor.set_linenumber_fgcolor(Color::from_u32(0x008b_8386));
        editor.set_trigger(CallbackTrigger::Changed);

        Self { editor }
    }
}

impl Deref for ScriptEditor {
    type Target = text::TextEditor;

    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl DerefMut for ScriptEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.editor
    }
}

pub struct MyMenu {
    _menu: menu::SysMenuBar,
}

impl MyMenu {
    pub fn new(s: &app::Sender<Message>) -> Self {
        let mut menu = menu::SysMenuBar::default().with_size(800, 35);
        menu.set_frame(FrameType::FlatBox);

        menu.add_emit(
            "&File/Save\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            *s,
            Message::Save,
        );

        menu.add_emit(
            "&File/Quit\t",
            Shortcut::Ctrl | 'q',
            menu::MenuFlag::Normal,
            *s,
            Message::Quit,
        );

        menu.add_emit(
            "&Edit/Cut\t",
            Shortcut::Ctrl | 'x',
            menu::MenuFlag::Normal,
            *s,
            Message::Cut,
        );

        menu.add_emit(
            "&Edit/Copy\t",
            Shortcut::Ctrl | 'c',
            menu::MenuFlag::Normal,
            *s,
            Message::Copy,
        );

        menu.add_emit(
            "&Edit/Paste\t",
            Shortcut::Ctrl | 'v',
            menu::MenuFlag::Normal,
            *s,
            Message::Paste,
        );

        Self { _menu: menu }
    }
}

pub struct MyApp {
    app: app::App,
    saved: bool,
    custom_actions: CustomActions,
    ca_filename: PathBuf,
    r: app::Receiver<Message>,
    buf: text::TextBuffer,
    editor: ScriptEditor,
    printable: text::TextDisplay,
    ca_list: menu::Choice,
    te: text::TextEditor,
}

impl MyApp {
    pub fn new(custom_action_config_file: PathBuf) -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        app::background(211, 211, 211);
        let custom_actions = parse_config(custom_action_config_file.clone());

        let (s, r) = app::channel::<Message>();
        let mut buf = text::TextBuffer::default();
        buf.set_tab_distance(4);
        let title = format!(
            "Editing => {}",
            custom_action_config_file
                .to_str()
                .expect("Problem with filename of custom action config")
        );
        let mut main_win = window::Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label(&title);

        let vpack = fltk::group::Pack::default()
            .with_size(800, 600)
            .center_of(&main_win);

        let _menu = MyMenu::new(&s);
        // Custom phrase based on typed characters or recorded phrase
        let mut hpack = fltk::group::Pack::default()
            .with_size(600, 25)
            .above_of(&vpack, 0);
        hpack.set_type(fltk::group::PackType::Horizontal);
        let _frame = Frame::default()
            .with_size(150, 25)
            .with_label("Custom phrase:");
        let mut tb = text::TextBuffer::default();
        tb.set_text("<custom action phrase>");
        let mut te = text::TextEditor::default().with_size(350, 0);
        te.set_buffer(Some(tb));
        te.set_insert_mode(true);
        // Drop down menu with existing actions to load
        let mut ca_list = Choice::new(0, 0, 150, 0, "");
        //        for ca in custom_actions.custom_actions.iter() {
        //           println!("Registring phrase: {}", ca.phrase);
        //         println!("  with script: {}", ca.script);
        custom_actions
            .custom_actions
            .iter()
            .for_each(|x| ca_list.add_choice(&x.phrase));
        ca_list.emit(s, Message::ChoiceChanged);
        // Recording of phrase
        let mut record_button = Button::default().with_size(80, 0).with_label("Record"); // TODO: Make recording and transcription
        hpack.end();

        let mut editor = ScriptEditor::new(buf.clone());
        editor.emit(s, Message::Changed);
        vpack.end();

        main_win.make_resizable(true);
        // only resize editor, not the menu bar
        main_win.resizable(&*editor);
        main_win.end();
        main_win.show();
        main_win.set_callback(move |_| {
            if app::event() == Event::Close {
                s.send(Message::Quit);
            }
        });

        // What shows when we attempt to print
        let mut printable = text::TextDisplay::default();
        printable.set_frame(FrameType::NoBox);
        printable.set_scrollbar_size(0);
        printable.set_buffer(Some(buf.clone()));

        Self {
            app,
            saved: true,
            custom_actions,
            ca_filename: custom_action_config_file,
            r,
            buf,
            editor,
            printable,
            ca_list,
            te,
        }
    }

    pub fn save_file(&mut self) -> Result<(), Box<dyn error::Error>> {
        // Read a custom action config file and append currently edited action
        let mut buf = text::TextBuffer::default();
        buf.load_file(&self.ca_filename)?;

        let mut content_to_append: String = "\n[[custom_actions]]\n phrase = \"".to_string();
        content_to_append += &self.te.buffer().unwrap().text().trim();
        content_to_append += "\"\n script = \"\"\"";
        content_to_append += &self.buf.text();
        content_to_append += "\"\"\"\n";
        buf.append(&content_to_append);
        buf.save_file(&self.ca_filename)?;
        self.saved = true;
        Ok(())
    }

    pub fn launch(&mut self) {
        while self.app.wait() {
            use Message::*;
            if let Some(msg) = self.r.recv() {
                match msg {
                    Changed => self.saved = false,
                    ChoiceChanged => {
                        match self.ca_list.value() {
                            -1 => (),
                            _ => {
                                // Set phrase from dropdown into phrase input field
                                let ca = &self.custom_actions.custom_actions
                                    [self.ca_list.value() as usize];
                                self.te.buffer().unwrap().set_text(&ca.phrase);
                                // Put corressponding script into script editor
                                // find script for phrase and put script into editor
                                self.editor.buffer().unwrap().set_text(&ca.script);
                            }
                        }
                        self.saved = false
                    }
                    Save => self.save_file().unwrap(),
                    Quit => {
                        if !self.saved {
                            let x = dialog::choice(
                                center().0 - 200,
                                center().1 - 100,
                                "Would you like to save your work?",
                                "Yes",
                                "No",
                                "",
                            );
                            if x == 0 {
                                self.save_file().unwrap();
                            }
                        }
                        self.app.quit();
                    }
                    Cut => self.editor.cut(),
                    Copy => self.editor.copy(),
                    Paste => self.editor.paste(),
                }
            }
        }
    }
}

pub fn action_creator(custom_action_config_file: PathBuf) {
    // Get user script name and pass it to be executed
    let mut app = MyApp::new(custom_action_config_file);
    app.launch();
}

fn load_script(script_name: &PathBuf) -> String {
    let mut file = File::open(script_name).expect("Error opening script");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading script file");
    contents
}

pub fn action_executor(script: &str) -> bool {
    // Executing an action from script
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("bash")
            .arg("-c")
            .arg("eval ".to_string() + script)
            .output()
            .expect("failed to execute process")
    };
    let stdout = output.stdout;
    println!("out: {}", String::from_utf8(stdout).unwrap());
    output.status.success()
}

#[derive(Deserialize, Debug)]
pub struct CustomActions {
    pub custom_actions: Vec<CustomAction>,
}

#[derive(Deserialize, Debug)]
pub struct CustomAction {
    pub phrase: String, // Command to execute..
    pub script: String, // ..script
}

pub fn parse_config(path: PathBuf) -> CustomActions {
    let file = std::fs::File::open(path);
    let mut reader = std::io::BufReader::new(file.expect("Cannot open file"));

    let mut c: String = "".to_string();
    reader.read_to_string(&mut c);
    toml::from_str(&c).expect("Error: Parsing of custom actions config")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_action() {
        let status = action_executor(if cfg!(target_os = "windows") {
            "dir"
        } else {
            "ls"
            //"gnome-terminal -- sudo iptraf-ng -g" //Network monitoring
        });
        assert_eq!(status, true);
    }
}
