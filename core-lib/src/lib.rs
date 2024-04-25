use buffer::buffer::Buffer;
use rustc_hash::FxHashMap;

struct Editor {
    tabs: Vec<Tab>,
    buffers: FxHashMap<String, Buffer>,
}

struct Tab {
    file_path: String
}