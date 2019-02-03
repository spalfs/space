extern crate serde_json;
extern crate termion;

use self::termion::async_stdin;
use self::termion::raw::IntoRawMode;
use std::io::{stdout, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::mass::Mass;

pub fn client_dashboard(mut buff_r: BufReader<TcpStream>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    loop {
        let mut recv = String::new();
        buff_r.read_line(&mut recv).unwrap();
        let ship: Mass = serde_json::from_str(&recv).unwrap();

        write!(
            stdout,
            "{}{}{:?}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            ship
        )
        .unwrap();

        if let Some(c) = stdin.next() {
            let c = c.unwrap() as char;
            if c == 'q' {
                break;
            }
        }
        stdout.flush().unwrap();
    }
}
