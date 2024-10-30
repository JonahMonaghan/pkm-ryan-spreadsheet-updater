use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::str;

fn read_player_name(dump_path: &str) -> io::Result<String> {
    let mut file = File::open(dump_path)?;

    // Offset for the player's name within WRAM
    let name_offset = 0x24EA4;
    let name_length = 7; // Adjust based on max length of player's name

    // Seek to the player's name location
    file.seek(SeekFrom::Start(name_offset))?;

    // Read bytes for the player's name
    let mut buffer = vec![0; name_length];
    file.read_exact(&mut buffer)?;

    // Convert bytes to a UTF-8 string, trimming null bytes
    let player_name = match str::from_utf8(&buffer) {
        Ok(name) => name.trim_end_matches('\0').to_string(),
        Err(_) => "Invalid name data".to_string(),
    };

    Ok(player_name)
}

fn main() -> io::Result<()> {
    let dump_path = "wram_dump.bin"; // Path to your WRAM dump file

    // Read the player's name from the dump
    let player_name = read_player_name(dump_path)?;

    // Display the name (or later, send to Google Sheets)
    println!("Player's Name: {}", player_name);

    Ok(())
}
