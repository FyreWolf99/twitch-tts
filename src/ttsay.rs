use twitch_irc::message;
use tts::*;

pub fn speak_message(message: &message::PrivmsgMessage)
{
    let _ = speak_str(&message.message_text);
}

fn speak_str(inp: &str) -> Result<(), Error>
{
    let tts = Tts::default();
    tts?.speak(inp, true)?;
    Ok(())
}
