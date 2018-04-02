use std::thread::sleep;
use std::io::BufReader;
use std::io::Read;
use std::io::BufRead;
use std::io;
use std::time::Duration;

pub trait LogLineGenerator<R: Read>: Iterator<Item = io::Result<String>> {
    fn from_reader(r: R) -> Self;
}

pub struct DefaultLogLineGenerator<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> LogLineGenerator<R> for DefaultLogLineGenerator<R> {
    fn from_reader(r: R) -> Self {
        Self { reader: BufReader::new(r) }
    }
}

impl<R: Read> Iterator for DefaultLogLineGenerator<R> {
    type Item = io::Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.try_get_line() {
                Ok(ref s) if s.len() == 0 => {
                    trace!("Sleeping!");
                    sleep(Duration::from_millis(500));
                    continue;
                }
                item @ _ => {
                    trace!("Returning item: {:?}", item);
                    return Some(item);
                }
            }
        }
    }
}

impl<R: Read> DefaultLogLineGenerator<R> {
    fn try_get_line(&mut self) -> io::Result<String> {
        let mut item = String::new();
        self.reader.read_line(&mut item)?;
        Ok(item)
    }
}
