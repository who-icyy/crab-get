use clap::{Arg, Command};
mod downloader;

fn main() {
    // Define the command-line arguments using `clap`
    let matches = Command::new("Print URL CLI")
        .version("1.0")
        .author("@Who-icyy")
        .about("A Simple WGET clone in RUST programming language.")
        .arg(
            Arg::new("url")
                .help("File Url to download")
                .required(true)
                .index(1), // This means it's a positional argument
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("File Output Name")
                .required(true)
                .value_name("File Name"),
        )
        .get_matches();

    let url = matches.get_one::<String>("url").unwrap();
    let o = matches.get_one::<String>("output").unwrap();

    match downloader::download_file(url, o) {
        Ok(()) => println!("File downloaded successfully to {}", o),
        Err(e) => eprintln!("Error: {}", e),
    }
}
