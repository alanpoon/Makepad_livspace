#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

use std::{
    fs::File,
    io::prelude::*,
    path::Path
};
#[wasmedge_bindgen]
pub fn read_foo_text() ->String{
    match File::open(Path::new("/data/user/0/dev.makepad.makepad_livspace/cache").join("foo.txt")){
        Ok(mut f) =>{
            let mut data = String::from("");

            match f.read_to_string(&mut data){
                Ok(_)=>{return data}
                Err(e)=>{
                    return format!("nnn {:?}",e);
                }
            }
        }
        Err(e) =>{
            return format!("mmm {:?}",e);

        }

    }
   
    String::from("nothing")
}
