use chrono::{DateTime, Duration, Utc};
use colored::*;
use openssl::ssl::{SslConnector, SslMethod};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use url::Url;

fn ensure_https(url: &str) -> String {
    if url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    }
}

fn check_certificate(url: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let parsed = Url::parse(url)?;
    let host = parsed
        .host_str()
        .ok_or("Invalid URL: missing host")?;
    let port = parsed.port_or_known_default().ok_or("No port")?;
    let addr = format!("{}:{}", host, port);
    let tcp = TcpStream::connect(addr)?;
    
    let connector = SslConnector::builder(SslMethod::tls())?.build();
    let stream = connector.connect(host, tcp)?;
    
    let cert = stream.ssl().peer_certificate().ok_or("No certificate")?;
    let not_after = cert.not_after();
    use chrono::{NaiveDateTime, DateTime, Utc, TimeZone};

    let not_after_str = not_after.to_string().trim().to_string();
    // println!("Certificate expiration string: {:?}", not_after_str);

    // Remove " GMT" if it's at the end.
    let stripped = if not_after_str.ends_with(" GMT") {
        &not_after_str[..not_after_str.len() - 4]
    } else {
        &not_after_str
    };

    let naive = NaiveDateTime::parse_from_str(stripped, "%b %d %T %Y")
        .map_err(|e| format!("Failed to parse date: {}", e))?;
    let expiry: DateTime<Utc> = Utc.from_utc_datetime(&naive);

    Ok(expiry)
}

fn check_certificates(urls: &[String]) {
    for original_url in urls {
        let url = ensure_https(original_url);
        match check_certificate(&url) {
            Ok(expiry) => {
                let now = Utc::now();
                let one_day = Duration::days(1);
                let one_week = Duration::days(7);
                if expiry < now {
                    println!(
                        "{}",
                        format!(
                            "The certificate for {} has expired on {}",
                            url.bold().on_white(),
                            expiry
                        )
                        .red()
                    );
                } else if expiry - now <= one_day {
                    println!(
                        "{}",
                        format!(
                            "The certificate for {} will expire within 24 hours on {}",
                            url.bold().on_white(),
                            expiry
                        )
                        .truecolor(255, 165, 0) // orange
                    );
                } else if expiry - now <= one_week {
                    println!(
                        "{}",
                        format!(
                            "The certificate for {} will expire within one week on {}",
                            url.bold().on_white(),
                            expiry
                        )
                        .yellow()
                    );
                } else {
                    println!(
                        "{}",
                        format!(
                            "The certificate for {} is valid until {}",
                            url, expiry
                        )
                        .green()
                    );
                }
            }
            Err(e) => {
                if e.to_string().contains("certificate has expired") {
                    eprintln!(
                        "{}",
                        format!(
                            "The certificate for {}: {}",
                            url.bold().on_white(),
                            e
                        )
                        .bright_red()
                    );
                } else {
                    eprintln!(
                        "{}",
                        format!(
                            "Failed to check the certificate for {}: {}",
                            url.bold().on_white(),
                            e
                        )
                        .magenta()
                    );
                }
            }
        }
    }
}

fn read_urls_from_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut urls = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            urls.push(line.trim().to_string());
        }
    }
    Ok(urls)
}

fn main() -> Result<(), Box<dyn Error>> {
    let urls = read_urls_from_file("urls.txt")?;
    check_certificates(&urls);
    Ok(())
}
