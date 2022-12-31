use std::env;
use std::fs;


const SAMPLE: &str = 
r#"Provided by The Internet Classics Archive.
Available online at http://classics.mit.edu/Plato/phaedrus.html

Phaedrus
By Plato


Translated by Benjamin Jowett

Persons of the Dialogue
SOCRATES
PHAEDRUS.

Scene
Under a plane-tree, by the banks of the Ilissus.
----------------------------------------------------------------------

Socrates. My dear Phaedrus, whence come you, and whither are you going?

Phaedrus. I come from Lysias the son of Cephalus, and I am going to
take a walk outside the wall, for I have been sitting with him the
whole morning; and our common friend Acumenus tells me that it is
much more refreshing to walk in the open air than to be shut up in
a cloister. 

Soc. There he is right. Lysias then, I suppose, was in the town?

Phaedr. Yes, he was staying with Epicrates, here at the house of 
Morychus;
that house which is near the temple of Olympian Zeus. 

Soc. And how did he entertain you? Can I be wrong in supposing that
Lysias gave you a feast of discourse? 

Phaedr. You shall hear, if you can spare time to accompany me.

Soc. And should I not deem the conversation of you and Lysias "a thing
of higher import," as I may say in the words of Pindar, "than any
business"? 

Phaedr. Will you go on? 

Soc. And will you go on with the narration? 

Phaedr. My tale, Socrates, is one of your sort, for love was the theme
which occupied us -love after a fashion: Lysias has been writing about
a fair youth who was being tempted, but not by a lover; and this was
the point: he ingeniously proved that the non-lover should be accepted
rather than the lover. 

Soc. O that is noble of him! I wish that he would say the poor man
rather than the rich, and the old man rather than the young one; then
he would meet the case of me and of many a man; his words would be
quite refreshing, and he would be a public benefactor. For my part,
I do so long to hear his speech, that if you walk all the way to Megara,
and when you have reached the wall come back, as Herodicus recommends,
without going in, I will keep you company."#;

fn main() {

    let mut lines = SAMPLE.lines();
    let mut current = "";
    // advance iteration of text until reaching the body text
    while current.is_empty() || !current.chars().all(|c| c == '-') {  // i.e when finding the "----" string
        current = lines.next().expect("error during skip").trim();
    }
    
    let mut formatted_text: String = String::new();
    let mut on_new_paragraph: bool = true;
    // format body text
    for current in lines {
        if current.is_empty() && !formatted_text.is_empty() {   // OBS! Does not account for sequential line breaks
            formatted_text.push_str(r#"\\"#);
            formatted_text.push_str("\n");
            on_new_paragraph = true;
        } else {
            let mut text: String = current.to_string();
            if on_new_paragraph { // check if entering a new paragraph
                let first_word = text.split(' ').next().unwrap();
                if first_word.chars().any(|c| c == '.') { // if first word is a speaker, make the speaker name bold.
                    let bold = format!("\\textbf{{{}}} ", first_word);
                    let mut new_text = String::new();
                    new_text.push_str(&bold);
                    for word in text.split(' ').skip(1) {
                        new_text.push_str(&(word.to_owned() + " "));
                    }
                    text = new_text;                    
                }
                on_new_paragraph = false;
            }
            formatted_text.push_str("\n");
            formatted_text.push_str(&text);
        }
    }

    println!("{}", formatted_text);

}
