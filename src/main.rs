fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn parse_markdown_file() {}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {}

fn usage() {}

fn main() {
    usage();
}
