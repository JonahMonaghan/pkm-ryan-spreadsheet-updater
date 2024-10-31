use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;
use dirs::home_dir;

fn create_pokemon_char_map() -> HashMap<u8, char> {
    let mut char_map = HashMap::new();

    // Add mappings for known characters
    //0x0?
    char_map.insert(0x01, 'À');
    char_map.insert(0x02, 'Á');
    char_map.insert(0x03, 'Â');
    char_map.insert(0x04, 'Ç');
    char_map.insert(0x05, 'È');
    char_map.insert(0x06, 'É');
    char_map.insert(0x07, 'Ê');
    char_map.insert(0x08, 'Ë');
    char_map.insert(0x09, 'Ì');
    char_map.insert(0x0B, 'Î');
    char_map.insert(0x0C, 'Ï');
    char_map.insert(0x0D, 'Ò');
    char_map.insert(0x0E, 'Ó');
    char_map.insert(0x0F, 'Ô');

    //0x1?
    char_map.insert(0x10, 'Œ');
    char_map.insert(0x11, 'Ù');
    char_map.insert(0x12, 'Ú');
    char_map.insert(0x13, 'Û');
    char_map.insert(0x14, 'Ñ');
    char_map.insert(0x15, 'ß');
    char_map.insert(0x16, 'à');
    char_map.insert(0x17, 'á');
    char_map.insert(0x19, 'ç');
    char_map.insert(0x1A, 'è');
    char_map.insert(0x1B, 'é');
    char_map.insert(0x1C, 'ê');
    char_map.insert(0x1D, 'ë');
    char_map.insert(0x1E, 'ì');

    //0x2?
    char_map.insert(0x20, 'î');
    char_map.insert(0x21, 'ï');
    char_map.insert(0x22, 'ò');
    char_map.insert(0x23, 'ó');
    char_map.insert(0x24, 'ô');
    char_map.insert(0x25, 'œ');
    char_map.insert(0x26, 'ù');
    char_map.insert(0x27, 'ú');
    char_map.insert(0x28, 'û');
    char_map.insert(0x29, 'ñ');
    char_map.insert(0x2A, 'º');
    char_map.insert(0x2B, 'ª');
    char_map.insert(0x2C, 'r');
    char_map.insert(0x2D, '&');
    char_map.insert(0x2E, '+');

    //0x3?
    char_map.insert(0x34, 'L');
    char_map.insert(0x35, '=');
    char_map.insert(0x36, ';');

    //NO CHARACTERS in 0x4?

    //0x5?
    char_map.insert(0x50, '▯');
    char_map.insert(0x51, '¿');
    char_map.insert(0x52, '¡');
    char_map.insert(0x53, 'P');
    char_map.insert(0x54, 'M');
    char_map.insert(0x55, '▯');
    char_map.insert(0x56, '▯');
    char_map.insert(0x57, '▯');
    char_map.insert(0x58, '▯');
    char_map.insert(0x59, '▯');
    char_map.insert(0x5A, 'Í');
    char_map.insert(0x5B, '%');
    char_map.insert(0x5C, '(');
    char_map.insert(0x5D, ')');

    //0x6?
    char_map.insert(0x68, 'â');
    char_map.insert(0x6F, 'í');

    //0x7?
    char_map.insert(0x79, '↑');
    char_map.insert(0x7A, '↓');
    char_map.insert(0x7B, '←');
    char_map.insert(0x7C, '→');
    char_map.insert(0x7D, '*');
    char_map.insert(0x7E, '*');
    char_map.insert(0x7F, '*');

    //0x8?
    char_map.insert(0x80, '*');
    char_map.insert(0x81, '*');
    char_map.insert(0x82, '*');
    char_map.insert(0x83, '*');
    char_map.insert(0x84, 'ᵉ');
    char_map.insert(0x85, '<');
    char_map.insert(0x86, '>');

    //0xA?
    char_map.insert(0xA1, '0');
    char_map.insert(0xA2, '1');
    char_map.insert(0xA3, '2');
    char_map.insert(0xA4, '3');
    char_map.insert(0xA5, '4');
    char_map.insert(0xA6, '5');
    char_map.insert(0xA7, '6');
    char_map.insert(0xA8, '7');
    char_map.insert(0xA9, '8');
    char_map.insert(0xAA, '9');
    char_map.insert(0xAB, '!');
    char_map.insert(0xAC, '?');
    char_map.insert(0xAD, '.');
    char_map.insert(0xAE, '-');

    //0xB?
    char_map.insert(0xB0, '‥');
    char_map.insert(0xB1, '“');
    char_map.insert(0xB2, '”');
    char_map.insert(0xB3, '‘');
    char_map.insert(0xB4, '\'');
    char_map.insert(0xB5, '♂');
    char_map.insert(0xB6, '♀');
    char_map.insert(0xB7, '円');
    char_map.insert(0xB8, ',');
    char_map.insert(0xB9, 'x');
    char_map.insert(0xBA, '/');
    char_map.insert(0xBB, 'A');
    char_map.insert(0xBC, 'B');
    char_map.insert(0xBD, 'C');
    char_map.insert(0xBE, 'D');
    char_map.insert(0xBF, 'E');

    //0xC?
    char_map.insert(0xC0, 'F');
    char_map.insert(0xC1, 'G');
    char_map.insert(0xC2, 'H');
    char_map.insert(0xC3, 'I');
    char_map.insert(0xC4, 'J');
    char_map.insert(0xC5, 'K');
    char_map.insert(0xC6, 'L');
    char_map.insert(0xC7, 'M');
    char_map.insert(0xC8, 'N');
    char_map.insert(0xC9, 'O');
    char_map.insert(0xCA, 'P');
    char_map.insert(0xCB, 'Q');
    char_map.insert(0xCC, 'R');
    char_map.insert(0xCD, 'S');
    char_map.insert(0xCE, 'T');
    char_map.insert(0xCF, 'U');

    //0xD?
    char_map.insert(0xD0, 'V');
    char_map.insert(0xD1, 'W');
    char_map.insert(0xD2, 'X');
    char_map.insert(0xD3, 'Y');
    char_map.insert(0xD4, 'Z');
    char_map.insert(0xD5, 'a');
    char_map.insert(0xD6, 'b');
    char_map.insert(0xD7, 'c');
    char_map.insert(0xD8, 'd');
    char_map.insert(0xD9, 'e');
    char_map.insert(0xDA, 'f');
    char_map.insert(0xDB, 'g');
    char_map.insert(0xDC, 'h');
    char_map.insert(0xDD, 'i');
    char_map.insert(0xDE, 'j');
    char_map.insert(0xDF, 'k');

    //0xE?
    char_map.insert(0xE0, 'l');
    char_map.insert(0xE1, 'm');
    char_map.insert(0xE2, 'n');
    char_map.insert(0xE3, 'o');
    char_map.insert(0xE4, 'p');
    char_map.insert(0xE5, 'q');
    char_map.insert(0xE6, 'r');
    char_map.insert(0xE7, 's');
    char_map.insert(0xE8, 't');
    char_map.insert(0xE9, 'u');
    char_map.insert(0xEA, 'v');
    char_map.insert(0xEB, 'w');
    char_map.insert(0xEC, 'x');
    char_map.insert(0xED, 'y');
    char_map.insert(0xEE, 'z');
    char_map.insert(0xEF, '►');

    //0xF?
    char_map.insert(0xF0, ':');
    char_map.insert(0xF1, 'Ä');
    char_map.insert(0xF2, 'Ö');
    char_map.insert(0xF3, 'Ü');
    char_map.insert(0xF4, 'ä');
    char_map.insert(0xF5, 'ö');
    char_map.insert(0xF6, 'ü');

    char_map.insert(0xFF, '\0'); // Null terminator (end of string)

    // Add more mappings here as needed

    char_map
}

fn decode_pokemon_string(bytes: &[u8], char_map: &HashMap<u8, char>) -> String {
    let mut result = String::new();

    for &byte in bytes {
        if let Some(&ch) = char_map.get(&byte) {
            if ch == '\0' {
                break; // Stop at null terminator
            }
            result.push(ch);
        } else {
            result.push('?'); // Use '?' for unknown characters
        }
    }

    result
}

fn read_player_name(dump_path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(dump_path)?;

    // Offset and length for the player's name within WRAM
    let name_offset = 0x24EA4;
    let name_length = 7; // Adjust based on max length of player's name

    // Seek to the player's name location
    file.seek(SeekFrom::Start(name_offset))?;

    // Read bytes for the player's name
    let mut buffer = vec![0; name_length];
    file.read_exact(&mut buffer)?;

    // Decode using the Pokémon character map
    let char_map = create_pokemon_char_map();
    let player_name = decode_pokemon_string(&buffer, &char_map);

    Ok(player_name)
}

fn main() -> io::Result<()> {
    let mut dump_path = home_dir().unwrap_or_else(|| PathBuf::from("/"));
    dump_path.push("memdump.bin");

    // Read the player's name from the dump
    match read_player_name(&dump_path) {
        Ok(player_name) => println!("Player's Name: {}", player_name),
        Err(e) => eprintln!("Error reading player name: {}", e),
    }

    Ok(())
}
