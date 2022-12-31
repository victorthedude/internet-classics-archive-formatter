// use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
// use std::io::Read;
// use std::io::Write;

const PATH: &str = "texts/phaedrus.txt";

fn main() {
    // read lines from file
    let read = File::open(PATH).expect("error when opening file");
    let mut lines = BufReader::new(read).lines();

    let mut current = String::new();
    // advance iteration of text until reaching the body text
    while current.is_empty() || !current.chars().all(|c| c == '-') {  // i.e when finding the "----" string
        // let next = &lines.next();
        let line = &lines.next().unwrap();
        match line {
            Ok(s) => current = s.to_owned(),
            Err(_) => panic!("error during skipping of head text"),
        }
    }
    
    let mut formatted_text: String = String::new();
    let mut on_new_paragraph: bool = true;
    // format body text
    for line in lines.skip(1) { // skip the blank line after the "----" string
        let mut current = line.expect("error when parsing lines");
        if current.is_empty() {   // OBS! Does not account for sequential line breaks
            formatted_text.push_str(r#"\\"#);
            formatted_text.push_str("\n");
            on_new_paragraph = true;
        } else {
            // let mut current: String = current.to_string();
            if on_new_paragraph { // check if entering a new paragraph
                let first_word = current.split(' ').next().unwrap().trim();
                if first_word.chars().any(|c| c == '.') { // if first word is a speaker, make the speaker name bold.
                    let bold = format!("\\textbf{{{}}} ", first_word); // currently formatting for LaTex 
                    let mut new_text = String::new();
                    new_text.push_str(&bold);
                    for word in current.split(' ').skip(1) {
                        new_text.push_str(&(word.to_owned() + " "));
                    }
                    current = new_text;                    
                }
                on_new_paragraph = false;
            }
            formatted_text.push_str("\n");
            formatted_text.push_str(&current);
        }
        // println!();
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
