use std::{ io::Write, fs::File };
use crate::models::JoinedChannel;

pub struct Playlist {
    /// Joined channels
    joined_channels: Vec<JoinedChannel>,
    /// Whether a playlist is for Kodi
    is_for_kodi: bool,
}

impl Playlist {
    /// Playlist constructor.
    pub fn new(joined_channels: Vec<JoinedChannel>, is_for_kodi: bool) -> Playlist {
        Playlist { joined_channels, is_for_kodi }
    }

    /// Escapes a comma.
    fn escape_comma(text: &str) -> String {
        text.replace(",", "\u{201A}")
    }

    /// Formats a category name.
    fn format_category_name(&self, category_name: &str) -> String {
        if self.is_for_kodi {
            format!("[COLOR powderblue][LIGHT]{} / [/LIGHT][/COLOR]", category_name)
        } else {
            format!("{} / ", category_name)
        }
    }

    /// Returns a playlist in the M3U8 format.
    fn get_m3u8(&self) -> String {
        let mut m3u8 = "#EXTM3U\n".to_string();

        for x in &self.joined_channels {
            let mut properties = vec![];
            let mut postfix_user_agent = "".to_string();

            if self.is_for_kodi && x.source_channel.url.ends_with(".mpd") {
                properties.append(&mut vec![
                    "#KODIPROP:inputstreamaddon=inputstream.adaptive".to_string(),
                    "#KODIPROP:inputstream.adaptive.manifest_type=mpd".to_string(),
                ]);
            }

            if let Some(x) = &x.source.user_agent {
                if self.is_for_kodi {
                    postfix_user_agent = format!("|User-Agent=\"{}\"", x);
                } else {
                    properties.push(format!("#EXTVLCOPT:http-user-agent=\"{}\"", x));
                }
            }

            m3u8 += &format!(
                "#EXTINF:-1{tv_guide_channel_external_id}{channel_tv_guide_logo},\
{category_name}{channel_name}\n\
{properties}{source_channel_url}{source_query_string}{postfix_user_agent}\n",
                tv_guide_channel_external_id = if let Some(x) = &x.tv_guide_channel {
                    format!(" tvg-id=\"{}\"", x.external_id)
                } else { "".to_string() },
                channel_tv_guide_logo = if let Some(x) = &x.channel.tv_guide_logo {
                    format!(" tvg-logo=\"{}\"", x)
                } else { "".to_string() },
                category_name = if let Some(x) = &x.category {
                    self.format_category_name(&Playlist::escape_comma(&x.short_name))
                } else { "".to_string() },
                channel_name = Playlist::escape_comma(&x.channel.name),
                properties = properties
                    .iter()
                    .map(|x| x.to_owned() + "\n")
                    .collect::<Vec<String>>()
                    .join(""),
                source_channel_url = &x.source_channel.url,
                source_query_string = if let Some(x) = &x.source.query_string {
                    format!("?{}", x)
                } else { "".to_string() },
                postfix_user_agent = postfix_user_agent,
            );
        }

        m3u8
    }

    /// Writes a playlist to a file.
    pub fn write_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Cannot create a file.");
        file.write_all(self.get_m3u8().as_ref()).expect("Cannot write a playlist to a file.");
    }
}
