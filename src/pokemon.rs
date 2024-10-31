
#[derive(Debug)]
struct Pokemon{
    name: String,
    ot_name: String,
    level: u8,
    moves: [Move; 4],
    current_hp: u16,
    stats: Stats,
}

#[derive(Debug)]
struct Stats{
    total_hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special_attack: u16,
    special_defense: u16,
}

#[derive(Debug)]
struct Move{
    name: String,
    pp: u8,
    power: u8,
    accuracy: u8,
    move_type: MoveType
}

#[derive(Debug)]
enum MoveType{
    Physical,
    Special,
    Status,
}

#[derive(Debug)]
enum StatusCondition{
    Sleep,
    Poison,
    Burn,
    Freeze,
    Paralysis,
    BadPoison,
}