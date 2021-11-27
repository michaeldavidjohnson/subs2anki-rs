use std::env;
use std::process;
use std::fs;
use regex::Regex;

fn main() {
    let _args: Vec<String> = env::args().collect();

    //Currently, there is no user input. So there are two file handlings I want to support. Firstly, subtitle splitting and forming
    //Secondly, subtitle extraction. A UI should be made that passes an arbritary amount of video files as well as couple extra subtitle files
    //This will have to be a specific format into _args which allows for the code to generalise. For now, I don't have this, Therefore the blocks
    //are commented out.
    let filepaths = &_args[1..];
    //Subtitle splitting, again we really need user input here to decide which functions to use instead of commenting out bad functions.
    for file in filepaths{
        let (start_times, end_times, text) = read_subtitles_and_split(file);
        for val in start_times{
            println!("{}",val);
        }

    }
    //Subtitle extraction, doesn't really have any handling of if you already have the files or not. Really missing some form of
    //consistent user input.
    //for file in filepaths{
    //    let subtitles = ffprobe_get_subtitle_sources(file);
    //    ffmpeg_subtitle_extractor(subtitles.clone(),file);
    //    create_subtitle_metadata(subtitles.clone());
    //}
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
    let mut v: Vec<String> = subtitle_string.split("\n").map(|s| s.to_string()).collect();
    v.truncate(v.len()-1);
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
            .arg("-hide_banner")
            .arg("-loglevel")
            .arg("error")
            .arg("-i")
            .arg(file.to_string())
            .arg("-map")
            .arg(stream_info)
            .arg(filename)
            .spawn().unwrap();
    }

}

fn create_subtitle_metadata(source_list:Vec<String>){
    let mut subtitle_paths: Vec<String> = Vec::<String>::new();
    for source in source_list{
        let t:Vec<&str> = source.split(",").collect();
        let mut subtitle_filepath = vec![format!("/home/iggy/Documents/{}.srt",t[1])];
        subtitle_paths.append(&mut subtitle_filepath)
    }
    println!("{:?}",subtitle_paths)

}

fn read_subtitles_and_split(file:&String)->(Vec<String>,Vec<String>,Vec<String>){
    //Should split the subtitle file properly, although there still are html formatting present in text.
    //Groups are order, start, end, text.
    let re = Regex::new(r"(?P<order>\d+)\n(?P<Start>[\d:,]+)\s+-{2}>\s+(?P<end>[\d:,]+)\n(?P<text>[\s\S].*)").unwrap();
    let contents = fs::read_to_string(file).expect("Failed to read filename");
    let mut start:Vec<String> = Vec::new();
    let mut end:Vec<String> = Vec::new();
    let mut text:Vec<String> = Vec::new();
    for cap in re.captures_iter(&contents){

        start.push(cap[2].to_string());
        end.push(cap[3].to_string());
        text.push(cap[4].to_string());
    }
    return (start, end, text)

    //println!("{}",contents);

}
