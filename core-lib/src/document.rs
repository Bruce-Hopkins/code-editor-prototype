use std::path::PathBuf;
use rfd::FileDialog;
use ropey::{Rope, RopeBuilder, RopeSlice};
use ropey::iter::Lines;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use unicode_segmentation::UnicodeSegmentation;

use crate::errors::EngineErrors;
use crate::position::Position;
use crate::selection::Range;

pub struct DocumentChange {
    pub range: Range,
    pub byte_range: ByteRange,
    pub text: String,
    pub file: String
}

impl DocumentChange {
    pub fn new(range: Range, byte_range: ByteRange, text: String, file:String) -> Self {
        Self {
            range,
            byte_range,
            text,
            file,
        }
    }
}

pub fn file_path(relative_path: &str) -> Result<String, EngineErrors> {
    let path = PathBuf::from(relative_path);
    let absolute_path = fs::canonicalize(path).map_err(EngineErrors::from_file_error("Get full file path"))?;
    let absolute_path = match absolute_path.to_str() {
        Some(value) => value,
        None => return Err(EngineErrors::InvalidString("Provided path is not valid unicode".to_owned())),
    };
    Ok(
        format!("file://{}", absolute_path)
    )
} 
pub struct ByteRange  {
    pub start: usize,
    pub end: usize
}

#[derive(Debug)]
pub struct FileData {
    name: String,
    uri: String
}

#[derive(Debug)]
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

impl From<&str> for Document {
    fn from(value: &str) -> Self {
        let mut rope = RopeBuilder::new();
        rope.append(value);
        Document { rope:rope.finish(), file_data: None, is_saved: true }
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

    pub fn open(filename: &str) -> Result<Self, EngineErrors> {
        let file = File::open(filename)
        .map_err(EngineErrors::from_file_error("Open File"))?;

        let rope = Rope::from_reader(BufReader::new(file))
        .map_err(EngineErrors::from_file_error("Open file to rope"))?;

        let uri = file_path(filename)?;
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

    pub fn save(&mut self) -> Result<(), EngineErrors> {
        let filename = match self.filename() {
            Some(value) => value.to_owned(),
            None => return Err(
                EngineErrors::FileError { 
                    operation_name: "Save File".to_owned(), 
                    explaination: "File path not set.".to_owned()
                }
            )
        };

        let file = File::create(filename.clone())
        .map_err(EngineErrors::from_file_error("Open file"))?;

        self.rope.write_to(BufWriter::new(file))
        .map_err(EngineErrors::from_file_error("Save File"))?;
        self.is_saved = true;

        if let None = self.file_data {
            self.file_data = Some (
                FileData {uri: file_path(&filename)?, name:filename }
            )
        }
        Ok(())
    }

    pub fn lines (&self) -> Lines<'_> {
        self.rope.lines()
    }

    pub fn get_character_pos(&self, position: &Position) -> Option<usize> {
        let result = self
        .rope
        .try_line_to_char(position.line);
        match result {
            Ok(character_pos) => return Some(character_pos.saturating_add(position.character)),
            Err(_) => None,
        }
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

    pub fn len_lines(&self) -> usize {
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
    pub fn replace(&mut self, start_idx: &Position, end_idx: &Position, character: String) -> Result<Option<ByteRange>, EngineErrors> {
        let result = self.delete(start_idx, end_idx);
        if let Ok(value) = result.as_ref() {
            if value.is_some() {
                self.insert(start_idx, character);
            }
        }
        result
    }

    /**
     * Returns the byte that was the starting position of the insert
     * 
     * Returns `None` if the position passed doesn't exist.
     */
    pub fn insert(&mut self, position: &Position, character: String) -> Option<usize> {
        let start_idx = self.get_character_pos(position)?;
        if start_idx > self.rope.len_bytes() {
            return None
        }
        self.rope.insert(start_idx, &character);
        self.is_saved = false;
        Some(start_idx)
    }

    /**
     * Returns the byte that was the starting position of the insert.
     * 
     * If the start value is greater than the max position in the document, then it will return `None`
     * 
     * If the end value passed is greater than the existing values in the document, then it will delete up until the 
     * end.     
     */
    pub fn delete(&mut self, start_pos: &Position, end_pos: &Position) -> Result<Option<ByteRange>, EngineErrors> {

        let start_line = match self.rope.get_line(start_pos.line) {
            Some(value) => value,
            None => return Ok(None),
        };
        let start_idx = match self.get_character_pos(start_pos) {
            Some(value) => value,
            None => return Ok(None),
        };
        let end_idx = self.get_character_pos_or_end_of_rope(end_pos);

        if start_idx > end_idx {
            return Err(
                EngineErrors::InvalidPosition { 
                    operation_name: "Delete".to_owned(), 
                    explaination: "Start index is greater than end index".to_owned() 
                }
            )
        }

        if start_line.len_chars() != 0 && start_idx != end_idx {
            self.rope.remove(start_idx..end_idx);
            self.is_saved = false;
            return Ok(Some(ByteRange{start: start_idx, end: end_idx}));
        }
        Ok(None)
    }

    /**
     * Gets the passed character position. If the position doesn't exist then it will return the total bytes of the 
     * rope.
     */
    fn get_character_pos_or_end_of_rope(&self, pos: &Position) -> usize{
        if let Some(pos) = self.get_character_pos(pos) {
            let end_of_rope = self.rope.len_bytes();
            if pos > end_of_rope {
                end_of_rope
            }
            else {
                pos
            }
        } else {
            self.rope.len_bytes()
        }
    }
}
