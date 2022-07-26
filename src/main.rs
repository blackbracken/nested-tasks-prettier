use std::{
    collections::HashSet,
    env,
    io::{self, BufRead},
};

use nested_tasks_prettier::{pretty, PrettyFlag};
use seahorse::{App, Context};

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .action(pretty_command)
        .run(env::args().collect());
}

fn pretty_command(_: &Context) {
    let input = read_lines();
    let flags: HashSet<PrettyFlag> = vec![].into_iter().collect();

    let prettied = pretty(input, flags);

    print!("{}", prettied.join("\n"));
}

fn read_lines() -> Vec<String> {
    let mut lines = io::stdin().lock().lines();
    let mut read = vec![];

    while let Some(Ok(line)) = lines.next() {
        read.push(line);
    }

    read.into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect()
}
