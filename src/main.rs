use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write, Lines};
use std::str::FromStr;
use strum_macros::EnumString;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,
}

#[derive(Debug, EnumString, Clone)]
enum Operator0 {
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
}

#[derive(Debug, EnumString, Clone)]
enum Operator1 {
    #[strum(serialize = "*")]
    Multiply,
    #[strum(serialize = "/")]
    Divide,
}

#[derive(Debug)]
enum TokenKind {
    Operator0{value: Operator0},
    Operator1{value: Operator1},
    BracketOpen,
    BracketClose,
    Number{value: String},
    EOF,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    next: Option<Box<Token>>,
}

fn eof() -> Option<Box<Token>> {
    Some(Box::new(Token{kind: TokenKind::EOF, next: None}))
}

fn tokenize(lines: Lines<BufReader<File>>) -> std::io::Result<Box<Token>> {
    let mut head = Box::new(Token{kind: TokenKind::EOF, next: None});
    let mut current = &mut head;
    
    for line in lines {
        for c in line?.chars() {
            match c {
                ' ' => continue,
                '0'..='9' => {
                    if let TokenKind::Number{value} = &current.kind {
                        (*current).kind = TokenKind::Number{value: format!("{}{}", value, c)};
                    } else {
                        let token = Token{kind: TokenKind::Number{value: c.to_string()}, next: eof()};
                        current.next = Some(Box::new(token));
                        current = current.next.as_mut().unwrap();
                    }
                },
                '(' => {
                    let token = Token{kind: TokenKind::BracketOpen, next: eof()};
                    current.next = Some(Box::new(token));
                    current = current.next.as_mut().unwrap();
                },
                ')' => {
                    let token = Token{kind: TokenKind::BracketClose, next: eof()};
                    current.next = Some(Box::new(token));
                    current = current.next.as_mut().unwrap();
                },
                _ => {
                    let token = 
                    if let Ok(operator) = Operator0::from_str(&c.to_string()) {
                        Token{kind: TokenKind::Operator0{value: operator}, next: eof()}
                    } else if let Ok(operator) = Operator1::from_str(&c.to_string()) {
                        Token{kind: TokenKind::Operator1{value: operator}, next: eof()}
                    } else {
                        panic!("unexpected");
                    };
                    
                    current.next = Some(Box::new(token));
                    current = current.next.as_mut().unwrap();
                },
            }
        }
    }
    
    Ok(head.next.unwrap())
}

#[derive(Debug)]
enum NodeKind {
    Operator0 { value: Operator0 },
    Operator1 { value: Operator1 },
    Number { value: String },
}

#[derive(Debug)]
enum Tree {
    Nil,
    Node { kind: NodeKind, lhs: Box<Tree>, rhs: Box<Tree> },
}

fn expr(token: &Box<Token>) -> (Box<Tree>, &Box<Token>) {
    let (mut tree, mut token) = mul(token);

    loop {
        if let TokenKind::Operator0{ ref value } = token.kind {
            token = token.next.as_ref().unwrap();
            let (rhs, tmp) = mul(token);
            token = tmp;
            tree = Box::new(Tree::Node { kind: NodeKind::Operator0 { value: (*value).clone() }, lhs: tree, rhs });
        } else {
            return (tree, token);
        }
    }
}

fn mul(token: &Box<Token>) -> (Box<Tree>, &Box<Token>) {
    let (mut tree, mut token) = primary(token);

    loop {
        if let TokenKind::Operator1{ ref value } = token.kind {
            token = token.next.as_ref().unwrap();
            let (rhs, tmp) = primary(token);
            token = tmp;
            tree = Box::new(Tree::Node { kind: NodeKind::Operator1 { value: (*value).clone() }, lhs: tree, rhs });
        } else {
            return (tree, token);
        }
    }
}

fn primary(mut token: &Box<Token>) -> (Box<Tree>, &Box<Token>) {
    if let TokenKind::BracketOpen = token.kind {
        token = token.next.as_ref().unwrap();
        let (tree, mut token) = expr(token);
        if let TokenKind::BracketClose = token.kind {
            token = token.next.as_ref().unwrap();
            return (tree, token);
        }
        panic!("unexpected");
    }

    if let TokenKind::Number { ref value } = token.kind {
        let tree = Box::new(Tree::Node {
            kind: NodeKind::Number { value: (*value).clone() },
            lhs: Box::new(Tree::Nil),
            rhs: Box::new(Tree::Nil),
        });
        token = token.next.as_ref().unwrap();
        return (tree, token);
    }

    (Box::new(Tree::Nil), token)
}

fn gen(out_file: &mut LineWriter<File>, tree: &Box<Tree>) -> std::io::Result<()> {
    println!("{:?}", tree);
    let (kind, lhs, rhs) = match tree.as_ref() {
        Tree::Nil => panic!("unexpected"),
        Tree::Node{ kind, lhs, rhs } => (kind, lhs, rhs),
    };

    if let NodeKind::Number { value } = kind {
        out_file.write_all(format!("        i32.const {}\n", value).as_bytes())?;
        return Ok(());
    }

    gen(out_file, lhs)?;
    gen(out_file, rhs)?;

    match kind {
        NodeKind::Operator0 { value } => {
            match value {
                Operator0::Plus => out_file.write_all(b"        i32.add\n")?,
                Operator0::Minus => out_file.write_all(b"        i32.sub\n")?,
            };
        },
        NodeKind::Operator1 { value } => {
            match value {
                Operator1::Multiply => out_file.write_all(b"        i32.mul\n")?,
                Operator1::Divide => out_file.write_all(b"        i32.div_s\n")?,
            };
        },
        NodeKind::Number { value: _ } => panic!("unexpected"),
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    match File::open(&args.file) {
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            let token = tokenize(lines)?;
            let (tree, _) = expr(&token);

            let out_file = File::create("./out/main.wat")?;
            let mut out_file = LineWriter::new(out_file);

            out_file.write_all(br#"(module
    (import "wasi_unstable" "proc_exit" (func $_exit (param i32)))
    (func $_start
"#)?;
            gen(&mut out_file, &tree)?;

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
