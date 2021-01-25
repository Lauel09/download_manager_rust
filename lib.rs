use std::io::Write;
use core::panic;
use std::{fs::OpenOptions};
use std::{error::Error};
use std::{fs::File, path::Path, process::exit};
pub enum State {
    Cont = 0,
    Exit = 1,
}

use State::{Cont, Exit};

pub struct ToDown<'a> {
    url: String,
    file_path: String,
    file_ext: &'a str,
    file: File,
}
pub enum Sig {
    InvalidUrl = 0,
    UrlIsOk = 1,
}
impl<'a> ToDown<'_> {
    pub fn new(url: String, folder_path: &str, file_name:& str) -> ToDown<'a> {
        let proper_path = check_args(folder_path);
        
        if check_url(&url) == false {
            eprintln!("Invalid Url");
            //Exit if url is invalid
            exit(1);
        }
         
        let file_ext_1 = check_file_ext(&url);
        
        let (file,path) = crt_file_hand_err(file_name, file_ext_1, proper_path);

        ToDown {
            url,
            file_path: path,
            file_ext:file_ext_1,
            file,
        }
        
    }
    
    pub fn download(&mut self) -> bool {
        if self.url == "INVALID" {
            println!("Given URL is invalid");
            return false;
        }

        let _x = download_file(&self.url, &mut self.file);

        _x.is_ok()
    }

   pub fn print_all(&self) {
        println!(
            "\n\tPath where stored: {}\n\tExtension: {},\n\tURL of the downloaded file: {}",
            self.file_path,self.file_ext, self.url
        )

    }
}

pub fn check_file_ext(url: &String) -> &'static str {
    let mut _file_extension: &str = "";

    let file_ext_list = [
        "jpg", "jpeg", "txt", "png", "bmp", "mp3", "mp4", "zip", "ico",
    ];

    for i in 0..file_ext_list.len() {
        if url.contains(file_ext_list[i]) {
            _file_extension = file_ext_list[i];
        }
    }
    if _file_extension.is_empty() {
        eprintln!("--------------------------------------------------------------------------------------------\n\t\tUNABLE TO INDENTIFY FILE EXTENSION\n--------------------------------------------------------------------------------------------");
        exit(1);
    }
    
    _file_extension
}

fn check_args(file_path: &str) -> String {
    let mut doc = match std::env::var("HOME") {
        Ok(val) => val,
        Err(e) => panic!("Not found HOME:{}", e),
    };
    
    if file_path == "home" {
        
        doc.push('/');//becomes something like '/home/user_name/'
        doc
        
    } else if file_path == "documents" || file_path == "Documents" {
        
        doc.push_str("/Documents/");
        //This push makes it something like '/home/user_name/Documents'
        return doc;
        
    } else if file_path.starts_with("Documents") {
        
        doc.push_str(format!("/{}", file_path).as_str());
        doc
        
    } else if file_path == "/Documents" {
        doc.push_str("/Documents/");
        // If it is just simply /Documents, we need to make sure the proper path
        // becoz home is `/home/user_name not` `/home/user_name/`
        // If we will not push the `/' then,
        // '/home/sol/Documents`  != '/home/solDocuments
        doc
    }
     else { 
        
        return file_path.to_string();
    
    }
}


pub fn download_file(url: &String, file_name: &mut File) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url.as_str())?;
    
    file_name.write_all(&response.bytes()?)?;

    
    Ok(())
}

pub fn crt_file_hand_err(
    name: &str,
    file_ext: &str,
    file_path: String,
) -> (File,String) {
    
    let mut file_path_new = file_path.clone();
   
    let mut buffer = String::new();

    if file_ext == "None" {
        file_path_new.push_str(name);
    } else {
        file_path_new.push_str(format!("{}.{}", name, file_ext).as_str());
    }
    let path = Path::exists(Path::new(file_path_new.as_str()));
    
    if path {
        println!("\tA file with name {} already exists\n\tType (y) for overwriting it\n\tOr (n<space><new_file_name>) for creating a new name\n\tNOTE:FOLDER PATH WOULD REMAIN SAME:-",name);
        
        std::io::stdin().read_line(&mut buffer).expect("Failed to take input");
        let buffer = buffer.trim();
        
            if buffer == "y" {

                let file = create_file(name,file_path_new.to_string(),file_ext);
    
                return (file,file_path_new);
                }
            
            else if buffer.starts_with("n"){
                let  x = buffer.split(' ').collect::<Vec<_>>();  
                
                let file = create_file(x.last().unwrap(), file_path.clone(), file_ext);
                
                return (file,file_path);
                
                }
            
            else {
            panic!("Invalid Input!");  
            }
        }
        

    else {
        let  file = match OpenOptions::new().create(true).write(true).open(Path::new(file_path_new.as_str())) {
            Ok(file) => file,
            Err(e) => panic!("UNABLE TO CREATE FILE:{}",e),
        };
        return (file,file_path_new);
        
    }
}

fn create_file(name:&str,file_path: String,file_ext: &str)-> File {
    
    //Giving a folder path to OpenOptions creates an empty file with no extension
    // So never let any folder path come to this guy, it will turn out to be awful
        let mut file_full_name = String::from("NONE");
        if file_path.ends_with("/") {
            file_full_name = format!("{}{}.{}",file_path,name,file_ext);
        }
        
        else if Path::exists(Path::new(file_full_name.as_str())) {
            
            let (file,_) = crt_file_hand_err(name, file_ext, file_path);
            return file;
        }
        else {
            format!("{}/{}.{}",file_path,name,file_ext);
        }
       
    let file = match OpenOptions::new().write(true).create(true).open(Path::new(file_full_name.as_str())) {
        Ok(file) => file,
        Err(e) => panic!("CREATING_FILE:{}",e),
    };
    return file;
}

pub fn usage() {
    println!("\t\tPROGRAM USAGE");
    println!("\t\t./<program_name> --url <your_url> --fn <file_name> --fld <folder_path>");
}
pub fn do_stuffs(x:&Vec<String>) -> State {
    
    match x.len() {
        1 => {usage();exit(1);},
        2 => {
            match x[1].as_str() {
                "--help" => {help();exit(1);},
                _ => {invalid(x.len());exit(1)},
            }
        }
        7  => {
            if x[1] == "--url" && x[3] == "--fn" && x[5] == "--fld" {
                let res = check_url(x[2].as_str());
                if res == true {
                    Cont
                }
                else {
                    Exit
                }
            }
            else {
                Exit
            }
        }
        _ => {usage();exit(1);},

    }
}

fn check_url(url: &str) -> bool {
    if url.starts_with("https://") || url.starts_with("http://") {
        if url.contains("www.") {
             true
        }
        else {
            false
        }
    }
    else {
        false
    }
}

fn invalid(prg_args: usize)  {
    println!("\t\tNot enough arguments\n\t\tCURRENT COUNTER:{}",prg_args - 1);
    usage();
}
fn help() {
    println!("\t\t HELP");

    println!("\t\t --url - option to include url");

    println!("\t\t --fn - Name by which the feel needs to be saved");

    println!("\t\t --fld - folder where file needs to be saved,\n\t\t you can also write direct path like Documents or home.");
}

