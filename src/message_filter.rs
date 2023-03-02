use twitch_irc::message;

use crate::{message_reader, ttsay};

pub struct Filter
{
    speakers: Vec<String>
}

impl Filter
{
    pub fn new() -> Filter
    {
        Filter { speakers: Vec::new() }
    }

    pub fn add_user(&mut self, username: &str)
    {
        self.speakers.push(username.to_owned());
    }

    pub fn handle_message(&self, message: message::ServerMessage)
    {
        let msg = match message
        {
            message::ServerMessage::Privmsg(msg) => msg,
            _ => return
        };

        if self.speakers.contains(&msg.sender.login.to_owned())
        {
            message_reader::print_important_message(&msg);
            ttsay::speak_message(&msg);
        }
        else
        {
            message_reader::print_message(&msg);
        }
    }
}
