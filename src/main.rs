mod backends;
use ropey::RopeBuilder;

fn main() {
    let hs = backends::CrosstermBackend::new("hello");
}
