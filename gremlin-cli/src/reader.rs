use rustyline::Editor;

pub struct Reader {
    editor: Editor<()>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            editor: Editor::new(),
        }
    }
    pub fn next(&mut self, prompt: &str) -> Option<(String, Vec<String>)> {
        match self.editor.readline(prompt) {
            Ok(line) => {
                let args = shellwords::split(&line).unwrap();

                Some((line, args))
            }
            Err(_) => None,
        }
    }

    pub fn update_history(&mut self, line: &str) -> bool {
        self.editor.add_history_entry(line)
    }
}
