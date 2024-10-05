# 🦀 Crab Get

Crab Get is a command-line tool written in Rust, designed as a lightweight clone of the popular `wget` utility. It allows users to easily download files from the web, handling various file types and providing progress feedback during downloads.

## Features 🌟

- **File Downloads**: Download files from HTTP and HTTPS URLs.
- **Progress Bar**: Provides a visual indication of download progress.
- **Custom Output Filename**: Specify the output filename if desired.

## Installation ⚙️

To install **Crab Get**, you need to have Rust and Cargo installed on your system. If you haven't done this yet, you can follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

Once Rust is installed, clone the repository and build the project:

```bash
git clone https://github.com/yourusername/crab_get.git
cd crab_get
cargo build --release
```

## Usage 

## Using Rust

```bash 
cargo run <URL> -o <Output file name Including .extention>
```
## cget

*  Download and Unzip release.zip 
*  Add output folder to path

```bash 
cget <URL> -o <Output file name Including .extention>
```