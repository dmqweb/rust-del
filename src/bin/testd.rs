use std::{env, fs};
use std::path::Path;
fn main(){
    let src = env::args().collect::<Vec<String>>()[1].clone();
    println!("目录：{src}");
    let path = Path::new(&src);
    println!("路径：{path:?}");
    if let Ok(entries) = fs::read_dir(path){
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata(){
                if metadata.is_file(){
                }else {
                }
            }
        }
    }
}
