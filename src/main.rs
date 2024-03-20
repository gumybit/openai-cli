use clap::{command, Parser, Subcommand};
mod libs;
use libs::api_client::{post_chat, post_completion};

use crate::libs::api_client::{PostChatArgs, PostComletionArgs, ResponseChat, ResponseCompletion};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Turn debuggin information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Completion {
        #[arg(help = "prompt passing to ChatGPT")]
        prompt: String,

        #[arg(long, short, default_value = "gpt-3.5-turbo")]
        model: Option<String>,

        /// The suffix that comes after a completion of inserted text.
        #[arg(long, short)]
        suffix: Option<String>,

        #[arg(long, default_value = "1000")]
        max_tokens: Option<u16>,

        /// 0 to 2. What sampling temperature to use.
        #[arg(long)]
        temperature: Option<f32>,

        /// 0 to 1. the model considers the results of the tokens with top_p probability mass
        #[arg(long)]
        top_p: Option<f32>,

        /// How many completions to generate for each prompt
        #[arg(long)]
        n: Option<u16>,
    },
    Chat {
        #[arg(help = "message passing to ChatGPT")]
        message: String,

        #[arg(help = "role passing to ChatGPT", default_value = "user")]
        role: String,

        #[arg(long, short, default_value = "gpt-3.5-turbo")]
        model: Option<String>,

        #[arg(long)]
        max_tokens: Option<u16>,

        /// 0 to 2. What sampling temperature to use.
        #[arg(long)]
        temperature: Option<f32>,

        /// 0 to 1. the model considers the results of the tokens with top_p probability mass
        #[arg(long)]
        top_p: Option<f32>,

        /// How many completions to generate for each prompt
        #[arg(long)]
        n: Option<u16>,

        /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse
        #[arg(long, default_value = "user")]
        user: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Chat {
            message,
            role,
            model,
            max_tokens,
            temperature,
            top_p,
            n,
            user,
        } => {
            let config = libs::config::load_config();
            let result = post_chat(PostChatArgs {
                token: config.token,
                message,
                role,
                n,
                model,
                max_tokens,
                temperature,
                top_p,
                user,
            })
            .await
            .expect("Request error");
            let json: ResponseChat = result.json().await.expect("json error");
            json.choices.iter().for_each(|message| {
                println!("{}", trim_leading_newlines(&message.message.content));
            });
        }
        Commands::Completion {
            prompt,
            model,
            suffix,
            max_tokens,
            temperature,
            top_p,
            n,
        } => {
            let config = libs::config::load_config();
            let result = post_completion(PostComletionArgs {
                token: config.token,
                prompt,
                model,
                suffix,
                max_tokens,
                temperature,
                top_p,
                n,
            })
            .await
            .expect("Request error");
            let json: ResponseCompletion = result.json().await.expect("json error");
            json.choices.iter().for_each(|message| {
                println!("{}", trim_leading_newlines(&message.text));
            });
        }
    }

    fn trim_leading_newlines(s: &String) -> String {
        let mut s = s.clone();
        while s.starts_with('\n') {
            s.remove(0);
        }
        s
    }
}
