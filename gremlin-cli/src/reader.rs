use rustyline::Editor;

use crate::GremlinOpt;

pub struct Reader {
    editor: Editor<()>,
    opt: GremlinOpt,
}

impl Reader {
    pub fn new(opt: GremlinOpt) -> Reader {
        let mut editor = Editor::new();

        if let Some(path) = opt.history.as_ref() {
            if editor.load_history(&path).is_err() {
                println!("WARN: Failled to load history file at : {:?}", path)
            }
        }
        Reader { editor, opt }
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

    pub fn save_history(&mut self) {
        if let Some(path) = self.opt.history.as_ref() {
            if self.editor.save_history(&path).is_err() {
                println!("WARN: Failed save history to file : {:?}", path);
            }
        }
    }
}
