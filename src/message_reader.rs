use twitch_irc::message;
use colored::*;
use regex::Regex;

pub fn print_important_message(message: &message::PrivmsgMessage)
{
    let out = format!
    {
        "{}{}{}",
        message.sender.name.black(),
        ":".yellow(),
        colour_msg_text(&message.message_text)
    };

    println!("{}", out.on_red());
}

pub fn print_message(message: &message::PrivmsgMessage)
{
    println!("{}{}{}", message.sender.name.red(), ":".yellow(), colour_msg_text(&message.message_text));
}

fn colour_msg_text(text: &str) -> String
{
    let link = Regex::new(r"\S+\.\S+").unwrap();

    let mut out = String::new();

    for word in text.split_whitespace()
    {
        out.push(' ');
        if link.is_match(word)
        {
            out.push_str(&format!("{}", word.blue().italic()));
        }
        else
        {
            out.push_str(word);
        }
    }

    return out;
}
