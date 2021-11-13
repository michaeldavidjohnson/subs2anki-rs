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
        .arg("-i")
        .arg(file.to_string())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Broken");

    let echo = metadata.stdout.expect("hmmm");

    println!("fucking rape me{:?}",echo);
    //let subtitle_grabber = process::Command::new("ffmpeg")
    //    .arg("-i")
    //    .arg(file.to_string())
    //    .arg("-map")
    //    .arg("0:s:0")
    //    .arg("/home/iggy/Documents/subs.srt");

    



}
