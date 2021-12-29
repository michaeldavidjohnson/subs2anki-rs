use std::env;
use std::process;
use std::fs;
use regex::Regex;

const ACCEPTED_SUBTITLE_EXTENSIONS: [&str ;1] = ["srt"];
const ACCEPTED_VIDEO_EXTENSIONS: [&str ;3] = ["mkv","mp4","avi"];
const TEMP_FILEPATH: &str = "/home/iggy/Documents/Rust/subs2srsclone/Temp/";

fn main() {
    let _args: Vec<String> = env::args().collect();

    //Currently, there is no user input. So there are two file handlings I want to support. Firstly, subtitle splitting and forming
    //Secondly, subtitle extraction. A UI should be made that passes an arbritary amount of video files as well as couple extra subtitle files
    //This will have to be a specific format into _args which allows for the code to generalise. For now, I don't have this, Therefore the blocks
    //are commented out.


    //Logic for separating inputs into subtitle files and video files, this will become the backbone of being able to call the other commands.
    //We now just need a way to zip the two vecs together to performe subtitle splitting, screenshot generation, etc.
    let filepaths = &_args[1..];
    let mut subtitles = Vec::<&String>::new();
    let mut videos = Vec::<&String>::new();
    let mut extractor_toggle: bool = false;
    for file in filepaths{
        if file == "__extract" || file == "_e"{
            extractor_toggle = true;
        }

        for sub in ACCEPTED_SUBTITLE_EXTENSIONS{
            let last_three: Vec<char> = file.chars().rev().take(3).collect();
            let last_three_subtitle: Vec<char> = sub.chars().rev().take(3).collect();
            let subtitle_condition = last_three_subtitle.into_iter().rev().collect::<String>() == last_three.into_iter().rev().collect::<String>();
            if subtitle_condition{
                subtitles.push(file);

            }
        }
        for viddy in ACCEPTED_VIDEO_EXTENSIONS{
            let last_three: Vec<char> = file.chars().rev().take(3).collect();
            let last_three_video: Vec<char> = viddy.chars().rev().take(3).collect();
            let video_condition = last_three_video.into_iter().rev().collect::<String>() == last_three.into_iter().rev().collect::<String>();
            if video_condition{
                videos.push(file)
            }
        }
    }

    if &subtitles.len()== &videos.len(){
        populate_information_and_screenshots(&subtitles, &videos);
    }

    if subtitles.len() == 0 as usize && extractor_toggle == true{
        subtitle_extraction(&videos);
    }


}


fn populate_information_and_screenshots(subtitle_files: &Vec<&String>,video_files: &Vec<&String>) {
    for i in 0..subtitle_files.len(){
        let (start_times, end_times, text) = read_subtitles_and_split(subtitle_files[i]);
        let formatted_text = format_text_to_remove_html(text);
        ffmpeg_generate_screenshots(&start_times,&end_times,&i,&video_files[i]);
    }
}

//Needs user input to select subtitles
fn subtitle_extraction(video_files: &Vec<&String>){
    let mut i = 0;
    for file in video_files {
        let subtitle_sources = ffprobe_get_subtitle_sources(file);
        ffmpeg_subtitle_extractor(subtitle_sources.clone(),file,&i);
        create_subtitle_metadata(subtitle_sources.clone());
        i = i+1;
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
    let mut v: Vec<String> = subtitle_string.split("\n").map(|s| s.to_string()).collect();
    v.truncate(v.len()-1);
    return v

}

fn ffmpeg_subtitle_extractor(source_list:Vec<String>,file:&String,file_number:&i32){
    fs::create_dir(format!("{}{}",TEMP_FILEPATH,file_number)).unwrap();
    for source in source_list{
        let t:Vec<&str> = source.split(',').collect(); 
        let stream = t[0];
        let name = t[1];
        let stream_info = format!("0:{}",stream);
        let filename = format!("{}{}/{}.srt",TEMP_FILEPATH,file_number,name);
        process::Command::new("ffmpeg")
            .arg("-hide_banner")
            .arg("-loglevel")
            .arg("error")
            .arg("-n")
            .arg("-i")
            .arg(file.to_string())
            .arg("-map")
            .arg(stream_info)
            .arg(filename)
            .spawn().unwrap().wait().unwrap();
    }

}

fn ffmpeg_generate_screenshots(start_time:&Vec<String>,end_time:&Vec<String>,file_number:&usize,video_file:&str){ 
    let temp_video = video_file;

    let start_times = start_time;
    let end_times = end_time;
    let mut index = 0;
    for i in start_times.iter(){
        let out_path = format!{"{}{}.jpg",TEMP_FILEPATH,index};
        let mut removed = i.to_string().clone();
        let mut sub_index = 0;
        let mut sub_string:Vec<char> = Vec::new();
        for c in removed.chars(){
            if sub_index == 8{
                break;
            }
            let value = c.clone();
            sub_string.push(value);
            sub_index = sub_index + 1;
        }
        let initial_time:String = sub_string.into_iter().collect();

        // Look into passing all times in one command, to improve performance
        process::Command::new("ffmpeg")
            .arg("-ss")
            .arg(initial_time)
            .arg("-i")
            .arg(temp_video)
            .arg("-vframes")
            .arg("1")
            .arg(out_path)
            .spawn()
            .unwrap().wait().unwrap();
        index = index + 1;
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
//Subtitle parsing
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

fn format_text_to_remove_html(text:Vec<String>)->Vec<String>{
    let re = Regex::new(r"<[^>]*>").unwrap();
    let text_to_change: Vec<&str> = text.iter().map(|s| &**s).collect();
    let mut result:Vec<String> = Vec::new();
    for line in text_to_change{
        result.push(re.replace_all(&line,"").to_string());
    }

    return result;
}
