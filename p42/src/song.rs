use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;

// Implement the SongIter type which implements the Iterator trait
pub struct SongIter {
    current_day: usize,
    max_days: usize,
}

impl SongIter {
    // Constructor for creating a new SongIter
    pub fn new(max_days: usize) -> Self {
        SongIter {
            current_day: 0,
            max_days,
        }
    }
}

// Implement the Iterator trait for SongIter
impl Iterator for SongIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // The lines of "The Twelve Days of Christmas" song
        const GIFTS: [&str; 12] = [
            "A partridge in a pear tree.",
            "Two turtle doves,",
            "Three French hens,",
            "Four calling birds,",
            "Five golden rings,",
            "Six geese a-laying,",
            "Seven swans a-swimming,",
            "Eight maids a-milking,",
            "Nine ladies dancing,",
            "Ten lords a-leaping,",
            "Eleven pipers piping,",
            "Twelve drummers drumming,",
        ];

        const ORDINALS: [&str; 12] = [
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth", "eleventh", "twelfth",
        ];

        // Check if we've reached the end
        if self.current_day >= self.max_days {
            return None;
        }

        // Get the current day (1-indexed for the song)
        let day = self.current_day + 1;

        // Increment the day for the next call
        self.current_day += 1;

        // Format the start of the verse
        let mut verse = format!(
            "On the {} day of Christmas, my true love gave to me:",
            ORDINALS[day - 1]
        );

        // Special case for "and a partridge" in verses after the first day
        if day == 1 {
            verse.push('\n');
            verse.push_str(GIFTS[0]);
        } else {
            // Add all gifts for the current day
            for i in (1..=day).rev() {
                verse.push('\n');

                // Special case for the last line with "and"
                if i == 1 {
                    verse.push_str("And a partridge in a pear tree.");
                } else {
                    verse.push_str(GIFTS[i - 1]);
                }
            }
        }

        Some(verse)
    }
}

// Function to create a SongIter with numbered lines
pub fn numbered_song_iter() -> impl Iterator<Item = String> {
    SongIter::new(12)
        .enumerate()
        .map(|(i, line)| format!("{:02}: {}", i + 1, line))
}

// Generic iterator wrapper that duplicates values
pub struct DuplicatingIter<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    inner: I,
    n: usize,
    current_item: Option<T>,
    current_count: usize,
}

impl<I, T> DuplicatingIter<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    pub fn new(iter: I, n: usize) -> Self {
        DuplicatingIter {
            inner: iter,
            n,
            current_item: None,
            current_count: 0,
        }
    }
}

impl<I, T> Iterator for DuplicatingIter<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // If we're still duplicating the current item
        if let Some(item) = &self.current_item {
            if self.current_count < self.n {
                self.current_count += 1;
                return Some(item.clone());
            }
        }

        // Get the next item from the inner iterator
        match self.inner.next() {
            Some(item) => {
                self.current_item = Some(item.clone());
                self.current_count = 1;
                Some(item)
            }
            None => None,
        }
    }
}

// Function to create a duplicating iterator
pub fn duplicate<I, T>(iter: I, n: usize) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    DuplicatingIter::new(iter, n)
}

// Functions that accept Iterator of Strings

/// Concatenates the iterator items into one String and returns it
pub fn song_to_string<I>(iter: I) -> String
where
    I: IntoIterator<Item = String>,
{
    iter.into_iter().collect::<Vec<String>>().join("\n\n")
}

/// Accepts iterator and path, creates file with the path, and writes the iterator
/// strings into it
pub fn song_to_file<I, P>(iter: I, path: P) -> io::Result<()>
where
    I: IntoIterator<Item = String>,
    P: AsRef<Path>,
{
    let mut file = File::create(path)?;
    for line in iter {
        writeln!(file, "{}\n", line)?;
    }
    Ok(())
}

/// Accepts iterator and address, opens TCP connection to the provided
/// address and writes the iterator strings into it
pub fn song_to_tcp<I>(iter: I, addr: SocketAddr) -> io::Result<()>
where
    I: IntoIterator<Item = String>,
{
    let mut stream = TcpStream::connect(addr)?;
    for line in iter {
        writeln!(stream, "{}\n", line)?;
    }
    Ok(())
}

/// Listens for incoming connections on the given port (and 0.0.0.0 IP address),
/// locks stdout, and prints into it lines received from the first incoming TCP
/// connection
pub fn song_from_tcp(port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", port))?;
    println!("Listening on 0.0.0.0:{}", port);

    // Accept the first connection
    let (stream, addr) = listener.accept()?;
    println!("Connection established with: {}", addr);

    let reader = BufReader::new(stream);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Read and print lines from the connection
    for line in reader.lines() {
        match line {
            Ok(line) => {
                writeln!(handle, "{}", line)?;
            }
            Err(e) => {
                writeln!(handle, "Error reading from connection: {}", e)?;
                break;
            }
        }
    }

    Ok(())
}

// Tests for the song module functionality
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_song_iterator() {
        let mut iter = SongIter::new(3); // Just the first 3 days for brevity

        // First day
        let day1 = iter.next().unwrap();
        assert!(day1.contains("first day of Christmas"));
        assert!(day1.contains("A partridge in a pear tree"));

        // Second day
        let day2 = iter.next().unwrap();
        assert!(day2.contains("second day of Christmas"));
        assert!(day2.contains("Two turtle doves"));
        assert!(day2.contains("And a partridge in a pear tree"));

        // Third day
        let day3 = iter.next().unwrap();
        assert!(day3.contains("third day of Christmas"));
        assert!(day3.contains("Three French hens"));
        assert!(day3.contains("Two turtle doves"));
        assert!(day3.contains("And a partridge in a pear tree"));

        // Should be None after we've gone through all days
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_duplicate_iterator() {
        let iter = SongIter::new(1); // Just first day
        let mut dup_iter = duplicate(iter, 2); // Duplicate each line twice

        let first = dup_iter.next().unwrap();
        let second = dup_iter.next().unwrap();

        assert_eq!(first, second); // Both should be identical
        assert_eq!(dup_iter.next(), None); // No more items
    }

    #[test]
    fn test_song_to_string() {
        let iter = SongIter::new(2); // First two days
        let song_string = song_to_string(iter);

        assert!(song_string.contains("first day of Christmas"));
        assert!(song_string.contains("second day of Christmas"));
        assert!(song_string.contains("Two turtle doves"));
    }

    #[test]
    fn test_song_to_file() {
        let test_file = "test_song.txt";
        let iter = SongIter::new(1); // Just first day

        // Write to file
        let result = song_to_file(iter, test_file);
        assert!(result.is_ok());

        // Check file exists and content
        assert!(Path::new(test_file).exists());
        let content = fs::read_to_string(test_file).unwrap();
        assert!(content.contains("first day of Christmas"));

        // Clean up
        let _ = fs::remove_file(test_file);
    }
}
