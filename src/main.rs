use std::fs;
use std::io;

fn encode_to_shit(textfile: &String, shitfile: &String, usefile: i32) -> u16 {
    let text;

    if usefile == 1 {
        text = fs::read_to_string(textfile).unwrap();
    } else {
        text = textfile.clone();
    }

    let mut data = Vec::new();

    // Header
    data.extend_from_slice(b"SHIT1");

    // Encode
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        // Commands starting with '\'
        if c == '\\' {
            match chars.peek() {
                // \n = newline
                Some('n') => {
                    chars.next();
                    data.extend_from_slice(&0x80u32.to_le_bytes());
                    continue;
                }

                // \i = italic ON
                Some('i') => {
                    chars.next();
                    data.extend_from_slice(&0x81u32.to_le_bytes());
                    continue;
                }

                // \b = bold ON
                Some('b') => {
                    chars.next();
                    data.extend_from_slice(&0x83u32.to_le_bytes());
                    continue;
                }

                // Unknown command
                _ => {}
            }
        }

        // i/ = italic OFF
        if c == 'i' && chars.peek() == Some(&'/') {
            chars.next();
            data.extend_from_slice(&0x82u32.to_le_bytes());
            continue;
        }

        // b/ = bold OFF
        if c == 'b' && chars.peek() == Some(&'/') {
            chars.next();
            data.extend_from_slice(&0x84u32.to_le_bytes());
            continue;
        }

        // Normal character
        let encoded = ((c as u32) << 7) + 1;
        data.extend_from_slice(&encoded.to_le_bytes());
    }

    fs::write(shitfile.to_owned() + ".shit", &data).unwrap();

    0
}

fn decode_to_text(shitfile: &String) -> u16 {
    let input = fs::read(shitfile).unwrap();

    if input.len() < 5 {
        println!("File is too small!");
        return 0;
    }

    if &input[0..5] != b"SHIT1" {
        println!("Invalid file format!");
        return 0;
    }

    for chunk in input[5..].chunks_exact(4) {
        let encoded = u32::from_le_bytes([
            chunk[0],
            chunk[1],
            chunk[2],
            chunk[3],
        ]);

        match encoded {
            // Newline
            0x80 => {
                print!("\n");
            }

            // Italic ON
            0x81 => {
                print!("\x1b[3m");
            }

            // Italic OFF
            0x82 => {
                print!("\x1b[0m");
            }

            // Bold ON
            0x83 => {
                print!("\x1b[1m");
            }

            // Bold OFF
            0x84 => {
                print!("\x1b[0m");
            }

            // Normal character
            _ => {
                if encoded < 129 {
                    println!("Invalid character encoding!");
                    continue;
                }

                let original = char::from_u32((encoded - 1) >> 7);

                match original {
                    Some(c) => print!("{}", c),
                    None => println!("Invalid Unicode character!"),
                }
            }
        }
    }

    0
}

fn main() {
    println!("-----SHIT-CONVERTER-AND-READER-----");

    loop {
        println!(
            "Do you want to encode text to shit or decode shit to text (d/e) or make a new file"
        );

        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();

        inputs = inputs.trim().to_string();

        // Decode
        if inputs.to_lowercase() == "d" {
            inputs.clear();

            println!("Enter your shit file");
            io::stdin().read_line(&mut inputs).unwrap();

            inputs = inputs.trim().to_string();

            decode_to_text(&inputs);
        }

        // Encode existing text file
        else if inputs.to_lowercase() == "e" {
            inputs.clear();

            println!("Enter your text file");
            io::stdin().read_line(&mut inputs).unwrap();

            let textfile = inputs.trim().to_string();

            inputs.clear();

            println!("Enter your shitfile name");
            io::stdin().read_line(&mut inputs).unwrap();

            let shitfile = inputs.trim().to_string();

            encode_to_shit(&textfile, &shitfile, 1);

            println!("Encoded successfully!");
        }

        // Make new SHIT file from typed text
        else if inputs.to_lowercase() == "n" {
            inputs.clear();

            println!("Enter the name to save it with");
            io::stdin().read_line(&mut inputs).unwrap();

            let filename = inputs.trim().to_string();

            inputs.clear();

            println!("Enter the file contents");
            io::stdin().read_line(&mut inputs).unwrap();

            encode_to_shit(&inputs, &filename, 0);

            println!("Encoded successfully!");
        }
    }
}
