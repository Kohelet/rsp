#![feature(plugin)]
<<<<<<< HEAD
=======
#![plugin(regex_macros)]
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
extern crate regex;
use regex::Regex;
use std::thread;
use std::io::{BufRead, Read, Write};
mod utils;

fn main()
{
    let host = "";
    let port: u16 = 6697;
    let ping = Regex::new(r".*(PING)\s+:(\w+)").unwrap();
    let mut handle = utils::irchandler::IrcHandler::new(host,port);
    {
        handle.send("NICK RainbowSandPony");
        handle.send("USER kohelet * * :kohelet");
        thread::sleep_ms(2000);
<<<<<<< HEAD
        handle.send("JOIN #ctdt");
        handle.send("JOIN #ctdt-links");
=======
        handle.send("JOIN <channel>");
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
    }
    loop
    {
        let line = match handle.read()
        {
            Some(x) => x,
            None => continue,
        };
        handle.parse(line);
    }
}

