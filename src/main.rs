use std::{collections::HashMap, io};

fn main() {
    let mut to_morse: HashMap<char, &str> = HashMap::new();
    let mut morse: HashMap<&str, char> = HashMap::new();
    init_morse_code(&mut to_morse, &mut morse);
    println!("Upisi 1 ili 2 za upisivanje rijeci ili Morseovog koda: ");
    let input = get_input();
    if input.trim().eq("1") {
        println!("Upisi tekst ne upisujući slova kao čćž: ");
        let text = get_input();
        let output = str_to_morse(text.trim(), &to_morse);
        println!("Text to Morse: {}", output);
    } else if input.trim().eq("2") {
        println!("Upisi Morseov kod stavljajuci razmak izmedju slova i kose crte izmedju rijeci: ");
        let code = get_input();
        let output = morse_to_str(code.trim(), &morse);
        println!("Morse to text: {}", output);
    } else {
        println!("Nije upisan 1 ili 2!");
        return;
    }
}

fn str_to_morse(str: &str, to_morse: &HashMap<char, &str>) -> String {
    let mut output = String::new();

    for c in str.chars() {
        let letter = c
            .to_lowercase()
            .to_string()
            .chars()
            .into_iter()
            .next()
            .unwrap();
        output.push_str(
            to_morse
                .get(&letter)
                .expect("Upisao si znak koji ne postoji u Morseovom kodu!"),
        );
        output.push(' ');
    }

    output
}

fn morse_to_str(str: &str, morse: &HashMap<&str, char>) -> String {
    let mut output = String::new();

    let mut letter = String::new();
    for c in str.chars() {
        if c == ' ' || c == '/' {
            if letter.len() != 0 {
                output.push(
                    morse
                        .get(letter.trim())
                        .expect("Upisana Morse kod kombinacija ne postoji.")
                        .clone(),
                );
                letter.clear();
            }
            if c == '/' {
                output.push(' ');
            }
        } else if c == '.' || c == '-' {
            letter.push(c);
        } else {
            println!("Samo je moguce upisivati razmake, tocke, crtice i kose crte.");
        }
    }
    output.push(
        morse
            .get(letter.trim())
            .expect("Upisana Morse kod kombinacija ne postoji.")
            .clone(),
    );

    output
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Greska pri upisu!");
    input
}

fn init_morse_code(to_morse: &mut HashMap<char, &str>, morse: &mut HashMap<&str, char>) {
    to_morse.insert('a', ".-");
    to_morse.insert('b', "-...");
    to_morse.insert('c', "-.-.");
    to_morse.insert('d', "-..");
    to_morse.insert('e', ".");
    to_morse.insert('f', "..-.");
    to_morse.insert('g', "--.");
    to_morse.insert('h', "....");
    to_morse.insert('i', "..");
    to_morse.insert('j', ".---");
    to_morse.insert('k', "-.-");
    to_morse.insert('l', ".-..");
    to_morse.insert('m', "--");
    to_morse.insert('n', "-.");
    to_morse.insert('o', "---");
    to_morse.insert('p', ".--.");
    to_morse.insert('q', "--.-");
    to_morse.insert('r', ".-.");
    to_morse.insert('s', "...");
    to_morse.insert('t', "-");
    to_morse.insert('u', "..-");
    to_morse.insert('v', "...-");
    to_morse.insert('w', ".--");
    to_morse.insert('x', "-..-");
    to_morse.insert('y', "-.--");
    to_morse.insert('z', "--..");
    to_morse.insert('1', ".----");
    to_morse.insert('2', "..---");
    to_morse.insert('3', "...--");
    to_morse.insert('4', "....-");
    to_morse.insert('5', ".....");
    to_morse.insert('6', "-....");
    to_morse.insert('7', "--...");
    to_morse.insert('8', "---..");
    to_morse.insert('9', "----.");
    to_morse.insert('0', "-----");
    to_morse.insert(' ', " / ");

    morse.insert(".-", 'a');
    morse.insert("-...", 'b');
    morse.insert("-.-.", 'c');
    morse.insert("-..", 'd');
    morse.insert(".", 'e');
    morse.insert("..-.", 'f');
    morse.insert("--.", 'g');
    morse.insert("....", 'h');
    morse.insert("..", 'i');
    morse.insert(".---", 'j');
    morse.insert("-.-", 'k');
    morse.insert(".-..", 'l');
    morse.insert("--", 'm');
    morse.insert("-.", 'n');
    morse.insert("---", 'o');
    morse.insert(".--.", 'p');
    morse.insert("--.-", 'q');
    morse.insert(".-.", 'r');
    morse.insert("...", 's');
    morse.insert("-", 't');
    morse.insert("..-", 'u');
    morse.insert("...-", 'v');
    morse.insert(".--", 'w');
    morse.insert("-..-", 'x');
    morse.insert("-.--", 'y');
    morse.insert("--..", 'z');
    morse.insert(".----", '1');
    morse.insert("..---", '2');
    morse.insert("...--", '3');
    morse.insert("....-", '4');
    morse.insert(".....", '5');
    morse.insert("-....", '6');
    morse.insert("--...", '7');
    morse.insert("---..", '8');
    morse.insert("----.", '9');
    morse.insert("-----", '0');
    morse.insert(" / ", ' ');
}
