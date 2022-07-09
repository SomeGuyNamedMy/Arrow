trait Backend {
    fn new(file: String) -> Self;
}

struct CrosstermBackend {}

impl CrosstermBackend {}

impl Backend for CrosstermBackend {
    fn new(file: String) -> Self {
        Self {}
    }
}
