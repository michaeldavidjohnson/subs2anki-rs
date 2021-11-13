use std::env;
use std::process;
fn main() {
    let _args: Vec<String> = env::args().collect();
    let filepaths = &_args[1..];
    for file in filepaths{
        get_subtitle(file);
    }
}

fn get_subtitle(file:&String){

    let metadata = process::Command::new("ffprobe")
        .arg("-loglevel")
        .arg("error")
        .arg("-select_streams")
        .arg("s")
        .arg("-show_entries")
        .arg("stream=index:stream_tags=language")
        .arg("-of")
        .arg("csv=p=0")
        .arg(file.to_string())
        .stdout(process::Stdio::piped())
        .output()
        .unwrap();

    let echo = String::from_utf8(metadata.stdout).unwrap();
    let v: Vec<&str> = echo.split('\n').collect();
    println!("{:?}",v);

    //let subtitle_grabber = process::Command::new("ffmpeg")
    //    .arg("-i")
    //    .arg(file.to_string())
    //    .arg("-map")
    //    .arg("0:s:0")
    //    .arg("/home/iggy/Documents/subs.srt");





}
