#![feature(plugin)]
use regex::Regex;
use regex::Captures;
use utils::tcp::*;
use utils::chrono::*;
use std::collections::HashMap;
use std::char;
use std::fs::File;
use std::process::exit;
use utils::rand::{thread_rng, Rng};


<<<<<<< HEAD
pub struct Commands
{
    PING: Regex,
    PRIVMSG: Regex,
    HELP: Regex,
    DAYS: Regex,
    REFRAD: Regex,
    COMMIT: Regex,
    TIMELEFT: Regex,
    QUOTE: Regex,
    LINK: Regex,
}

impl Commands
{
    pub fn new() -> Commands
    {
        let PING: Regex = Regex::new(r".*(PING)\s+:(\w+)").unwrap();
        let PRIVMSG: Regex = Regex::new(r"^:(\w+)!.*(PRIVMSG)\s+(.\w+)\s+:(.+)+").unwrap();
        let HELP: Regex  = Regex::new(r".*(\.help).*").unwrap();
        let DAYS: Regex = Regex::new(r".*(\.days)+\s([^!][0-9]+)*(.*).*").unwrap();
        let REFRAD: Regex = Regex::new(r".*(\.refrad)+\s([0-9]+)*(.*).*").unwrap();
        let COMMIT: Regex = Regex::new(r".*(\.commitment)+\s([0-9]+)*(.*).*").unwrap();
        let TIMELEFT: Regex = Regex::new(r".*(\.commitment|\.refrad)+\s([^!][0-9]+)*(.*).*").unwrap();
        let QUOTE: Regex = Regex::new(r".*(\.quote)+\s*(.*).*").unwrap();
        let LINK: Regex = Regex::new(r"^(https?://).*").unwrap();
        Commands { PING: PING, PRIVMSG: PRIVMSG, HELP: HELP, DAYS: DAYS, REFRAD: REFRAD,
            COMMIT: COMMIT, TIMELEFT: TIMELEFT,  QUOTE: QUOTE, LINK: LINK}
    }
}
=======
static PING: Regex = regex!(r".*(PING)\s+:(\w+)");
static PRIVMSG: Regex = regex!(r"^:(\w+)!.*(PRIVMSG)\s+(.\w+)\s+:(.+)+");
static HELP: Regex = regex!(r".*(\.help).*");
static DAYS: Regex = regex!(r".*(\.days)+\s([^!][0-9]+)*(.*).*");
static TIMELEFT: Regex = regex!(r".*(\.commitment|\.refrad)+\s([^!][0-9]+)*(.*).*");
static QUOTE: Regex = regex!(r".*(\.quote)+\s*(.*).*");
static LINK: Regex = regex!(r"^(https?://).*");

static QUOTEFILE: &str = ""
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5

pub struct IrcHandler
{
    connection: SSLConnection,
    graddates: HashMap<&'static str, NaiveDate>,
    quotes: HashMap<u32, String>,
<<<<<<< HEAD
    commands: Commands,
=======
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
}

impl  IrcHandler
{
    pub fn new(host: &str, port: u16) -> IrcHandler
    {
        let mut cnx = IrcHandler::build(host, port);
        let mut days = IrcHandler::initDays();
        let mut quotes = IrcHandler::initQuotes();
<<<<<<< HEAD
        let mut commands = Commands::new();
        IrcHandler{connection: cnx, graddates: days, quotes: quotes, commands: commands}
=======
        IrcHandler{connection: cnx, graddates: days, quotes: quotes}
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
    }

    fn build(host: &str, port: u16) -> SSLConnection 
    {
        SSLConnection::new(host, port)
    }

    fn initDays() -> HashMap<&'static str, NaiveDate>
    {
        let mut days: HashMap<&'static str, NaiveDate> = HashMap::new();
        days.insert("2000", NaiveDate::from_ymd(2000,05,27));
        days.insert("2001", NaiveDate::from_ymd(2001,06,02));
        days.insert("2002", NaiveDate::from_ymd(2002,06,01));
        days.insert("2003", NaiveDate::from_ymd(2003,05,31));
        days.insert("2004", NaiveDate::from_ymd(2004,05,29));
        days.insert("2005", NaiveDate::from_ymd(2005,05,28));
        days.insert("2006", NaiveDate::from_ymd(2006,05,27));
        days.insert("2007", NaiveDate::from_ymd(2007,05,26));
        days.insert("2008", NaiveDate::from_ymd(2008,05,31));
        days.insert("2009", NaiveDate::from_ymd(2009,05,23));
        days.insert("2010", NaiveDate::from_ymd(2010,05,22));
        days.insert("2011", NaiveDate::from_ymd(2011,05,21));
        days.insert("2012", NaiveDate::from_ymd(2012,05,25));
        days.insert("2013", NaiveDate::from_ymd(2013,05,25));
        days.insert("2014", NaiveDate::from_ymd(2014,05,28));
        days.insert("2015", NaiveDate::from_ymd(2015,05,23));
        days.insert("2016", NaiveDate::from_ymd(2016,05,21));
        days.insert("2017", NaiveDate::from_ymd(2017,05,27));
        days.insert("2018", NaiveDate::from_ymd(2018,05,26));
        days.insert("2019", NaiveDate::from_ymd(2019,05,25));
        days
    }

    fn initQuotes() -> HashMap<u32, String>
    {
<<<<<<< HEAD
        let mut fh = File::open("/path/to/quotes/file").unwrap();
=======
        let mut fh = File::open(QUOTEFILE).unwrap();
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
        let mut reader = BufReader::new(fh);
        
        let mut lines = reader.lines();
        let mut count = 0;
        let mut quotes = HashMap::new(); 
        while let Some(line) = lines.next()
        {
            let mut splitter = line.unwrap();
            let index = splitter.split("|").nth(0).unwrap().parse::<u32>().unwrap();
            let s = splitter.split("|").nth(1).unwrap().to_string();
            quotes.insert(index, s);
        }
        return quotes;
    }

  
    pub fn send(&mut self, to_send: &str)
    {
        self.connection.send(to_send);
    }

    pub fn send_pong(&mut self, s: &str)
    {
        let PONG = "PONG :".to_string();
        let to_send = PONG + s;
        self.connection.send(&to_send);
    }

    pub fn read(&mut self) -> Option<String>
    {
        self.connection.read()
    }

    pub fn parse(&mut self, line: String)
    {
        println!("{}", line);
        match &line
        {
<<<<<<< HEAD
            m if self.commands.PING.is_match(&m) =>
            {
                let resp = self.commands.PING.captures(&m).unwrap().at(2).unwrap();
                println!("Sending PONG: {}", resp);
                self.send_pong(resp);
            },
            m if self.commands.PRIVMSG.is_match(&m) =>
            {
                let info = self.commands.PRIVMSG.captures(&m).unwrap();
=======
            m if PING.is_match(&m) =>
            {
                let resp = PING.captures(&m).unwrap().at(2).unwrap();
                println!("Sending PONG: {}", resp);
                self.send_pong(resp);
            },
            m if PRIVMSG.is_match(&m) =>
            {
                let info = PRIVMSG.captures(&m).unwrap();
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
                let username = info.at(1).unwrap();
                let mut channel = info.at(3).unwrap();
                let message = info.at(4).unwrap();
                if !('#'  == char::from_u32(channel.as_bytes()[0] as u32).unwrap())
                {
                    channel = username;
                }
                self.messageParse(channel.to_string(), username.to_string(), message.to_string());
            },
            _ => (),
        }
    }

    fn messageParse(&mut self, channel: String, username: String, message: String)
    {
        match &message
        {
<<<<<<< HEAD
            m if self.commands.HELP.is_match(&m) =>
=======
            m if HELP.is_match(&m) =>
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
            {
                println!("Sending generic help to {}", username);
                self.displayHelp(username);
            },
<<<<<<< HEAD
            m if self.commands.DAYS.is_match(&m) =>
            {
                let info = match self.commands.DAYS.captures(&m)
=======
            m if DAYS.is_match(&m) =>
            {
                let info = match DAYS.captures(&m)
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
                {
                    None => return,
                    Some(x) => Some(x)
                };
                let mut year  =  match info
                {
                    None => return,
                    Some(x) => 
                    {
                        match x.at(2)
                        {
                            None => 
                            {
                                let sendStr = format!("PRIVMSG {} :Sorry, no entry for that year.", username);
                                self.send(&sendStr);
                                return
                            },
                            Some(x) => x
                        }
                    }
                };
                let days = match self.days(year, false)
                {
                    None =>
                    {
                        let sendStr = format!("PRIVMSG {} :Sorry, no entry for that year.", username);
                        self.send(&sendStr);
                        return
                    },
                    Some(x) => x
                };
                if days > 0
                {
                    let sendStr = format!("PRIVMSG {} :There have been {} days since the class of {} graduated!", channel, days, year);
                    self.send(&sendStr);
                } else if days < 0
                {
                    let sendStr = format!("PRIVMSG {} :There are {} and a butt days until the class of {}, graduates from the Academy!", channel, days.abs() - 1, year);
                    self.send(&sendStr);
                } else
                {
                    let sendStr = format!("PRIVMSG {} : The class of {} graduates today!", channel, year);
                    self.send(&sendStr);
                }
            },
<<<<<<< HEAD
            m if self.commands.TIMELEFT.is_match(&m) =>
            {
                let info = match self.commands.TIMELEFT.captures(&m)
=======
            m if TIMELEFT.is_match(&m) =>
            {
                let info = match TIMELEFT.captures(&m)
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
                {
                    None => None,
                    Some(x) => Some(x)
                };
                let year = match info
                {
                    None =>
                    {
                        let sendStr = format!("PRIVMSG {} :Sorry, no entry for that year.", username);
                        self.send(&sendStr);
                        return
                    },
                    Some(x) =>
                    {
                        match x.at(2)
                        {
                            None => 
                            {
                                let sendStr = format!("PRIVMSG {} :Sorry, no entry for that year.", username);
                                self.send(&sendStr);
                                return
                            },
                            Some(x) => x
                        }
                    }
                };
                let days = match self.days(year, true)
                {
                    None =>
                    {
                        let sendStr = format!("PRIVMSG {} :Sorry, no entry for that year", username);
                        self.send(&sendStr);
                        return
                    },
                    Some(x) => 
                    {
                        if x == -1
                        {
                            let sendStr = format!("PRIVMSG {} :Sorry, the class of {} hasn't graduated yet. Maybe try waiting until then :-/", channel, year);
                            self.send(&sendStr);
                            return
                        } else
                        {
                            x
                        }
                    }

                };
                if days > 0
                {
                    let sendStr = format!("PRIVMSG {} :There have been {} days since the class of {}'s commitment ended!", channel, days, year);
                    self.send(&sendStr);
                } else if days < 0
                {
                    let sendStr = format!("PRIVMSG {} :There are {} and a butt days until the class of {}'s commitment ends!", channel, days.abs() -1, year);
                    self.send(&sendStr);
                } else
                {
                    let sendStr = format!("PRIVMSG {} :The class of {}'s commitment ends today!", channel, year);
                    self.send(&sendStr);
                }
            },
<<<<<<< HEAD
            m if self.commands.KOREA.is_match(&m) =>
            {
                let koreadate = NaiveDate::from_ymd(2016,05,20);
                let today = Local::today().naive_local();
                let days = (today - koreadate).num_days();
                let sendStr = format!("PRIVMSG {} :There are {} and a butt days until turnersr is free from the clutches of Kim Jon Un!", channel, days.abs() -1);
                self.send(&sendStr);
            },
            m if self.commands.QUOTE.is_match(&m) =>
=======
            m if QUOTE.is_match(&m) =>
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
            {
                let mut rng = thread_rng();
                let n: u32 = rng.gen_range(1,40);
                let sendStr = format!("PRIVMSG {} :{}", channel, self.quotes.get(&n).unwrap());
                self.send(&sendStr);
            },
<<<<<<< HEAD
            m if self.commands.LINK.is_match(&m) =>
            {
                let sendStr = format!("PRIVMSG {} :{}", username, "Please place links in #ctdt-links unless you are planning to discuss the link.");
=======
            m if LINK.is_match(&m) =>
            {
                let sendStr = format!("PRIVMSG {} :{}", username, "Please place links in appropriate channel unless you are planning to discuss the link.");
>>>>>>> 48aa855df1aeb319d865cef7f05d988c62f82de5
                self.send(&sendStr);
            },
            _ => ()
        }
    }

    fn displayHelp(&mut self, username: String)
    {
        let helpStrings = vec![
            ".help                  : Displays this help message",
            ".help <command>        : Displays the detailed help for <command>",
            ".quote                 : Get a quote from the sages",
            ".days <gradYear>       : Get the days until/since <gradYear> graduate(s/d)",
            ".quoteadd <quote>      : Add a quote to the database. Quote must be between < and >",
            ".commitment <gradYear> : Get the days until/since <gradYear>'s commitment end(s/ed)",
            ".refrad                : Aliases to .commitment"];
        for helpString in helpStrings
        {
            let sendStr = format!("PRIVMSG {} :{}", username, helpString);
            self.send(&sendStr);
        }
    }

    fn days(&mut self, year: &str, refrad: bool) -> Option<i64>
    {
        let gradDate = match self.graddates.get(year)
        {
                None => return None,
                Some(x) => *x,
        };
        let today = Local::today().naive_local();
        if !refrad
        {
            let days = (today - gradDate).num_days();
            Some(days)
        } else
        {
            if (today-gradDate).num_days() < 0
            {
                return Some(-1);
            }
            let refradDate = gradDate.with_year(gradDate.year()+5).unwrap();
            let days = (today - refradDate).num_days();
            Some(days)
        }
   }
}
        
