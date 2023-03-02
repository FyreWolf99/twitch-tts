use twitch_irc::
{
    login::StaticLoginCredentials,
    ClientConfig,
    SecureTCPTransport,
    TwitchIRCClient,
};
use colored::*;
use std::io::stdin;

mod message_reader;
mod ttsay;
mod message_filter;

#[tokio::main]
pub async fn main()
{
    println!("Enter name of channel to watch:");
    let mut chan = String::new();
    let _ = stdin().read_line(&mut chan).expect("Reading User Input Failed!");
    chan = chan
        .to_lowercase()
        .chars()
        .filter(|c| c != &'\n' && !c.is_whitespace())
        .collect::<String>();

    println!("Add chatters you would like to be able to speak, type ({}) to finish", "QUIT".red());
    let mut filter = message_filter::Filter::new();
    //filter.add_user("fyrewolf99");
    'selection: loop
    {
        let mut usr = String::new();
        let _ = stdin().read_line(&mut usr).expect("Reading User Input Failed!");
        if usr == "QUIT\n".to_owned()
        {
            break 'selection;
        }
        usr = usr
            .to_lowercase()
            .chars()
            .filter(|c| c != &'\n' && !c.is_whitespace())
            .collect::<String>();
        filter.add_user(&usr);
    }

    let config = ClientConfig::default();
    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);


    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move
    {
        while let Some(message) = incoming_messages.recv().await
        {
            filter.handle_message(message);
            /*
            message_reader::handle_message(&message);
            ttsay::speak_message(&message);
            */
        }
    });

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    client.join(chan).expect(&format!("{} Use the {}, not the {}!", "Streamer Doesn't Exist!".red(), "username".blue(), "display name".blue()));

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.expect("Something went wrong?");
}
