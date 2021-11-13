use std::env;
use std::process;
fn main() {
    let _args: Vec<String> = env::args().collect();
    let filepaths = &_args[1..];
    for file in filepaths{
        let subtitles = ffprobe_get_subtitle_sources(file);
        ffmpeg_subtitle_extractor(subtitles,file);
    }
}

fn ffprobe_get_subtitle_sources(file:&String) -> Vec<String>{
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

    let subtitle_string: String = String::from_utf8(metadata.stdout).unwrap();
    let v: Vec<String> = subtitle_string.split("\n").map(|s| s.to_string()).collect();
    return v

}

fn ffmpeg_subtitle_extractor(source_list:Vec<String>,file:&String){
    for source in source_list{
        let t:Vec<&str> = source.split(',').collect(); 
        let stream = t[0];
        let name = t[1];
        let stream_info = format!("0:{}",stream);
        let filename = format!("/home/iggy/Documents/{}.srt",name);
        process::Command::new("ffmpeg")
            .arg("-i")
            .arg(file.to_string())
            .arg("-map")
            .arg(stream_info)
            .arg(filename)
            .spawn().unwrap();
    }

}
