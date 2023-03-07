use core::panic;
// use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
// use std::io::Read;
// use std::io::Write;

const PATH: &str = "texts/sample-formatted.txt";

fn main() {
    

    // create iterator for text lines
    let text_file = File::open(PATH).expect("error when opening file, file does not exist?");
    let file_size = match fs::metadata(PATH) {
        Ok(md) => md.len() as usize,
        Err(_) => panic!("should not happen"),
    };

    let mut lines = BufReader::new(text_file).lines();

    let mut formatted_text: String = String::with_capacity(file_size + file_size/10); // obs: not very memory efficient
    let mut on_new_paragraph: bool = true;

    // format head text
    let mut cur = lines.next().unwrap().expect("iterator should not be empty during parsing of head text!");
    while cur.is_empty() || !cur.chars().all(|c| c == '-') {
        formatted_text.push_str(&cur);
        if !cur.is_empty() {
            formatted_text.push_str(r#" \\"#)
        }
        formatted_text.push_str("\n");
        cur = lines.next().unwrap().expect("iterator should not be empty during parsing of head text!");
    }
    formatted_text.push_str(&cur);
    formatted_text.push_str("\n");
    
    // format body text
    for line in lines {
        let current = line.expect("'line' should never be of type 'Err' during parsing");
        if current.is_empty() {   // OBS! Does not account for sequential line breaks?
            formatted_text.push_str(r#"\\"#);
            formatted_text.push_str("\n");
            on_new_paragraph = true;
        } else {
            // let mut current: String = current.to_string();
            if on_new_paragraph { // check if entering a new paragraph
                on_new_paragraph = false;

                let first_word = current.split_whitespace().next().expect("variable 'current' should never be of type 'None'"); // next().unwrap().trim();
                if is_speaker(first_word) { // if first word is a speaker, make the speaker name bold.
                    //let new = bold_first_word(&current);
                    formatted_text.push_str("\n");
                    formatted_text.push_str(&bold_first_word(&current));
                    continue;
                }
            }
            formatted_text.push_str("\n");
            formatted_text.push_str(&current);
        }
    }

    let new_path = {
        let ps: Vec<&str> = PATH.split('.').collect();
        format!("{}-formatted.{}", ps[0], ps[1])
    };

    match fs::write(new_path, &formatted_text) {
        Ok(_) => println!("Done!"),
        Err(e) => panic!("failed to write to file: {:?}", e),
    }
}

fn bold_first_word<'a>(line: &'a str) -> String {   // currently formatting for LaTex
    let mut res = String::with_capacity(line.len() + r"\textbf{} ".len());
    let mut iter = line.split(' ');
    res.push_str(r"\textbf{");
    res.push_str(iter.next().unwrap());
    res.push_str("} ");
    for w in iter {
        res.push_str(w);
        res.push_str(" ");
    }
    res
}

fn is_speaker(w: &str) -> bool {    // OBS: very specific 
    return w.chars().next_back().unwrap() == '.';
}
