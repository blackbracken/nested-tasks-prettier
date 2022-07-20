use std::{io::{self, BufRead}, string};

enum HeadSpaceCount {
    UntilSpaces(u8),
    ReachedText(u8),
}

impl HeadSpaceCount {
    fn count(&self) -> u8 {
        match self {
            HeadSpaceCount::UntilSpaces(n) => *n,
            HeadSpaceCount::ReachedText(n) => *n,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        let spaces = line.chars()
            .fold(
                HeadSpaceCount::UntilSpaces(0),
                |c: HeadSpaceCount, ch| match c {
                    HeadSpaceCount::UntilSpaces(n) if ch.is_ascii_whitespace() => {
                        HeadSpaceCount::UntilSpaces(n + 1)
                    }
                    HeadSpaceCount::UntilSpaces(n) => HeadSpaceCount::ReachedText(n),
                    _ => c,
                },
            );
        
        let content: String = line.chars().skip(spaces.count().into()).collect();

        println!("{}: {}", spaces.count(), content)
    }
}
