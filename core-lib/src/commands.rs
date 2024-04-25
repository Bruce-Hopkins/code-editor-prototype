use rustc_hash::FxHashMap;

use crate::editor::Editor;

struct Commands(FxHashMap<String, fn(&mut Editor)>);

impl Default for Commands {
    fn default() -> Self {
        let mut commands_list = FxHashMap::default();
        Commands::insert(&mut commands_list, "undo", undo);
        Self(commands_list)
    }
}

impl Commands {
    fn insert(list:&mut FxHashMap<String, fn(&mut Editor)>, name: &str,  fun:fn(&mut Editor)) {
        if let Some(_) = list.insert(name.to_string(), fun) {
            eprintln!("Command with the name: '{name}' already exists. Replacing it.")
        }
    }
}

fn undo(editor: &mut Editor) {
    todo!()
}

fn redo(editor: &mut Editor) {
    todo!()
}

fn delete_line(editor: &mut Editor) {
    todo!()
}

fn next_word(editor: &mut Editor) {
    todo!()
}

fn previous_word(editor: &mut Editor) {
    todo!()
}

fn page_up(editor: &mut Editor) {
    todo!()
}

fn page_down(editor: &mut Editor) {
    todo!()
}

fn indent_line(editor: &mut Editor) {
    todo!()
}

fn unindent_line(editor: &mut Editor) {
    todo!()
}

fn goto_beginning_of_file(editor: &mut Editor) {
    todo!()
}

fn comment_line(editor: &mut Editor) {
    todo!()
}

fn uncomment_line(editor: &mut Editor) {
    todo!()
}

fn next_tab(editor: &mut Editor) {
    todo!()
}

fn previous_tab(editor: &mut Editor) {
    todo!()
}

fn find(editor: &mut Editor) {
    todo!()
}
