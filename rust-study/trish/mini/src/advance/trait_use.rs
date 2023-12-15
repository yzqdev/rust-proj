pub trait Playable {
    fn play(&self);

    //trait中也可以有具体的实现
    fn pause() {
        println!("Paused");
    }
}
struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}
impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }

    //重载trait中已有的方法
    fn pause() {
        println!("Video Paused");
    }
}
pub fn start_play() {
    println!("Super player!");
    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("big_buck_bunny.mkv".to_string());

    audio.play();
    video.play();

    //跟struct的关联函数一样的，用::访问
    Audio::pause();
    Video::pause()
}