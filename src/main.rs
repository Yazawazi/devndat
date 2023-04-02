use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use zip::ZipArchive;

// Just PKZip signature
const SIGNATURE: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];

// Key used to decrypt the file header and footer (need reverse in footer)
// Text: `d6c5fKI3GgBWpZF3Tz6ia3kF0`
const KEY: [u8; 25] = [
    0x64, 0x36, 0x63, 0x35, 0x66, 0x4B, 0x49, 0x33, 0x47, 0x67, 0x42, 0x57, 0x70, 0x5A, 0x46, 0x33,
    0x54, 0x7A, 0x36, 0x69, 0x61, 0x33, 0x6B, 0x46, 0x30,
];

const REVERSED_KEY: [u8; 25] = [
    0x30, 0x46, 0x6B, 0x33, 0x61, 0x69, 0x36, 0x7A, 0x54, 0x33, 0x46, 0x5A, 0x70, 0x57, 0x42, 0x67,
    0x47, 0x33, 0x49, 0x4B, 0x66, 0x35, 0x63, 0x36, 0x64,
];

#[derive(Parser, Debug)]
#[clap(version, name = "devndat")]
struct Options {
    /// Input vndat file
    #[arg(short = 'i', long = "input")]
    input: PathBuf,
    /// Output folder
    #[arg(short = 'o', long = "output")]
    output: PathBuf,
}

fn check_file_signature(file: &PathBuf) -> bool {
    let mut file = File::open(file).unwrap();
    let mut signature = [0; 4];

    file.read_exact(&mut signature).unwrap();

    signature == SIGNATURE
}

fn un_zip_with_decrypt_pk_file(pk_file: &PathBuf, folder: &Path) {
    println!("Unzipping and decrypting file: {}", pk_file.display());

    let file = File::open(pk_file).unwrap();
    let mut zip_file = ZipArchive::new(file).unwrap();

    for i in 0..zip_file.len() {
        let mut file = zip_file.by_index(i).unwrap();
        let file_path = folder.join(file.name());

        fs::create_dir_all(file_path.parent().unwrap()).unwrap();

        if file.is_dir() {
            println!("Creating folder: {}", file.name());

            fs::create_dir_all(folder.join(file.name())).unwrap();
        } else {
            println!("Decrypting file: {}", file.name());

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();

            let length = buffer.len();

            if file_path.exists() {
                fs::remove_file(&file_path).unwrap();
            }

            let mut out_file = File::create(&file_path).unwrap();

            if length < 99 {
                if length == 0 {
                    println!("File is empty: {}", file.name());
                } else {
                    println!("File size is smaller than 99: {}", file.name());

                    for i in 0..length {
                        buffer[i] ^= REVERSED_KEY[i % 25];
                    }
                }
            } else {
                for i in 0..99 {
                    buffer[length - 99 + i] ^= REVERSED_KEY[i % 25];
                }

                for i in 0..100 {
                    buffer[i] ^= KEY[i % 25];
                }
            }
            out_file.write_all(&buffer).unwrap();
        }
    }
}

fn main() {
    let args = Options::parse();

    if !args.input.exists() {
        println!("Input file does not exist");
        exit(1);
    }

    if !args.input.is_file() {
        println!("Input is not a file");
        exit(1);
    }

    if !check_file_signature(&args.input) {
        println!("Input file is not a valid vndat file");
        exit(1);
    }

    if args.output.exists() {
        if !args.output.is_dir() {
            println!("Output is not a directory");
            exit(1);
        }
    } else {
        fs::create_dir_all(&args.output).unwrap();
    }

    un_zip_with_decrypt_pk_file(&args.input, args.output.as_path());
}
