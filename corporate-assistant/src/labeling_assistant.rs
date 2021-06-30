pub mod labeling_assistant {

    use fltk::{
        app, button::Button, frame::Frame, prelude::*, text::TextBuffer, text::TextEditor,
        window::Window,
    };
    pub use record::recorder::{Recorder};

    pub fn run(
        rec: &Recorder,
        recorded_vec: Vec<i16>,
        commands_list: Vec<&str>,
        candidate_label: &str,
    ) -> String {
        let app = app::App::default();
        let mut wind = Window::default()
            .with_size(640, 768)
            .center_screen()
            .with_label("What did you mean?");

        // List of possible options
        let mut proposals: Vec<Button> = Vec::new();
        let (s, r) = app::channel::<String>();
        let vpack = fltk::group::Pack::default()
            .with_size(600, 600)
            .center_of(&wind);
        let frame = Frame::default()
            .with_size(0, 50)
            .with_label("Existing labels:");
        // Iterate over existing labels and make
        // a button for each of them
        for l in commands_list.iter() {
            let mut label_button = Button::default().with_size(0, 50).with_label(l);
            label_button.emit(s.clone(), label_button.label());
            proposals.push(label_button);
        }

        vpack.end();

        // Custom typing of label based on recognized characters
        let mut hpack = fltk::group::Pack::default()
            .with_size(600, 50)
            .above_of(&vpack, 0);
        hpack.set_type(fltk::group::PackType::Horizontal);
        let mut tb = TextBuffer::default();
        tb.set_text(candidate_label);
        let mut te = TextEditor::default().with_size(350, 0);
        te.set_buffer(Some(tb));
        te.set_insert_mode(true);
        let mut custom_label = Button::default().with_size(50, 0).with_label("Ok");
        let placeholder = Frame::default().with_size(100, 0);
        let mut player = Button::default().with_size(50, 0).with_label("Play");
        hpack.end();
        let frame = Frame::default()
            .with_size(600, 50)
            .with_label("Custom label:")
            .above_of(&hpack, 0);

        //wind.make_resizable(true);
        wind.end();
        wind.show();

        /* Event handling */
        custom_label.emit(s.clone(), String::from("text_editor"));
        player.emit(s.clone(), String::from("play"));

        while app.wait() {
            let msg = r.recv();
            match &msg {
                Some(msg) => {
                    if msg == "text_editor" {
                        return String::from(&te.buffer().unwrap().text());
                    } else if msg == "play" {
                        rec.replay_recorded_vec(&recorded_vec);
                    } else {
                        return String::from(msg);
                    }
                }
                _ => (),
            }
        }

        app.run().unwrap();
        String::from("")
    }
}
