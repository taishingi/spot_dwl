use notify_rust::Notification;
use std::{
    env::args,
    fs,
    path::Path,
    process::{exit, Command, ExitCode, Stdio},
};
///
/// adazaz
///
fn main() -> ExitCode {
    let args: Vec<String> = args().collect();

    let p: String = format!("{}/.icons", env!("HOME"));
    let f = format!("{p}/spot_dwl.png");
    if !Path::new(p.as_str()).is_dir() {
        fs::create_dir(&p).expect("Fail to create the icons directory");
    }
    if !Path::new(f.as_str()).is_file() {
        Command::new("wget")
            .arg("https://raw.githubusercontent.com/taishingi/spot_dwl/master/icons/spot_dwl.png")
            .spawn()
            .expect("failed to get icon");
        fs::copy("./spot_dwl.png", p.as_str()).expect("msg");
        fs::remove_file("./spot_dwl.png").expect("failed to remove file");
    }
    if args.len() == 1 {
        println!("missing query");
        exit(1);
    }
    for (x, item) in args.iter().enumerate().skip(1) {
        Command::new("spotdl")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("--config")
            .arg(item.as_str())
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to parse {x} list retry to run spot_dwl {item}"));

        Notification::new()
            .summary("Spotify Downloader")
            .body(format!("Query {x} downloaded").as_str())
            .icon("spot_dwl")
            .show()
            .expect("Missing notify-send");
    }
    exit(0);
}
