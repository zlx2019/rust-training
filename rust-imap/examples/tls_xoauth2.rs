#![allow(dead_code)]
use std::{net::TcpStream, sync::Arc};

use anyhow::Result;
use rustls::{pki_types::ServerName, ClientConnection, Stream};

/// 
/// 访问 Outlook 邮件Demo，使用新式验证
/// 


/// Outlook 新式验证
struct OutlookXOauth2Auth {
    username: String,
    access_token: String
}
impl OutlookXOauth2Auth{
    fn new(username: String, access_token: String) -> Self{
        OutlookXOauth2Auth{
            username, access_token
        }
    }
}

/// Outlook XOauth2 令牌认证格式
/// AUTHENTICATE XOAUTH2 <base64 string in XOAUTH2 format>
impl imap::Authenticator for OutlookXOauth2Auth{
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, challenge: &[u8]) -> Self::Response {
        format!("user={}\x01auth=Bearer {}\x01\x01", self.username, self.access_token)
    }
}

const SERVER_HOST: &str = "outlook.office365.com";
const SERVER_PORT: u16 = 993;

fn main() {
    // 构建 Outlook 认证方式
    let username: String = "warnochp7192@outlook.com".to_string();
    let access_token = "EwBYA+l3BAAUcDnR9grBJokeAHaUV8R3+rVHX+IAAS7Rf+7kjt0cCx3rKeVOaMeyCQWmhzh12We3CzTy+JSvQ6NH3W46SKHSFtV5k1bAjADZ61s+WKtlA3nwfySVYUSQZSxXsKH0cP4pOAOyyss2Bd+J/DWZN1bhtxrEipD0HkrSF/khqavswyaez7fAV5BA9A8p5AUMHIn7N49hWeQ9e3jnMajGovUgN6xxz+t+T+3pfcHegRfJvjkgZSG8caaZgbq5xbAjBIoX8JV0DIe4l0ak6fLzS0LjeF7lp8yFrzQC2MM64cdUs93sLUuwJkh3A9E5Q+Jfpt4cvXq3KExn3IMDD0a41dd83fGSfOx7qifSbDlNCpkiHiUBDaPusF0QZgAAEExV6G1aSBuyqJyL5BSETQ4gAje2uOtiHnwdQLb6T747XrzCFtmA7fdS1HXeiB3oROv/1HLO9PydH9JCJyWfTKjLolqqOL2iEY5G8ROW1EF5CkZiipSK9pNIQqd3dWnZNMJNT5gOgnaOU5Lvaz/VOdViC67G+mdBJ+IpJUojf9EYTCEDD2a9YdFm6Pae8Hg7lUvzwurCZKJuE7SJ0OIY4eoPGtyWHZXpD9Sc1Q0LGZdYz5mgdVVVKX1+0k47PTRh1/DXEJIirL7qtGmKGVF1yKLFcClH3N0BG57NAhu1dR19V8WNpxPm8HkqvDLXFGHt/rZgo14e+oTgXvERFW0wGLyQCVcyB2ajYUrz3SD+GMPDWELUp8OuQKKHsRSDxIkiyZf2CMcF7lYQBurdZnoAuW+CLHs/czoNa37IMPUePPisGMCQKuU9aE/w9A29r/B2qijbC+KSyfF31API4bXyFaG0C6mCHjiKzSjYQynx2WPyRcWrz0/vbXfLdr1t7WhDmDKBUn1VIf4BJ4lKLjgP+M9Gt/uqJsB8/FRFm6/GeKS6djL00GwdJ02CY6r8wPFeiGlS3St5RZScWrcUz84WiBuF7Xc8/O2fYAu8KyQwjBZ1s2EAbCEEfM9zJ/57zetdLeTJPsmgnXpzoeDC6b9GXogimE+OhAzbFuwC3wQof8sz0x4NMndxoHMzquD/fEef2A3fWKAuIZ85I+aKyZPVPmA4UyjhPzfKNC83V8flWUhWPN1SAg==".to_string();
    let auth = OutlookXOauth2Auth::new(username, access_token);
    
    // 建立安全连接
    let mut tcp_conn = TcpStream::connect(format!("{}:{}", SERVER_HOST, SERVER_PORT)).unwrap();
    let mut tls_conn = build_tls_conn(SERVER_HOST).unwrap();
    let conn = Stream::new(&mut tls_conn, &mut tcp_conn);

    // XOauth2 登录
    let client = imap::Client::new(conn);
    let mut session = client.authenticate("XOAUTH2", &auth)
        .expect("Could not connect to outlook.office365.com");
    println!("login success");

    // 打开收件箱
    match session.select("INBOX") {
        Ok(mailbox) => {
            println!("{}", mailbox);
            println!("收件箱中的邮件数量: {}", mailbox.exists);
        },
        Err(e) => println!("Error selecting INBOX: {}", e),
    };

    // 读取邮件ID为2的
    match session.fetch("2", "BODY[TEXT]") {
        Ok(msgs) => {
            for message in msgs.iter() {
                let body = message.text().expect("message did not have a body!");
                let body = std::str::from_utf8(body)
                .expect("message was not valid utf-8")
                .to_string();
                print!("邮件正文: {}", body);
            }
        }
        Err(e) => println!("Error Fetching email 2: {}", e),
    };
    // 退出
    session.logout().unwrap();
}


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



/// Gmail Oauth2 认证
struct GmailOAuth2 {
    user: String,
    access_token: String,
}
impl imap::Authenticator for GmailOAuth2 {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}