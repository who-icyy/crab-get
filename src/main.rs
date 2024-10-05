use clap::{Arg, Command};

fn main() {
    // Define the command-line arguments using `clap`
    let matches = Command::new("Print URL CLI")
        .version("1.0")
        .author("Your Name")
        .about("Prints a URL from command-line input")
        .arg(
            Arg::new("url")
                .help("The URL to print")
                .required(true)
                .index(1), // This means it's a positional argument
        )
        .get_matches();

    // Get the URL argument from the command-line input
    let url = matches.get_one::<String>("url").unwrap();

    // Call the function to print the URL
    print_url(url);
}

// Simple function that takes a URL and prints it
fn print_url(url: &str) {
    println!("The provided URL is: {}", url);
}
