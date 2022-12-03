use std::{collections::HashMap, fs::File, io::BufReader};

pub fn read_in() -> std::io::Result<HashMap<String, HashMap<String, isize>>>{
    let f = File::open("temp_boss.json")?;
    let file = BufReader::new(f);
    let ai_state = serde_json::from_reader(file)?;
    Ok(ai_state)
}

pub fn read_in2() -> std::io::Result<HashMap<String, HashMap<String, isize>>>{
    let f = File::open("temp_mob.json")?;
    let file = BufReader::new(f);
    let ai_state = serde_json::from_reader(file)?;
    Ok(ai_state)
}