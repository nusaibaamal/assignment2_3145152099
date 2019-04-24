pub fn reply(message: &str) -> &str {
    //unimplemented!("have Bob reply to the incoming message: {}", message)
    let messages = message.trim();
    let yelling = (messages == messages.to_uppercase()) && (messages != messages.to_lowercase());
    let question = messages.ends_with('?');
    let whitespace = messages.ends_with(' ');

    match messages {
        "" => "Fine. Be that way!",
        _ if question && yelling => "Calm down, I know what I'm doing!",
        _ if question && whitespace => "Sure.",
        _ if question => "Sure.",
        _ if yelling => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
