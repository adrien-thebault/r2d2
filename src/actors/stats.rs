use actix::prelude::*;
use actors::SlackClient;
use chrono::prelude::*;
use messages::{Command, Envelope, SaveRequest, SendMessage, SendTyping};
use slack;
use std::{cmp, collections::HashMap, default::Default, time::Duration};

lazy_static! {
    static ref USER_FILE: String = {
        let mut dir = String::from(config!(data_dir));
        dir.push_str("stats_users.json");
        dir
    };
    static ref BOT_FILE: String = {
        let mut dir = String::from(config!(data_dir));
        dir.push_str("stats_r2d2.json");
        dir
    };
}

pub struct Stats {
    slack_client: Addr<SlackClient>,
    user_stats: HashMap<String, UserStats>,
    bot_stats: BotStats,
}

#[derive(Clone, Serialize, Deserialize)]
struct UserStats {
    nb_messages: u64,
}

impl Default for UserStats {
    fn default() -> Self {
        Self { nb_messages: 0 }
    }
}

#[derive(Serialize, Deserialize)]
struct BotStats {
    since: DateTime<Local>,
}

impl Default for BotStats {
    fn default() -> Self {
        Self {
            since: Local::now(),
        }
    }
}

impl Stats {
    pub fn new(slack_client: Addr<SlackClient>) -> Self {
        Self {
            slack_client,
            user_stats: super::load(&USER_FILE),
            bot_stats: super::load(&BOT_FILE),
        }
    }
}

impl Actor for Stats {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify_later(
            SaveRequest::default(),
            Duration::from_secs(*config!(save_interval)),
        );
        info!("stats started")
    }
}

impl Handler<Envelope> for Stats {
    type Result = ();
    fn handle(&mut self, msg: Envelope, ctx: &mut Self::Context) -> Self::Result {
        debug!("received envelope");
        debug!("{:?}", msg);
        match msg {
            Envelope::Command(cmd) => ctx.notify(*cmd),
            Envelope::Message(msg) => match *msg {
                slack::Message::Standard(ref m) => match &m.user {
                    Some(user) => {
                        info!("message from {}", user);
                        let mut stats = match self.user_stats.get(user) {
                            Some(v) => v.clone(),
                            None => UserStats::default(),
                        };
                        stats.nb_messages += 1;
                        self.user_stats.insert(user.to_string(), stats);
                    }
                    None => warn!("received slack::Message from unknown user"),
                },
                _ => debug!("received unhandled slack::Message: {:?}", msg),
            },
            Envelope::Event(evt) => debug!("received unhandled slack::Event : {:?}", evt),
        }
    }
}

impl Handler<SaveRequest> for Stats {
    type Result = ();
    fn handle(&mut self, _msg: SaveRequest, ctx: &mut Self::Context) -> Self::Result {
        info!("saving data");
        super::save(&USER_FILE, &self.user_stats);
        super::save(&BOT_FILE, &self.bot_stats);
        ctx.notify_later(
            SaveRequest::default(),
            Duration::from_secs(*config!(save_interval)),
        );
    }
}

impl Handler<Command> for Stats {
    type Result = ();
    fn handle(&mut self, msg: Command, _ctx: &mut Self::Context) -> Self::Result {
        debug!("received command");
        match msg.channel() {
            Some(channel) => {
                if let Some(cmd) = msg.command() {
                    match cmd.as_ref() {
                        "stats" => {
                            self.slack_client.do_send(SendTyping::new(channel));

                            let mut total_msg = 0;
                            let mut classement = Vec::new();

                            for (k, v) in &self.user_stats {
                                total_msg += v.nb_messages;
                                classement.push((k, v));
                            }

                            classement.sort_by(|x, y| y.1.nb_messages.cmp(&x.1.nb_messages));

                            let mut stats = format!(
                                concat!(
                                    "J'ai démarré le {}/{}/{} à {}h{}\n",
                                    "{} messages ont été postés depuis cette date.\n\n",
                                    "Classement des membres les plus actifs :\n"
                                ),
                                if self.bot_stats.since.day() > 9 {
                                    format!("{}", self.bot_stats.since.day())
                                } else {
                                    format!("0{}", self.bot_stats.since.day())
                                },
                                if self.bot_stats.since.month() > 9 {
                                    format!("{}", self.bot_stats.since.month())
                                } else {
                                    format!("0{}", self.bot_stats.since.month())
                                },
                                self.bot_stats.since.year(),
                                self.bot_stats.since.hour(),
                                if self.bot_stats.since.minute() > 9 {
                                    format!("{}", self.bot_stats.since.minute())
                                } else {
                                    format!("0{}", self.bot_stats.since.minute())
                                },
                                total_msg
                            );

                            for (i, us) in classement
                                .iter()
                                .enumerate()
                                .take(cmp::min(classement.len(), 5))
                            {
                                stats.push_str(&format!(
                                    "{} - <@{}> : {} message{}\n",
                                    i + 1,
                                    us.0,
                                    us.1.nb_messages,
                                    if us.1.nb_messages > 1 { 's' } else { ' ' }
                                ))
                            }

                            self.slack_client.do_send(SendMessage::new(channel, &stats));
                        }
                        _ => debug!("received unhandled command: {:?}", msg),
                    }
                } else {
                    debug!("received unhandled command: {:?}", msg)
                }
            }
            None => warn!("dropping command : received command with an empty channel"),
        }
    }
}
