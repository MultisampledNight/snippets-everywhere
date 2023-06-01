fn main() {
    if let Err(err) = snippets_everywhere::run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
