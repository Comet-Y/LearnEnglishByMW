use std::io;
use std::fs;
use serde::{Deserialize,Serialize};
use anyhow::Result;
use serde_json::from_str;
#[derive(Deserialize,Serialize)]
pub struct UserInput{
    pub api_key:String,
    pub words:Vec<String>
}

pub fn read_input()->Result<UserInput>{
    let user_input_file=fs::File::open("input.json")?;
    let reader=io::BufReader::new(user_input_file);
    let ret=from_str::<UserInput>(&io::read_to_string(reader)?)?;
    Ok(ret)

}