mod song;
pub fn christmas_song() {
    let mut song_lyrics:String = "".to_string();
    let mut day: &str;
    let mut gift:String = "".to_string();
    let mut song_pattern:String;
    for n in 0..12{
        match n {
            0 => gift.push_str("a Partridge in a Pear Tree\n"),
            1 => gift.push_str("two Turtle Doves\n"),
            2 => gift.push_str("three French Hens\n"),
            3 => gift.push_str("four Calling Birds\n"),
            4 => gift.push_str("five Gold Rings\n"),
            5 => gift.push_str("six Geese a Laying\n"),
            6 => gift.push_str("seven Swans a Swimming\n"),
            7 => gift.push_str("eight Maids a Milking\n"),
            8 => gift.push_str("nine Ladies Dancing\n"),
            9 => gift.push_str("ten Lords a Leaping\n"),
            10 => gift.push_str("eleven Pipers Piping\n"),
            11 => gift.push_str("twelve Drummers Drumming\n"),
            _ => gift.push_str("unknown gift")
        }
        match n {
            0 => day = "first",
            1 => day = "second",
            2 => day = "third",
            3 => day = "fourth",
            4 => day = "fifth",
            5 => day = "sixth",
            6 => day = "seventh",
            7 => day = "eighth",
            8 => day = "ninth",
            9 => day = "tenth",
            10 => day = "eleventh",
            11 => day = "twelfth",
            _ => day = "unknown"
        }
        song_pattern = format!("On the {} day of Christmas my true love gave to me:\n{}\n", day, gift);
        song_lyrics.push_str(&song_pattern);
    }
    println!("{}", song_lyrics)
}