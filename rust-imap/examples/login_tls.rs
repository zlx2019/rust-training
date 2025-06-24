use std::{error::Error, net::TcpStream, sync::Arc};
use anyhow::Result;
use base64::Engine;
use rustls::{pki_types::ServerName, ClientConnection, Stream};


/// IMAP LOGIN 认证实现
struct LoginAuth {
    username: String,
    password: String,
}

impl LoginAuth {
    fn new(username: String, password: String) -> LoginAuth {
        LoginAuth { username, password }
    }
}

impl imap::Authenticator for LoginAuth {
    type Response = String;

    fn process(&self, _: &[u8]) -> Self::Response {
        let username_encoded = base64::engine::general_purpose::STANDARD.encode(&self.username);
        let password_encoded = base64::engine::general_purpose::STANDARD.encode(&self.password);
        format!("{}\x01{}", username_encoded, password_encoded)
    }
}


/// 使用 rustls + imap 访问邮件
fn main() -> Result<(), Box<dyn Error>> {
    let host = "imap.zmailservice.com".to_string();
    let user = "GerrardRevells00730@hotmail.com".to_string();
    let password = "WvreUk7DBhi".to_string();
    let port = 993;
    let email_result = fetch_inbox_top(host, user, password, port);
    match email_result {
        Ok(Some(_)) => {
            println!("读取邮件成功");
        },
        Ok(None) => {
            println!("未可读邮件");
        }
        Err(e) => {
            println!("读取邮件错误：{}", e);
        },
    }
    Ok(())
}

fn fetch_inbox_top(host: String, user: String, password: String, port: u16,
) -> Result<Option<String>> {
    // 与邮箱服务器建立TCP连接
    let mut tcp_conn = TcpStream::connect(format!("{}:{}", host, port)).unwrap();
    // 进行 TLS 握手
    let mut tls_conn = build_tls_conn(&host)?;
    let conn = Stream::new(&mut tls_conn, &mut tcp_conn);
    // 使用IMAP协议进行通信
    let mut client = imap::Client::new(conn);
    // 是否开启DEBUG 模式
    client.debug = true;

    // 使用 用户名密码方式登录
    let auth = LoginAuth::new(user.clone(), password.clone());
    // let mut session = client.authenticate("LOGIN", &auth).map_err(|e| e.0)?;
    let mut session = client.login(&user, &password).map_err(|e| e.0)?;
    // 打开收件箱
    session.select("INBOX")?;
    // 获取ID为1的邮件
    let messages = session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    println!("获取到的邮件数量: {}", messages.len());
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // 退出
    session.logout()?;
    Ok(Some(body))
}


/// 与邮箱服务器创建TLS连接
fn build_tls_conn(server_host: &str) -> Result<ClientConnection> {
    let store = rustls::RootCertStore::from_iter(
        webpki_roots::TLS_SERVER_ROOTS.iter().cloned()
    );
    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(store)
        .with_no_client_auth();
    Ok(rustls::ClientConnection::new(
        Arc::new(tls_config),
        ServerName::try_from(server_host.to_string())?)?)
}
