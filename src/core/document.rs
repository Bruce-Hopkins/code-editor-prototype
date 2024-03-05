use std::io::Error;


use crate::lsp::client::file_path;
use rfd::FileDialog;
use ropey::{Rope, RopeSlice};
use ropey::iter::Lines;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use unicode_segmentation::UnicodeSegmentation;

use super::position::{Position};

pub struct ByteRange  {
    pub start: usize,
    pub end: usize
}

pub struct FileData {
    name: String,
    uri: String
}

pub struct Document {
    rope: Rope,
    file_data: Option<FileData>,
    is_saved: bool,
}

impl ToString for Document {
    fn to_string(&self) -> String {
        self.rope.to_string()
    }
}

impl Document {

    pub fn uri(&self) ->  Option<&String> {
        Some(&self.file_data.as_ref()?.uri)
    }

    pub fn filename(&self) -> Option<&String> {
        Some(&self.file_data.as_ref()?.name)
    }

    pub fn slice_all(&self) -> RopeSlice {
        self.rope.slice(..)
    }

    pub fn open(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let rope = Rope::from_reader(BufReader::new(file))?;

        let uri = file_path(filename);
        let file_data = FileData {
            name:filename.to_owned(),
            uri,
        };
        Ok(Self { 
            rope, 
            file_data:Some(file_data), 
            is_saved: true
        })
    }

    pub fn new() -> Self {
        let rope = Rope::new();
        Self {
            rope, 
            file_data: None,
            is_saved: true
        }
    }

    pub fn save(&mut self, workspace: Option<String>) -> Result<(), Error> {
        let mut dialog = FileDialog::new();
        let filename = match self.filename() {
            Some(value) => value.to_owned(),
            None => {
                // Let the user select the save path
                if let Some(workspace) = workspace {
                    dialog = dialog.set_directory(workspace);
                }
                let path = dialog
                    .save_file()
                    .unwrap();

                path.to_str()
                    .unwrap()
                    .to_owned()
            }
        };

        let file = File::create(filename.clone())?;
        self.rope.write_to(BufWriter::new(file)).expect("Failed to save file");
        self.is_saved = true;

        if let None = self.file_data {
            self.file_data = Some (
                FileData { name:filename.clone(), uri: file_path(&filename)}
            )
        }
        Ok(())
    }

    pub fn lines (&self) -> Lines<'_> {
        self.rope.lines()
    }

    pub fn get_character_pos(&self, position: &Position) -> usize {
        self.rope.line_to_char(position.line()) + position.character()
    }

    pub fn str_from_range(&self, start: usize, end: usize) -> RopeSlice<'_> {
        self.rope.slice(start..end)
    }

    pub fn get_line_bytes(&self, line_number: usize) -> usize {
        if line_number > self.rope.len_lines() {
            return self.rope.len_bytes();
        }
        self.rope.line_to_byte(line_number)
    }

    pub fn is_saved(&self) -> bool {
        self.is_saved
    }

    /**
     * Get's the length of characters within the line
     */
    pub fn line_len(&self, y: usize) -> usize {
        match self.rope.line(y).as_str() {
            Some(value) => value.graphemes(true).count(),
            None => self
                .rope
                .line(y)
                .to_string()
                .as_str()
                .graphemes(true)
                .count(),
        }
    }

    pub fn len(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn get_line(&self, line_idx: usize) -> Option<RopeSlice<'_>> {
        self.rope.get_line(line_idx)
    }

    pub fn last_line(&self) -> Option<RopeSlice<'_>> {
        self.get_line(self.rope.len_lines() - 1)
    }

    /**
     * Replaces the strings within the range of the position with the character inputted
     */
    pub fn replace(&mut self, start_idx: &Position, end_idx: &Position, character: String) -> Option<ByteRange> {
        let result = self.delete(start_idx, end_idx);
        self.insert(start_idx, character);
        result
    }

    /**
     * Returns the byte that was the starting position of the insert
     */
    pub fn insert(&mut self, position: &Position, character: String) -> usize {
        let start_idx = self.get_character_pos(position);
        self.rope.insert(start_idx, &character.to_string());
        self.is_saved = false;
        start_idx
    }

    /**
     * Returns the byte that was the starting position of the insert
     */
    pub fn delete(&mut self, start_idx: &Position, end_idx: &Position) -> Option<ByteRange> {
        let start_line = self.rope.get_line(start_idx.line()).unwrap();
        let start_idx = self.get_character_pos(start_idx);
        let end_idx = self.get_character_pos(end_idx);
        if start_line.len_chars() != 0 && end_idx <= self.rope.len_bytes() {
            self.rope.remove(start_idx..end_idx);
            self.is_saved = false;
            return Some(ByteRange{start: start_idx, end: end_idx});
        }
        None
    }
}
