use std::env;
use bot::Bot;

mod bot;

fn err_msg(message: &str) {
    println!("{}. Press any key to exit", message);
            std::io::stdin().read_line(&mut String::new()).unwrap();
}

#[tokio::main]
async fn main() {
    // We also want to option to store the token in a `.env` file
    dotenv::dotenv().ok();

    // Grab the bot token from the environment
    let token;
    match env::var("{{token_variable_name}}"){
        Ok(t) => token = t,
        Err(_) => {
            err_msg("Could not find bot token in environment");
            return;
        }
    }

    // Run the bot. If it errors out, let the user know.
    if let Err(err) = Bot::run(token.as_str()).await{
        err_msg("The bot encountered a catastrophic error. Please contact the bot creator if this happens frequently");
        return;
    }
}
