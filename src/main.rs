use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::io::Write;


fn parse(filename: &str) {

    let filepath = Path::new(filename);
    let filehndl = File::open(&filepath).expect("unable to open file.");

    let mut open_para: bool = false;
    let mut open_h1: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(filehndl);

    for line in reader.lines() {

        let data: String = line.unwrap();
        let mut first: Vec<char> = data.chars().take(1).collect();
        let mut output = String::new();
        
        match first.pop() {
            Some('#') => {

                if open_para {
                    open_para = false;
                    output.push_str("</p>\n\n");
                }

                if open_h1 {
                    // open_h1 = false;
                    output.push_str("</h1>\n");
                }

                open_h1 = true;
                output.push_str("<h1>");
                output.push_str(&data[2..]);
            },
            _ => {

                if open_h1 {
                    open_h1 = false;
                    output.push_str("</h1>\n");
                }

                if !open_para {
                    open_para = true;
                    output.push_str("<p>");
                }

                output.push_str(&data);
            }
        }

        if !output.is_empty() && open_para {
            open_para = false;
            output.push_str("</p>\n\n");
        }

        if output != "<p></p>\n\n" {
            tokens.push(output);
        }
    }

    let mut output_filename = String::from(filename);
    let slice_index = output_filename.find('.').unwrap_or(filename.len());
    output_filename.replace_range(slice_index.., ".html");

    let mut outputfile = File::create(&output_filename)
        .expect("fail");

    for token in &tokens {
        outputfile.write_all(token.as_bytes())
            .expect("fail");
    }

    println!("done! parsed {}, written {}", filename, &output_filename);
}

fn title() -> String {
    return format!("{0} (v{1}), a tiny markdown compiler (by {2})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
}

fn usage() {
    println!("{}", title());
    println!("usage: {} <filename.md>", env!("CARGO_PKG_NAME"));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse(&args[1]),
        _ => usage()
    }
}
