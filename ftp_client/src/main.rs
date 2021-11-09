use ftp::FtpStream;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::str;

pub enum Command {
    Put(String),
    Get(String, String),
    List,
    Quit,
}

impl Command {
    pub fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();

        match words.as_slice() {
            ["List"] => Some(Command::List),
            ["Quit"] => Some(Command::Quit),
            ["Put", filepath] => Some(Command::Put(filepath.to_string())),
            ["Get", filename, save_path] => {
                Some(Command::Get(filename.to_string(), save_path.to_string()))
            }
            _ => None,
        }
    }
}

fn main() {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    let _ = ftp_stream.login("test1", "1234").unwrap();

    println!("ftp > Type 'Put <file_path> to add an file");
    println!("ftp > Type 'Get <file_name> <save_path>' to get the file");
    println!("ftp > Type 'List' to list all file in ftp server");
    println!("ftp > Type 'Quit' to quit");
    println!("ftp > ");

    let stdin = io::stdin();
    ftp_stream.cwd("files").unwrap();

    for line in stdin.lock().lines()
    {
        let input = line.expect("ftp > [Error]: unable to read user input");

        match Command::from_input(&input) {
            None => println!("ftp > [Error]: invalid input format"),

            Some(Command::Put(filepath)) => {
                let filename = filepath.split('/').last().unwrap().clone();

                let incoming_file = fs::File::open(&filepath).unwrap();
                let mut buf_reader = BufReader::new(incoming_file);
                let mut contents = Vec::new();
                let _ = buf_reader.read_to_end(&mut contents);

                let _ = ftp_stream
                    .put(&filename, &mut contents.as_slice())
                    .expect("error");
                println!("ftp > [Info]: file downloaded to ftp_client");
            }

            Some(Command::Get(filename, save_path)) => {
                let remote_file = ftp_stream.simple_retr(filename.as_str()).unwrap();

                let file_path = save_path + filename.as_str();
                let file = fs::File::create(file_path);
                let _ = file.unwrap().write_all(remote_file.into_inner().as_ref());
                println!("ftp > [Info]: file downloaded to your computer");
            }

            Some(Command::List) => {
                let all_file = &ftp_stream.list(None).unwrap();
                if all_file.len() == 0 {
                    println!("ftp > [Info]: no files");
                } else {
                    for name in all_file.iter() {
                        println!("{}", name);
                    }
                }
            }

            Some(Command::Quit) => {
                let _ = ftp_stream.quit();
                break;
            }
        }
    }

    println!("ftp > ");
    println!("ftp > Goodbye!");
}
