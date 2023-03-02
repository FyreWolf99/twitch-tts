use twitch_irc::
{
    login::StaticLoginCredentials,
    ClientConfig,
    SecureTCPTransport,
    TwitchIRCClient,
    message
};
use colored::*;
use regex::Regex;

#[tokio::main]
pub async fn main()
{
    let config = ClientConfig::default();
    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move
    {
        while let Some(message) = incoming_messages.recv().await
        {
            handle_message(message);
        }
    });

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    client.join("xqc".to_owned()).expect("Streamer doesn't exist!\nUse the username, not the display name!");

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.expect("Something went wrong?");
}

fn handle_message(message: message::ServerMessage)
{
    match message
    {
        message::ServerMessage::Privmsg(msg) => println!("{}{}{}", msg.sender.name.red(), ":".yellow(), colour_msg_text(&msg.message_text)),
        _ => {}
    }
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
