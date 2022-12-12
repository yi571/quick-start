mod settings;
use std::{process::Command, env};

fn main() {
    if !cfg!(target_os = "windows"){
        return;
    }
    println!("開始啟動程序");
    

    let mut appsettings_path = env::current_dir().unwrap();
    appsettings_path.push("appsettings.json");

    let open_listener = settings::get_appsettings(appsettings_path).unwrap();



    for item in open_listener.iter() {
        let mut command = Command::new(&item.path);
        if item.args.len() > 0 {
            for arg in &item.args{
                command.arg(arg);
            }
            
        }

        _ = command.spawn();

        println!("{}啟動指令已發出", item.name);
    }
    
}
