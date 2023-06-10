use crossterm::{
    cursor::{self, MoveToRow},
    execute,
    style::{self, Print, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
// https://docs.rs/crossterm/latest/crossterm/
use std::io::{stdout, Write};
use std::sync::mpsc::Receiver;
pub fn consume_message() {
    let mut stdout = stdout();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    print_message(1, 4, 10);
    print_message(2, 4, 10);
    print_message(1, 5, 10);
    print_message(2, 6, 10);
    print_message(2, 7, 10);

    stdout.flush().unwrap();
}

fn print_message(thread_number: u8, step: u8, total: u8) {
    stdout()
        .execute(MoveToRow(thread_number as u16))
        .unwrap()
        .execute(Print(step))
        .unwrap()
        .flush()
        .unwrap();
}

fn state(r: Receiver<(u8, u8, u8)>, init: u8) {
    let mut current_state: Vec<(u8, u8)> = Vec::new();
    for i in 0..init {
        current_state.push((0, 0));
    }
    r.iter().map(|(thread_number, step, total)| {
        // implem
    });
}
