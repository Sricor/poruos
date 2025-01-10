use std::sync::LazyLock;

pub mod database {
    use diesel::r2d2::ConnectionManager;
    use diesel::sqlite::SqliteConnection;

    use super::LazyLock;

    type Pool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;

    pub static DATABASE: LazyLock<Pool> = LazyLock::new(|| {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        diesel::r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    });
}

pub mod claim_encrypt {
    use crate::common::cipher::ChaCha20Poly1305;

    use super::LazyLock;

    pub static ENCRYPTER: LazyLock<ChaCha20Poly1305> = LazyLock::new(|| {
        dotenvy::dotenv().ok();

        ChaCha20Poly1305::new(
            std::env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set")
                .as_bytes()
                .try_into()
                .unwrap(),
            std::env::var("SECRET_NONCE")
                .expect("SECRET_NONCE must be set")
                .as_bytes()
                .try_into()
                .unwrap(),
        )
        .unwrap()
    });
}

pub mod telegram_bot {
    use teloxide::prelude::*;
    use teloxide::utils::command::BotCommands;
    use teloxide::Bot;

    use super::LazyLock;

    pub static TELEGRAM_BOT: LazyLock<Bot> = LazyLock::new(|| {
        dotenvy::dotenv().ok();

        let telegram_token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");

        let bot = Bot::new(telegram_token);

        let command_bot = bot.clone();

        tokio::spawn(async move {
            Command::repl(command_bot, answer).await;
        });

        bot
    });

    #[derive(BotCommands, Clone)]
    #[command(rename_rule = "lowercase", description = "These commands are supported:")]
    enum Command {
        #[command(description = "display this text.")]
        Help,
        #[command(description = "handle a username.")]
        Username(String),
        #[command(description = "handle a username and an age.", parse_with = "split")]
        UsernameAndAge { username: String, age: u8 },
    }

    async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
            Command::Username(username) => {
                bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
            }
            Command::UsernameAndAge { username, age } => {
                bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                    .await?
            }
        };

        Ok(())
    }

}