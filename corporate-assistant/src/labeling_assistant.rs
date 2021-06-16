
pub mod labeling_assistant {

use fltk::{app, button::Button, prelude::*, text::TextEditor, text::TextBuffer, window::Window };

pub fn run(candidate_label : &str) -> String {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(640, 768)
        .center_screen()
        .with_label("What did you mean?");

    let mut hpack = fltk::group::Pack::default().with_size(190, 40).center_of(&wind);
    hpack.set_type(fltk::group::PackType::Horizontal);

//    let mut int1 = Button::default()
//        .with_size(0,50)
//        .with_label("File an issue");
//    let mut int2 = Button::default()
//        .with_size(0,50)
//        .with_label("Create an issue");

    let mut tb = TextBuffer::default();
        tb.set_text(candidate_label);
    let mut te = TextEditor::default()
        .with_size(100,0);
        te.set_buffer(Some(tb));
        te.set_insert_mode(true);
    let mut int3 = Button::default()
        .with_size(50,0)
        .with_label("Ok");

    hpack.end();


    //wind.make_resizable(true);
    wind.end();
    wind.show();

    /* Event handling */

    let (s, r) = app::channel::<String>();

//    int1.emit(s.clone(), int1.label());
//    int2.emit(s.clone(), int2.label());
    int3.emit(s.clone(), String::from("text_editor"));

    while app.wait() {
        let msg = r.recv();
        match &msg {
            Some(msg) => if msg == "text_editor" { return String::from(&te.buffer().unwrap().text()) } else { return String::from(msg)}, 
            _ => ()
        }
    }

    app.run().unwrap();
    String::from("")
}

}
