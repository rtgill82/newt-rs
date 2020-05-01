//
// Copyright (C) 2019 Robert Gill <locke@sdf.org>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

extern crate tempfile;

use std::env;
use std::io;
use std::io::{BufRead,BufReader,Write};
use std::os::unix::io::AsRawFd;
use std::os::unix::net::{UnixListener,UnixStream};
use std::process::exit;
use tempfile::tempdir;

use newt::widgets::form::FDFlags;
use newt::prelude::*;

fn help() {
    println!("USAGE: form_exit_fd [-client SOCKET]\n");
    println!("Run 'form_exit_fd' without command line parameters to start the server.");
    println!("A socket name will be provided after startup.");
    println!("Then run 'form_exit_fd -client /path/to/socket' to connect to the server.");
    exit(1);
}

fn client(socket_path: &str) {
    let mut stream = UnixStream::connect(socket_path).unwrap();
    println!("Enter text (Ctrl-C to exit):\n");

    let stdin = io::stdin();
    loop {
        let mut msg = String::new();
        stdin.read_line(&mut msg).unwrap();
        stream.write_all(msg.as_bytes()).unwrap();
    }
}

fn server() {
    let dir = tempdir().unwrap();
    let socket = dir.path().join("newtfd");
    let listener = UnixListener::bind(&socket).unwrap();
    let mut text = String::new();

    println!("Connect to {}", socket.to_str().unwrap());
    let (stream, _) = listener.accept().unwrap();

    newt::init().unwrap();
    newt::cls();
    newt::centered_window(50, 20, Some("FD Exit Test")).unwrap();

    let t = Textbox::new(1, 3, 48, 5, 0);
    let b = CompactButton::new(22, 19, "Exit");

    let mut form = Form::new(None, 0);
    form.add_components(&[&t, &b]).unwrap();
    form.watch_fd(stream.as_raw_fd(), FDFlags::Read);

    loop {
        let r = form.run().unwrap();
        match r {
            ExitReason::Component(_co) => break,
            ExitReason::FDReady(fd) => {
                if fd == stream.as_raw_fd() {
                    let mut buf = BufReader::new(&stream);
                    let mut msg = String::new();
                    if buf.read_line(&mut msg).unwrap() == 0 { break; }
                    text.push_str(&msg);
                    t.set_text(&text);
                }
            },
            _ => ()
        }
    }

    newt::finished();
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => server(),
        2 => help(),
        3 => {
            if args[1] != "-client" { help() };
            client(&args[2])
        },
        _ => help()
    }
}
