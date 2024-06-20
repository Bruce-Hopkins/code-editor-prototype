use std::collections::HashMap;

use buffer::{buffer::Buffer, position::Position, selection::{Selection, SelectionList}};
use rustc_hash::FxHashMap;

use crate::editor::Editor;

pub struct Commands(FxHashMap<&'static str, fn(&mut Editor)>);

impl Default for Commands {
    fn default() -> Self {
        let mut commands_list = FxHashMap::default();
        let commands: [(&'static str, fn(&mut Editor)); 2] = [
            ("undo", undo),
            ("next_word_end", next_word_end),
        ];
        for (command_name, command_fun) in commands {
            Commands::insert(&mut commands_list, command_name, command_fun);
        }
        Self(commands_list)
    }
}

impl Commands {
    fn insert(list:&mut FxHashMap<&'static str, fn(&mut Editor)>, name: &'static str,  fun:fn(&mut Editor)) {
        if let Some(_) = list.insert(name, fun) {
            eprintln!("Command with the name: '{name}' already exists. Replacing it.")
        }
    }

    pub fn get(&self, value: &str) -> Option<&fn(&mut Editor)> {
        self.0.get(value)
    }

    pub fn hashmap(&self) -> &FxHashMap<&'static str, fn(&mut Editor)> {
        &self.0
    }
}


fn undo(editor: &mut Editor) {
    if let Some(buffer) = editor.get_mut_buffer().as_mut() {
        buffer.undo()
    }
}

fn redo(editor: &mut Editor) {
    if let Some(buffer) = editor.get_mut_buffer().as_mut() {
        buffer.redo()
    }
}

fn delete_line(editor: &mut Editor) {
    todo!()
}

fn delete(editor: &mut Editor) {
    if let Some(buffer) = editor.get_mut_buffer().as_mut() {
        buffer.delete()
    }
}

fn backspace(editor: &mut Editor) {
    if let Some(buffer) = editor.get_mut_buffer().as_mut() {
        let mut selection_list = SelectionList::default();
        for selection in buffer.get_selections() {
            if selection.is_empty() {
                let new_pos = buffer.move_back(selection.selection_start());
                selection_list.add_cursor(new_pos)
            }
            else {
                selection_list.add_selection(selection)
            }
        }
        buffer.set_selections(selection_list);
        buffer.delete();
    }
}

fn next_word_end(editor: &mut Editor) {
    if let Some(buffer)  = editor.get_mut_buffer() {
        let mut selection_list = SelectionList::default();
        for selection in buffer.get_selections() {
            let position = selection.selection_end();
            let mut position = Position::new(position.line, position.character.saturating_add(1));
            position = move_until_character(position, buffer, &[' ', '\t', '\n', '(', ')']).unwrap();
            position = move_until_next_character_is(position, buffer, &[' ', '\t', '\n', '(', ')']).unwrap();
            selection_list.add_cursor(position);
        }
        buffer.set_selections(selection_list)
    }
}

fn move_until_character(character_pos: Position, buffer: &Buffer, charcter_list: &[char]) -> Option<Position> {
    let mut character_hash_map = FxHashMap::default();
    for c in charcter_list {
        character_hash_map.insert(c.to_string(), ());
    }
    for pos in buffer.doc_iter(character_pos) {
        let ch = buffer.get_slice_from_pos(&pos).expect("Character is somehow missing");
        let result = character_hash_map.get(&ch.to_string());
        if result.is_none() {
            return Some(pos)
        }
    }
    None
}

fn move_until_next_character_is(character_pos: Position, buffer: &Buffer, charcter_list: &[char]) -> Option<Position> {
    let mut character_hash_map = FxHashMap::default();
    for c in charcter_list {
        character_hash_map.insert(c.to_string(), ());
    }
    for pos in buffer.doc_iter(character_pos) {
        let ch = buffer.get_slice_from_pos(&pos).expect("Character is somehow missing");
        let result = character_hash_map.get(&ch.to_string());
        if result.is_some() {
            return Some(pos)
        }
    }
    None
}

fn move_up(editor: &mut Editor) {
    todo!()
}

fn move_down(editor: &mut Editor) {
    todo!()
}

fn move_left(editor: &mut Editor) {
    todo!()
}

fn move_right(editor: &mut Editor) {
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


