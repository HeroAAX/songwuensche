use egui::TextBuffer;
use twitch_irc::message::PrivmsgMessage;

#[derive(Clone)]
pub struct RecommendedSong {
    pub user: String,
    pub title: String,
    pub url: String,
    pub id: u64,
}

impl RecommendedSong {
    pub fn random() -> Self {
        let user = format!("{}", rand::random::<f32>());
        let title = format!("{}", rand::random::<f32>());
        let url = format!("{}", rand::random::<f32>());

        let id = rand::random();

        Self {
            user,
            title,
            url,
            id,
        }
    }

    pub async fn fetch_yt_title(&mut self) {
        let res = reqwest::get(self.url.as_str()).await;

        let body = &match res {
            Ok(res) => res.text().await,
            _ => return,
        };

        let body = match body {
            Ok(res) => res,
            _ => return,
        };

        let mut body_split = body.split("<title>");
        body_split.next().unwrap(); // throw away everything before "<title>"

        let body_split = match body_split.next() {
            Some(res) => res,
            _ => return,
        }
        .split("</title>")
        .next();

        let mut body_split = match body_split {
            Some(res) => res,
            _ => return,
        };

        for i in (0..body_split.len()).step_by(22) {
            body_split.insert_text("\n", i);
        }

        self.title = body_split.to_owned();
    }
}

pub struct SongRecommendationSolver {
    filter_urls: Vec<String>,
    current_id: u64,
}

impl SongRecommendationSolver {
    pub fn new(filter_urls: Vec<String>) -> Self {
        Self {
            filter_urls,
            current_id: 0u64,
        }
    }

    pub async fn extract_recommendation(
        &mut self,
        irc_message: PrivmsgMessage,
    ) -> Vec<RecommendedSong> {
        let mut out: Vec<RecommendedSong> = vec![];
        let message_split = irc_message.message_text.split_whitespace();

        for message in message_split {
            for filter in self.filter_urls.clone() {
                if message.starts_with(filter.as_str()) {
                    out.push(self.gen_recommended_song(message, &irc_message).await);
                }
            }
        }
        out
    }

    async fn gen_recommended_song(
        &mut self,
        message: &str,
        irc_msg: &PrivmsgMessage,
    ) -> RecommendedSong {
        self.current_id += 1;
        let mut user = irc_msg.sender.name.clone();

        if user.capacity() > 30 {
            user = user[0..30].to_owned();
        }

        RecommendedSong {
            title: "-".to_owned(),
            url: message.to_owned(),
            user,
            id: self.current_id,
        }
    }
}
