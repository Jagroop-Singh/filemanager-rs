pub struct App {
    pub current_directory: String,
    pub current_selection: u32,
    pub files: Vec<String>,
    pub input_buf: Vec<String>,
    pub output_buf: Vec<String>,
    pub mode: Mode,
    pub window_size: (u16, u16),
}

pub enum Mode {
    InputMode,
    CommandMode,
}

impl App {
    pub fn new() -> App {
        App {
            current_directory: String::new(),
            current_selection: 0,
            files: Vec::new(),
            input_buf: Vec::new(),
            output_buf: Vec::new(),
            mode: Mode::CommandMode,
            window_size: (0, 0),
        }
    }
}
