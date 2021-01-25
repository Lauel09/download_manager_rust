use goog_api::{ ToDown, do_stuffs};
use std::env::args;
fn main() {
    let prog_args = args().collect::<Vec<String>>();

    let _x = do_stuffs(&prog_args);

    let mut obj = ToDown::new(
        prog_args[2].clone(),
        prog_args[6].as_str(),
        prog_args[4].as_str(),
    );

    if obj.download() {
        println!("Succeeded");
    } else {
        println!("Failed");
    }
    
    obj.print_all();

}


