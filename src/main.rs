use std::process::Command;

//use execute::Execute;

// use bindings::{ 
//     Windows::Win32::WindowsAndMessaging::*,
// };

fn main() {
    if !cfg!(target_os = "windows"){
        return;
    }
    println!("開始啟動程序");
    //let open_listener = ["C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe"];

    let open_listener = [ 
        &OpenItem { name: "Telegram", path: "D:\\Program Files\\Telegram Desktop\\Telegram.exe", arg: "" },
        &OpenItem { name: "firefox", path: "C:\\Program Files\\Mozilla Firefox\\firefox.exe", arg: "" },
        &OpenItem { name: "vs code", path: "C:\\Program Files\\Microsoft VS Code\\Code.exe", arg: "" },
        &OpenItem { name: "line", path: "explorer.exe", arg: "shell:appsfolder\\NAVER.LINEwin8_8ptj331gd3tyt!LINE" },
        //&OpenItem { name: "chrome", path: "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe", arg: "" },
    ];

    for item in open_listener.iter() {
        let mut command = Command::new(item.path);
        if item.arg != "" {
            command.arg(item.arg);
        }

        // if let Some(exit_code) = command.execute().unwrap() {
        //     if exit_code == 0 {
        //         println!("{}已啟動", item.name);
        //     } else {
        //         eprintln!("Failed.");
        //     }
        // } else {
        //     eprintln!("Interrupted!");
        // }

        _ = command.spawn();

        //println!("{:#?}", output);

        println!("{}啟動指令已發出", item.name);
    }
    //println!("啟動程序執行完成");
    // unsafe{
    //     MessageBoxW(None, "啟動程序執行完成", "", MESSAGEBOX_STYLE::MB_OK);
    // }

    
}

struct OpenItem<'a>{
    name: &'a str,
    path: &'a str,
    arg: &'a str,
}