use std::fs::File;
use std::io::{self, BufRead, LineWriter};
use std::io::prelude::*;
use std::path::Path;

fn make_href(title: String, link: String) -> String {
    "[".to_owned() + &title + "]" + "(" + &link + ")"
}

fn main() -> std::io::Result<()> {
    let prefix: String = "https://github.com/krshrimali/rust-leetcode/blob/main/".to_string();
    let readme_prefix: String = "# rust-leetcode

**Solving Leetcode problems in Rust**
"
    .to_string();
    let readme_suffix: String = "
Full disclosure: I was inspired by [UncleScientist](https://www.youtube.com/c/UncleScientist) to do it this way, specially the testing.".to_string();

    let file = File::create("README.md")?;
    let mut file = LineWriter::new(file);
    file.write_all(readme_prefix.as_bytes())?;
    file.write_all(b"\n")?;

    if let Ok(lines) = read_lines("src/lib.rs") {
        let mut count: i32 = 1;
        for line in lines {
            let ip = line.expect("couldn't find a value");
            if ip.contains("pub mod ") {
                let mut str_ip = ip.to_string();
                str_ip = str_ip
                    .strip_prefix("pub mod ")
                    .expect("Couldn't find prefix: pub mod ")
                    .to_string();
                str_ip = str_ip
                    .strip_suffix(';')
                    .expect("Couldn't find semicolon at the end: ;")
                    .to_string();
                let file_path = "src/".to_string() + &str_ip + ".rs";
                if let Ok(file_lines) = read_lines(&file_path) {
                    let mut stripped_link: String = String::from("");
                    let mut stripped_title: String = String::from("");
                    let mut code_str: String = String::from("");
                    for (count, file_line) in file_lines.into_iter().enumerate() {
                        if count == 2 {
                            break;
                        }
                        let comment: String = file_line.expect("no string found").to_string();
                        let title_line = comment.strip_prefix("// Title: ");
                        if let Some(title) = Some(title_line) {
                            if title.is_some() {
                                stripped_title = title.as_ref().unwrap().to_string();
                            }
                        }

                        let link_line = comment.strip_prefix("// Link: ");
                        if let Some(link) = Some(link_line) {
                            if link.is_some() {
                                stripped_link = link.as_ref().unwrap().to_string();
                            }
                        }

                        let file_name_str = format!("`{str_ip}`");
                        let mut code_path: String = prefix.to_owned();
                        code_path.push_str(&file_path);
                        let file_name_code_path = code_path;
                        code_str = make_href(file_name_str, file_name_code_path);
                    }
                    if !stripped_title.is_empty() && !stripped_link.is_empty() {
                        let title_str = make_href(stripped_title, stripped_link);
                        println!("title str: {}", title_str);
                        println!("{:?}", code_str);
                        let mut final_str = count.to_string() + ". " + &title_str + ": ";
                        final_str.push_str(&code_str);
                        file.write_all(final_str.as_bytes())?;
                        file.write_all(b"\n")?;
                        count += 1;
                    }
                }
            }
        }
    }

    file.write_all(readme_suffix.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
