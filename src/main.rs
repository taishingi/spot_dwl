use std::{
    env::args,
    fs,
    path::Path,
    process::{exit, Command, ExitCode},
};

use notifme::Notification;

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
        assert!(Command::new("wget")
            .arg("https://raw.githubusercontent.com/taishingi/spot_dwl/master/icons/spot_dwl.png")
            .arg("-q")
            .spawn()
            .expect("failed to get icon")
            .wait()
            .expect("msg")
            .success());
        assert!(fs::copy("spot_dwl.png", f.as_str()).is_ok());
        assert!(fs::remove_file("spot_dwl.png").is_ok());
    }
    if args.len() == 1 {
        println!("missing query");
        exit(1);
    }
    let l = args.len() - 1;
    assert!(Notification::new()
        .summary("Spotify Downloader")
        .body(format!("Started to download {l} query").as_str())
        .icon("spot_dwl")
        .send());
    for (x, item) in args.iter().enumerate().skip(1) {
        assert!(Command::new("clear")
            .spawn()
            .expect("windows")
            .wait()
            .expect("msg")
            .success());
        assert!(Command::new("spotdl")
            .arg("--config")
            .arg(item.as_str())
            .spawn()
            .expect("spotdl not founded")
            .wait()
            .expect("msg")
            .success());

        assert!(Notification::new()
            .summary("Spotify Downloader")
            .body(format!("Query {x}/{l} downloaded successfully").as_str())
            .icon("spot_dwl")
            .send());
    }

    if l >= 2 {
        assert!(Notification::new()
            .summary("Spotify Downloader")
            .body(format!("Finnish to downloaded {l} queries").as_str())
            .icon("spot_dwl")
            .send());
    } else {
        assert!(Notification::new()
            .summary("Spotify Downloader")
            .body(format!("Finnish to downloaded {l} query").as_str())
            .icon("spot_dwl")
            .send());
    }

    exit(0);
}
