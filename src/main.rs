use atty::Stream;
use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the user requested help
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_usage();
        return;
    }

    if atty::is(Stream::Stdin) {
        // No pipe input, run interactive mode
        println!("No input detected. Running interactive mode...");
        interactive_mode();
    } else {
        // Piped input, process it
        let stdin = io::stdin();
        let handle = stdin.lock();
        let mut text: String = "".to_string();
        for line in handle.lines() {
            let line = line.expect("Failed to read line") + "\n";
            text += &line;
        }
        println!("Processed input:\n{}", text);
    }
}

fn interactive_mode() {
    println!("This is an interactive mode. You can implement other features here.");
}

fn print_usage() {
    println!("copycmd - simple clipboard process");
    println!("\nUsage:");
    println!("  copycmd");
    println!("      Shows the current clipboard contents");
    println!("  <command> | copycmd");
    println!("      Pipes a stdout to your clipboard");
}
