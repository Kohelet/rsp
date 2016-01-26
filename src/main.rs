#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;
use regex::Regex;
use std::thread;
use std::io::{BufRead, Read, Write};
mod utils;

fn main()
{
    let host = "irc.cyclonecobra.com";
    let port: u16 = 6697;
    let ping = Regex::new(r".*(PING)\s+:(\w+)").unwrap();
    let mut handle = utils::irchandler::IrcHandler::new(host,port);
    {
        handle.send("NICK RainbowSandPony");
        handle.send("USER kohelet * * :kohelet");
        thread::sleep_ms(2000);
        handle.send("JOIN #ctdt");
        handle.send("JOIN #ctdt-links");
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

