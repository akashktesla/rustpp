#![allow(warnings)]
use std::fs::File;
use std::fs::write;
use std::fs::create_dir;
use std::io::prelude::*;
use serde_json::Value as JsonValue;
use serde::de::DeserializeOwned;
fn main(){

}

pub fn create_file(path:String,file_name:String){
match create_dir(&path){
    Ok(v) => (),
    Err(e) => () ,
}
match File::create(format!("{}/{}",path,file_name)){
    Ok(v) => (),
    Err(e) => () ,
}
}//end

pub fn read_file(path:&String)->String{
    let mut _file = File::open(path).expect("can't open file");
    let mut contents = String::new();
    _file.read_to_string(&mut contents).expect("can't read the file");
    contents = format!(r#"{}"#,contents); 
    return contents
}

pub fn write_file(path:&String,contents:&String){
    match write(path,contents){
    Ok(v) =>(),
    Err(e) => () ,
    }
}

pub fn read_json(contents:&String)->JsonValue {
    let res =serde_json::from_str(&contents);
    if res.is_ok(){
        let p:JsonValue = res.unwrap();
        return p
    }
    else{
        panic!("something went wrong!!")
    } 

}

pub fn write_json(path:&String,contents:&JsonValue){
    let con = contents.to_string();
    write_file(path,&con);
}

pub fn sync_json<T:DeserializeOwned>(contents:&String)->T {
    let res =serde_json::from_str(&contents);
    if res.is_ok(){
        let p:T = res.unwrap();
        return p
    }
    else{
        panic!("something went wrong!!")
    } 

}

//extracts data from path and converts into appropriate structure
pub fn extractor <T:DeserializeOwned> (_name:String,_path:String)->T{
    let contents:String=read_file(&format!("{}",_path));
    let data:JsonValue = read_json(&contents);
    let data:String = data[_name].to_string(); 
    let returns:T = sync_json(&data);
    returns
}

pub fn store_var(path:&String,key:&String,value:&String){
    println!("path uh {}",path);
    let rcon = read_file(path);
    let mut rjcon = read_json(&rcon);
    rjcon[key]=JsonValue::from(value.to_string());
    write_json(path,&rjcon);
}

pub fn retrive_var(path:&String,key:&String)->String{
    println!("path uh {}",path);
    let rcon = read_file(path);
    let rjcon = read_json(&rcon);
    rjcon[key].to_string()
}
