mod main_data;
mod sense_organization;
mod other_structure;
mod test;
mod tags;
mod token_decode;
mod libs;
mod text_decoration;
mod read_input;
use anyhow::{Result};
use tokio;
use std::io::Write;
use serde_json;
async fn request(url:reqwest::Url)->Result<String>{
        let body=reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(body)
}
async fn output_word(word:&str,api_key:&str,writer:&mut std::io::BufWriter<std::fs::File>){
        let url=reqwest::Url::parse_with_params(
        &format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}",word),
        &[("key",api_key)]
    ).unwrap();
    let body=&request(url).await.unwrap();

    let body=serde_json::from_str::<Vec<main_data::Data>>(body);
    writer.write_all(text_decoration::header(word,1).as_bytes()).unwrap();
    match body{
        Err(e)=>panic!("Error in {}{:?}",word,e),
        Ok(body)=>{
            for b in body{
                writer.write_all(&b.all().as_bytes()).unwrap();
                // writer.write_all("<div style=\"page-break-before:always\"></div>".as_bytes()).unwrap();
            }
        }
    }
}
#[tokio::main]
async fn main() {
    // let api_key="04abba36-4844-4168-95c6-418ca6129b4d";
    // let word="traffic";

    // let url=reqwest::Url::parse_with_params(
    //     &format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}",word),
    //     &[("key",api_key)]
    // ).unwrap();
    // let body=&request(url).await.unwrap();
    // // println!("{}",body);
    // // println!("");
    // // println!("{}",&body[4045..]);
    // let body=serde_json::from_str::<Vec<main_data::Data>>(body);
    // match body{
    //     Err(e)=>panic!("{:?}",e),
    //     Ok(body)=>{
    //         let mut writer=std::io::BufWriter::new(std::fs::File::create("product.md").unwrap());
    //         for b in body{
    //             writer.write_all(&b.all().as_bytes()).unwrap();
    //         }
    //     }
    // }
    let mut writer=std::io::BufWriter::new(std::fs::File::create("product.md").unwrap());
    let user_input=read_input::read_input().expect("input.json is invalid");
    for word in user_input.words{
        output_word(&word,&user_input.api_key,&mut writer).await;
    }


}