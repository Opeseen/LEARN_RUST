fn main() {
    let message = ChatMessage {
        content: "Hello",
        time: String::from("2025-03-12"),
    };
    println!("{:?}", message.retrieve_time());

    let notification = ChatMessage {
        content: String::from("What's your favorite pizza topping"),
        time: String::from("2026-03-13"),
    };
    println!("{:?}", notification.retrieve_time());

    let audio = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("2027-03-13"),
    };
    audio.consume_entertainment();
    println!("{:?}", audio.retrieve_time());
}

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}
