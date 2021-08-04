// 1. Make a window where script can be put and by default it
//   we put this script there
// 2. Upon finishing to edit Save a script

use fltk::{
    app, dialog,
    enums::{CallbackTrigger, Color, Event, Font, FrameType, Shortcut},
    menu,
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
    filename: String,
    r: app::Receiver<Message>,
    buf: text::TextBuffer,
    editor: ScriptEditor,
    printable: text::TextDisplay,
}

impl MyApp {
    pub fn new(script_name: PathBuf) -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        app::background(211, 211, 211);
        let filename: String = script_name.into_os_string().into_string().unwrap();

        let (s, r) = app::channel::<Message>();
        let mut buf = text::TextBuffer::default();
        buf.load_file(&filename).expect("Error loading script file");
        buf.set_tab_distance(4);
        let mut main_win = window::Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label(&("Editing => ".to_string() + &filename));
        let _menu = MyMenu::new(&s);
        let mut editor = ScriptEditor::new(buf.clone());
        editor.emit(s, Message::Changed);
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
            filename,
            r,
            buf,
            editor,
            printable,
        }
    }

    pub fn save_file(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.buf.save_file(&self.filename)?;
        self.saved = true;
        Ok(())
    }

    pub fn launch(&mut self) {
        while self.app.wait() {
            use Message::*;
            if let Some(msg) = self.r.recv() {
                match msg {
                    Changed => self.saved = false,
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

pub fn action_creator(script_name: PathBuf) {
    // Load a script into editor in order to have it edited by user
    println!("Script {} editing!", script_name.display());
    let mut app = MyApp::new(script_name);
    app.launch();
}

fn load_script(script_name: &PathBuf) -> String {
    let mut file = File::open(script_name).expect("Error opening script");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading script file");
    contents
}

pub fn action_executor(script_name: PathBuf) {
    // Executing an action from script
    println!("Script to be executed: {}", script_name.display());
    println!("Script:\n {}", load_script(&script_name));
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("bash")
            .arg(script_name.into_os_string())
            .output()
            .expect("failed to execute process")
    };
    let stdout = output.stdout;
    println!("out: {}", String::from_utf8(stdout).unwrap());
}
