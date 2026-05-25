//! lanqr - print a QR code of your LAN address in the terminal.
//!
//! Scan it from your phone to open a local dev server on the same network.
//! With no argument it encodes `http://<your-lan-ip>`; pass a port to append it,
//! or pass any URL or text to encode that verbatim.

use qrcode::render::unicode;
use qrcode::QrCode;
use std::net::UdpSocket;

/// Best-effort primary LAN IPv4.
///
/// Opens a UDP socket "to" a public address so the OS picks the outbound
/// interface, then reads the socket's local address. No packet is actually
/// sent - this just resolves which interface would be used.
fn lan_ip() -> Option<String> {
    let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
    sock.connect("8.8.8.8:80").ok()?;
    Some(sock.local_addr().ok()?.ip().to_string())
}

/// Decide what string to encode: no arg -> the LAN URL, a bare port -> that
/// port on the LAN IP, anything else -> the argument verbatim.
fn target_from_args(args: &[String], ip: &str) -> String {
    match args.first() {
        None => format!("http://{ip}"),
        Some(a) if !a.is_empty() && a.chars().all(|c| c.is_ascii_digit()) => {
            format!("http://{ip}:{a}")
        }
        Some(a) => a.clone(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if matches!(args.first().map(String::as_str), Some("-h") | Some("--help")) {
        println!(
            "lanqr - QR code of your LAN address in the terminal\n\n\
             usage: lanqr [PORT | URL | TEXT]\n  \
             (no arg)   http://<lan-ip>\n  \
             PORT       http://<lan-ip>:PORT\n  \
             URL | TEXT encoded verbatim"
        );
        return;
    }

    let ip = match lan_ip() {
        Some(ip) => ip,
        None => {
            eprintln!("lanqr: could not detect a LAN IP");
            std::process::exit(1);
        }
    };

    let target = target_from_args(&args, &ip);
    let code = match QrCode::new(target.as_bytes()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("lanqr: {e}");
            std::process::exit(1);
        }
    };

    // Render light modules on the (usually dark) terminal background so phones
    // can scan it without inverting.
    let qr = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .quiet_zone(true)
        .build();

    println!("\n{target}\n\n{qr}");
}

#[cfg(test)]
mod tests {
    use super::target_from_args;

    fn s(v: &str) -> Vec<String> {
        vec![v.to_string()]
    }

    #[test]
    fn no_arg_uses_lan_url() {
        assert_eq!(target_from_args(&[], "10.0.0.5"), "http://10.0.0.5");
    }

    #[test]
    fn numeric_arg_is_a_port() {
        assert_eq!(target_from_args(&s("9876"), "10.0.0.5"), "http://10.0.0.5:9876");
    }

    #[test]
    fn url_is_verbatim() {
        assert_eq!(target_from_args(&s("https://x.dev"), "10.0.0.5"), "https://x.dev");
    }

    #[test]
    fn text_is_verbatim() {
        assert_eq!(target_from_args(&s("hello world"), "10.0.0.5"), "hello world");
    }
}
