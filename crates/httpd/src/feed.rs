use chrono::{DateTime, Utc};
use std::net::SocketAddr;

/// Export crawl results as the RSS
pub struct Feed(String);

impl Feed {
    pub fn new(title: &str, description: Option<&str>) -> Self {
        let t = chrono::Utc::now().to_rfc2822();
        let mut b = String::new();

        b.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\"><channel>");

        b.push_str("<pubDate>");
        b.push_str(&t);
        b.push_str("</pubDate>");

        b.push_str("<lastBuildDate>");
        b.push_str(&t);
        b.push_str("</lastBuildDate>");

        b.push_str("<title>");
        b.push_str(title);
        b.push_str("</title>");

        if let Some(d) = description {
            b.push_str("<description>");
            b.push_str(d);
            b.push_str("</description>")
        }

        Self(b)
    }

    /// Appends `item` to the feed `channel`
    pub fn push(
        &mut self,
        time: &DateTime<Utc>,
        address: &SocketAddr,
        host: &String,
        map: &String,
        online: u32,
    ) {
        let a = address.to_string(); // allocate once

        self.0.push_str(&format!(
            "<item><guid>{}</guid><title>{host}</title><link>udp://{a}</link>",
            time.timestamp() // must be unique as the event
        ));

        self.0.push_str(&format!(
            "<description>online: {online}\nmap: {map}\nconnect: {a}\n</description>"
        ));

        self.0.push_str("<pubDate>");
        self.0.push_str(&time.to_rfc2822());
        self.0.push_str("</pubDate>");

        self.0.push_str("</item>")
    }

    /// Write final bytes
    pub fn commit(mut self) -> String {
        self.0.push_str("</channel></rss>");
        self.0
    }
}
