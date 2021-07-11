use std::env::args;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::io::{BufReader, BufRead, Write};

fn main() {
    let args: Vec<String> = args().collect();
    let file = &args[1];

    if !Path::new(file).exists() {
        println!("Can't find {}", file);
        exit(1);
    } else {
        println!("Compiling {}...", file);
    }

    let output_file_name = file.replace(".md", ".html");
    let mut output_file = File::create(output_file_name).unwrap();

    let mut md_reader = BufReader::new(File::open(file).expect("Couldn't find file"));

    output_file.write_fmt(format_args!("{}", "<html>\n<body>\n"));

    for line in md_reader.lines() {
        let mut md_line = line.unwrap();

        if md_line.contains("#") {
            // <h1>
            md_line = md_line.replace("#", "<h1>");
            md_line.push_str("</h1>");
        } else if md_line.contains("##") {
            // <h2>
            md_line = md_line.replace("##", "<h2>");
            md_line.push_str("</h2>");
        } else if md_line.contains("###") {
            md_line = md_line.replace("###", "<h3>");
            md_line.push_str("</h3>");
        } else if md_line.contains("####") {
            md_line = md_line.replace("####", "<h4>");
            md_line.push_str("</h4>");
        } else if md_line.contains("#####") {
            md_line = md_line.replace("#####", "<h5>");
            md_line.push_str("</h5>");
        } else if md_line.contains("######") {
            md_line = md_line.replace("######", "<h6>");
            md_line.push_str("</h6>");
        } else if md_line.contains("#######") {
            md_line = md_line.replace("#######", "<h7>");
            md_line.push_str("</h7>");
        } else if md_line.contains("########") {
            md_line = md_line.replace("########", "<h8>");
            md_line.push_str("</h8>");
        } else if md_line.trim().is_empty() {
            // Don't do anything
        } else {
            md_line.push_str("</p>");
            let p = String::from("<p>") + md_line.as_str();
            md_line = p;
        }

        output_file.write_fmt(format_args!("{}\n", md_line));
    }

    output_file.write_fmt(format_args!("{}", "\n</body>\n</html>"));
}
