use std::env;
use std::fs;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    handle_dir(file_path);
}
fn handle_file(file:&str){
    let content = fs::read_to_string(file).expect("读取文件失败");
    let modified_content = remove_comments_and_empty_lines(content);
    fs::write(file, modified_content).expect("写入文件失败");
}
fn handle_dir(file:&str){
    if Path::new(file).is_dir(){
        if let Ok(entries) = fs::read_dir(file){
            for entry in entries.flatten(){
                println!("处理子文件：{entry:?}");
                handle_dir(entry.path().to_str().unwrap());
            }
        }
    }else {
        handle_file(file)
    }
}
fn remove_comments_and_empty_lines(content: String) -> String {
    let mut string = "".to_string();
    for item in content.lines(){
        let trimed = item.trim();
        if !trimed.starts_with("//") && !trimed.trim().is_empty(){
            string.push_str(format!("{item}\r\n").as_str());
        }
    };
    string
}
