// use std::{
//     env,
//     io::{BufRead, Write},
//     time::Duration,
// };

// use color_eyre::Result;
// use grammers_client::{Client, Config, InitParams, SignInError, Update};
// use grammers_session::Session;
// use log::info;
// use tokio::runtime;

// fn prompt(message: &str) -> Result<String> {
//     use std::io;

//     let stdout = io::stdout();
//     let mut stdout = stdout.lock();
//     stdout.write_all(message.as_bytes())?;
//     stdout.flush()?;

//     let stdin = io::stdin();
//     let mut stdin = stdin.lock();

//     let mut line = String::new();
//     stdin.read_line(&mut line)?;
//     Ok(line)
// }

// async fn async_main() -> Result<()> {
//     simple_logger::SimpleLogger::new()
//         .with_level(log::LevelFilter::Info)
//         .with_module_level("grammers_session", log::LevelFilter::Info)
//         .init()
//         .unwrap();

//     let api_id = env::var("TG_ID")?.parse().expect("TG_ID invalid");
//     let api_hash = env::var("TG_HASH")?;
//     let session_file = env::var("SESSION")?;

//     info!("Connecting to Telegram...");

//     let mut client = Client::connect(Config {
//         session: Session::load_file_or_create(&session_file)?,
//         api_id,
//         api_hash: api_hash.to_string(),
//         params: InitParams {
//             catch_up: true,
//             flood_sleep_threshold: Some(1),
//             ..Default::default()
//         },
//     })
//     .await?;

//     info!("Connected!");

//     if !client.is_authorized().await? {
//         info!("Signing in...");
//         let phone = prompt("Enter your phone number (international format): ")?;
//         let token = client.request_login_code(&phone, api_id, &api_hash).await?;
//         let code = prompt("Enter the code you received: ")?;
//         let signed_in = client.sign_in(&token, &code).await;
//         match signed_in {
//             Err(SignInError::PasswordRequired(password_token)) => {
//                 let hint = password_token
//                     .hint()
//                     .map(String::as_str)
//                     .unwrap_or_default();
//                 let prompt_message = format!("Enter the password (hint {}): ", &hint);
//                 let password = prompt(prompt_message.as_str())?;

//                 client
//                     .check_password(password_token, password.trim())
//                     .await?;
//             }
//             Ok(_) => (),
//             Err(e) => panic!("{}", e),
//         };
//         info!("Signed in!");
//         match client.session().save_to_file(&session_file) {
//             Ok(_) => {}
//             Err(e) => {
//                 info!(
//                     "NOTE: failed to save the session, will sign out when done: {}",
//                     e
//                 );
//                 client.sign_out().await?;
//             }
//         }
//     }
//     let me = client.get_me().await?;

//     info!("Logged in as @{}", me.username().unwrap_or("unknown"));

//     info!("Waiting for messages...");

//     let mut tick = tokio::time::interval(Duration::from_millis(500));

//     while let Some(update) = tokio::select! {
//         _ = tokio::signal::ctrl_c() => Ok(None),
//         result = client.next_update() => result,
//     }? {
//         match update {
//             Update::NewMessage(message) if !message.outgoing() => {
//                 let chat = message.chat();
//                 info!("{:?}", chat.pack());
//                 let msg = message.text();
//                 info!("Message from {}: {}", chat.name(), msg);
//             }
//             _ => {}
//         }
//         tick.tick().await;
//     }

//     info!("Saving session file and exiting...");
//     client.sync_update_state();
//     client.session().save_to_file(&session_file)?;
//     Ok(())
// }

// fn main() -> Result<()> {
//     color_eyre::install()?;
//     runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(async_main())
// }
