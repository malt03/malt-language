use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write, Lines};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,
}

enum Operator {
    Plus,
    Minus,
}

enum TokenKind {
    Head,
    Operator{value: Operator},
    Number{value: String},
}

struct Token {
    kind: TokenKind,
    next: Option<Box<Token>>,
}

fn tokenize(lines: Lines<BufReader<File>>) -> std::io::Result<Option<Box<Token>>> {
    let mut head = Box::new(Token{kind: TokenKind::Head, next: None});
    let mut current = &mut head;
    
    for line in lines {
        for c in line?.chars() {
            match c {
                ' ' => continue,
                '0'..='9' => {
                    if let TokenKind::Number{value} = &current.kind {
                        (*current).kind = TokenKind::Number{value: format!("{}{}", value, c)};
                    } else {
                        let token = Token{kind: TokenKind::Number{value: c.to_string()}, next: None};
                        current.next = Some(Box::new(token));
                        current = current.next.as_mut().unwrap();
                    }
                },
                '+' => {
                    let token = Token{kind: TokenKind::Operator{value: Operator::Plus}, next: None};
                    current.next = Some(Box::new(token));
                    current = current.next.as_mut().unwrap();
                },
                '-' => {
                    let token = Token{kind: TokenKind::Operator{value: Operator::Minus}, next: None};
                    current.next = Some(Box::new(token));
                    current = current.next.as_mut().unwrap();
                },
                _ => panic!("unexpected"),
            }
        }
    }
    
    Ok(head.next)
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    match File::open(&args.file) {
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            let mut current_token = tokenize(lines)?;
            
            let out_file = File::create("./out/main.wat")?;
            let mut out_file = LineWriter::new(out_file);
            out_file.write_all(br#"(module
    (import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
    (func $_start
"#)?;

            // let mut number = String::from("");
            // let mut last_token: Option<char> = None;
            // for line in lines {
            //     for c in line?.chars() {
            //         match c {
            //             '0'..='9' => number.push(c),
            //             '+' | '-' => {
            //                 out_file.write_all(format!("        i32.const {}\n", number).as_bytes())?;
            //                 number = "".into();
            //                 if let Some(token) = last_token {
            //                     match token {
            //                         '+' => out_file.write_all(b"        i32.add\n")?,
            //                         '-' => out_file.write_all(b"        i32.sub\n")?,
            //                         _ => panic!("unexpected"),
            //                     }
            //                 }
            //                 last_token = Some(c);
            //             },
            //             _ => panic!("unexpected"),
            //         }
            //     }
            // }
            // out_file.write_all(format!("        i32.const {}\n", number).as_bytes())?;
            // if let Some(token) = last_token {
            //     match token {
            //         '+' => out_file.write_all(b"        i32.add\n")?,
            //         '-' => out_file.write_all(b"        i32.sub\n")?,
            //         _ => panic!("unexpected"),
            //     }
            // }

            out_file.write_all(br#"        call $_exit)
    (memory 0)
    (export "memory" (memory 0))
    (export "_start" (func $_start))
)"#)?;
            out_file.flush()?;
        }
        Err(err) => {
            println!("{}: {}", err, args.file.to_str().unwrap());
            std::process::exit(1);
        }
    }

    Ok(())
}
