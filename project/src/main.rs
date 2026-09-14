mod gui;
mod lib;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let start = std::time::Instant::now();
    if args.iter().any(|arg| arg == "-gui") {
        gui::launch_gui();
    } else {
        lib::launch();
    }
}
