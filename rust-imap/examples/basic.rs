extern crate imap;

/// 基础案例，直接使用用户名密码方式认证

fn main() {
    fetch_inbox_top().unwrap();
}

fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let client = imap::ClientBuilder::new("imap.qq.com", 993).connect()?;

    // 默认使用 PLAIN 认证方式
    let mut session = client
        .login("1143967454@qq.com", "zgdqonjnfpergjcd")
        .map_err(|e| e.0)?;

    session.select("INBOX")?;

    let messages = session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // be nice to the server and log out
    session.logout()?;
    println!("{}", body);
    Ok(Some(body))
}