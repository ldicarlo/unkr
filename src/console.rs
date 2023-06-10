use crossterm::{
    cursor::MoveTo,
    style::Print,
    terminal::{self, Clear},
    ExecutableCommand,
};
use std::{thread, time};
// https://docs.rs/crossterm/latest/crossterm/
use std::io::{stdout, Write};
use std::sync::mpsc::Receiver;

pub enum PrintableMessage {
    ThreadStatus(ThreadStatusPayload),
    Default(String),
}

pub struct ThreadStatusPayload {
    pub thread_number: usize,
    pub step: usize,
    pub total: usize,
    pub current_combination: String,
}

pub fn thread_consume_messages(r: Receiver<PrintableMessage>, thread_count: usize) {
    let mut stdout = stdout();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    r.iter().for_each(|x| print(x, thread_count));

    stdout.flush().unwrap();
}

fn print(pm: PrintableMessage, thread_count: usize) {
    match pm {
        PrintableMessage::ThreadStatus(p) => print_thread_status(p),
        PrintableMessage::Default(str) => print_default(str, thread_count),
    }
}

fn print_default(str: String, thread_count: usize) {
    stdout()
        .execute(MoveTo(0, thread_count as u16 + 1))
        .unwrap()
        .execute(Clear(terminal::ClearType::CurrentLine))
        .unwrap()
        .execute(Print(str))
        .unwrap()
        .flush()
        .unwrap();
    let ten_millis = time::Duration::from_millis(1000);
    thread::sleep(ten_millis);
}

fn print_thread_status(
    ThreadStatusPayload {
        thread_number,
        step,
        total,
        current_combination,
    }: ThreadStatusPayload,
) {
    stdout()
        .execute(MoveTo(0, thread_number as u16))
        .unwrap()
        .execute(Clear(terminal::ClearType::CurrentLine))
        .unwrap()
        .execute(Print(format!(
            "thread_{:02}: {:03}/{:03} ({})",
            thread_number, step, total, current_combination
        )))
        .unwrap()
        .flush()
        .unwrap();
    let ten_millis = time::Duration::from_millis(1000);
    thread::sleep(ten_millis);
}
