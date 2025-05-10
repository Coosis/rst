use lib::Uuid;
use tokio;
mod subcommands;
use subcommands::*;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
mod error;
use crate::error::ClientError;
use clap::Parser;
use clap::Subcommand;

mod core;
use core::*;

mod login;
use login::*;

mod connection;
use connection::*;

const SERVER: &str = "ws://127.0.0.1:3000/ws";

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

/// Subcommands used by rst-client
#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    /// Connect to server
    Connect {
        /// Address to connect to
        addr: String, 
        #[arg(short, long, value_name = "sub", default_value_t = false)]
        sub: bool,
    },

    /// Register a user
    Register {
        /// Address to connect to
        addr: String, 
        #[arg(long, value_name = "phone")]
        phone: Option<String>,
        #[arg(long, value_name = "email")]
        email: Option<String>,
        #[arg(short, long, value_name = "username")]
        username: String,
        #[arg(short, long, value_name = "passwd")]
        passwd: String,
    },

    /// Send a request to a user to chat
    SendRequest {
        /// Address to connect to
        addr: String, 
        #[arg(short, long, value_name = "token")]
        token: String,
        #[arg(long, value_name = "email")]
        email: Option<String>,
        #[arg(long, value_name = "phone")]
        phone: Option<String>,
        #[arg(long, value_name = "id")]
        id: Option<String>,

        #[arg(long, value_name = "name")]
        name: String,
        #[arg(long, value_name = "description")]
        description: String,
    },

    ShowInvites {
        /// Address to connect to
        addr: String,
        #[arg(short, long, value_name = "token")]
        token: String,
    },

    AcceptInvite {
        /// Address to connect to
        addr: String,
        #[arg(short, long, value_name = "token")]
        token: String,
        #[arg(short, long, value_name = "invite_id")]
        invite_id: String,
    },

    SendMessage {
        /// Address to connect to
        addr: String,
        #[arg(short, long, value_name = "token")]
        token: String,
        #[arg(short, long, value_name = "chat_id")]
        chat_id: String,
        #[arg(short, long, value_name = "message")]
        message: String,
    },

    ShowChats {
        /// Address to connect to
        addr: String,
        #[arg(short, long, value_name = "token")]
        token: String,
    }
}

// #[tokio::main]
// async fn main() -> Result<(), ClientError> {
//     let cli = Cli::parse();
//     match cli.subcommand {
//         Commands::Connect { addr, sub } => connect(addr, sub).await,
//         Commands::Register { addr, phone, email, username, passwd } 
//             => register(addr, phone, email, username, passwd).await,
//         Commands::SendRequest { addr, token, email, phone, id, name, description }=> {
//             let addr = format!("{}?token={}", addr, token);
//             send_chat_request(&addr, email, phone, id, name, description).await
//         },
//         Commands::ShowInvites { addr, token } => {
//             let addr = format!("{}?token={}", addr, token);
//             show_invites(&addr).await
//         },
//         Commands::AcceptInvite { addr, token, invite_id } => {
//             let addr = format!("{}?token={}", addr, token);
//             accept_invite(&addr, invite_id).await
//         },
//         Commands::SendMessage { addr, token, chat_id, message } => {
//             let addr = format!("{}?token={}", addr, token);
//             let chat_id = Uuid::parse_str(&chat_id)
//                 .map_err(|_| ClientError::InvalidInput("Invalid chat id".to_string()))?;
//             send_message(&addr, chat_id, message).await
//         }
//         Commands::ShowChats { addr, token } => {
//             let addr = format!("{}?token={}", addr, token);
//             show_chats(&addr).await
//         }
//         // _ => {
//         //     println!("Not implemented");
//         //     Ok(())
//         // }
//     //     // Commands::Login { addr, email, password } 
//     //     //     => login(&addr, email, password).await,
//     }
// }

use std::thread;

use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, DefaultPrompt, Emacs, ExternalPrinter, KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent, ReedlineMenu, Signal
};
pub struct State {
    pub connected: bool,
    pub server: String,
    pub token: Option<String>,
    pub connection: Option<Connection>
}

impl State {
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.token = None;
    }
}

type Result<T> = std::result::Result<T, ClientError>;
type SharedState = std::sync::Arc<tokio::sync::Mutex<State>>;
type LockedState<'a> = tokio::sync::MutexGuard<'a, State>;

#[tokio::main]
async fn main() {
    let prompt = DefaultPrompt::default();
    let commands = vec![
        "h".into(),
        "help".into(),
        "clear".into(),
        "status".into(),
        "server".into(),
        "set server".into(),
        "login".into(),
        "logout".into(),
        "register".into(),
        "send invite".into(),
        "show invites".into(),
        "accept invite".into(),
        "send message".into(),
        "list chats".into(),
    ];
    let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
    // Use the interactive menu to select options from the completer
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
    // Set up the required keybindings
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );
    let printer = ExternalPrinter::<String>::default();

    let edit_mode = Box::new(Emacs::new(keybindings));
    
    let state = std::sync::Arc::new(tokio::sync::Mutex::new(State {
        connected: false,
        server: "ws://localhost:3000/ws".to_string(),
        token: None,
        connection: None,
    }));

    let separator = "=".repeat(30);
    let mut line_editor = Reedline::create()
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode)
        .with_external_printer(printer.clone());
    // let _ = line_editor.clear_scrollback();
    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                match buffer.as_str() {
                    "h" | "help" => {
                        print_help();
                    }
                    "clear" => {
                        let _ = line_editor.clear_scrollback();
                    }
                    "status" => {
                        let state = state.lock().await;
                        println!("Connected: {}", state.connected);
                        println!("Server: {}", state.server);
                        if let Some(token) = &state.token {
                            println!("Token: {}", token);
                        } else {
                            println!("No token");
                        }
                    }
                    "server" => {
                        println!("Current server address: ");
                        let state = state.lock().await;
                        println!("{}", state.server);
                        println!()
                    } 
                    "set server" => {
                        println!("set server address: ");
                        let mut server = String::new();
                        std::io::stdin().read_line(&mut server).unwrap();
                        let server = server.trim();
                        if server.is_empty() {
                            println!("Server address cannot be empty");
                            continue;
                        }
                        println!("Server address set to: {}", server);
                    }
                    "login" => {
                        match login(&state, printer.clone()).await {
                            Ok(_) => {
                                println!("Logged in");
                            }
                            Err(e) => {
                                println!("Failed to login: {}", e);
                            }
                        }
                    }
                    "logout" => {
                        let mut state = state.lock().await;
                        state.disconnect();
                        println!("Logged out");
                    }
                    "register" => {
                        let state = state.lock().await;
                        println!("Registering...");
                        println!("Email: (you can leave empty)");
                        let mut email = String::new();
                        std::io::stdin().read_line(&mut email).unwrap();
                        email = email.trim().to_string();
                        println!("Phone: (you can leave empty)");
                        let mut phone = String::new();
                        std::io::stdin().read_line(&mut phone).unwrap();
                        phone = phone.trim().to_string();
                        if email.is_empty() && phone.is_empty() {
                            println!("No email, or phone provided, please at least provide one");
                            continue;
                        }
                        println!("Username: ");
                        let mut username = String::new();
                        std::io::stdin().read_line(&mut username).unwrap();
                        username = username.trim().to_string();
                        if username.is_empty() {
                            println!("Username cannot be empty");
                            continue;
                        }

                        println!("Password: ");
                        let mut passwd = String::new();
                        std::io::stdin().read_line(&mut passwd).unwrap();
                        passwd = passwd.trim().to_string();
                        if passwd.is_empty() {
                            println!("Password cannot be empty");
                            continue;
                        }

                        println!("Registering with server: {}", REGISTER_SERVER);
                        println!("Phone: {}", phone);
                        println!("Email: {}", email);
                        println!("Username: {}", username);
                        println!("Password: {}", passwd);
                        println!("Are you sure? (Y/n)");
                        let mut confirm = String::new();
                        std::io::stdin().read_line(&mut confirm).unwrap();
                        confirm = confirm.trim().to_string();
                        if confirm.to_uppercase() != "Y" && !confirm.is_empty() {
                            println!("Aborted!");
                            continue;
                        }
                        let phone = if phone.is_empty() { None } else { Some(phone) };
                        let email = if email.is_empty() { None } else { Some(email) };
                        match register(
                            REGISTER_SERVER.to_string(),
                            phone, email,
                            username, passwd).await {
                            Ok(_) => {
                                println!("Registered");
                            }
                            Err(e) => {
                                println!("Failed to register: {}", e);
                            }
                        };
                    }

                    "send invite" => {
                        let state = state.lock().await;
                        println!("Sending invite...");
                        println!("Other party's email: (you can leave empty)");
                        let mut email = String::new();
                        std::io::stdin().read_line(&mut email).unwrap();
                        email = email.trim().to_string();
                        println!("Other party's phone: (you can leave empty)");
                        let mut phone = String::new();
                        std::io::stdin().read_line(&mut phone).unwrap();
                        phone = phone.trim().to_string();
                        let mut uid = String::new();
                        println!("Other party's user id: (you can leave empty)");
                        std::io::stdin().read_line(&mut uid).unwrap();
                        uid = uid.trim().to_string();
                        if email.is_empty() && phone.is_empty() && uid.is_empty() {
                            println!("No email, phone, or user id provided, please at least provide one");
                            continue;
                        }
                        let email = if email.is_empty() { None } else { Some(email) };
                        let phone = if phone.is_empty() { None } else { Some(phone) };
                        let uid = if uid.is_empty() { None } else { Some(uid) };
                        println!("Choose a chat name: ");
                        let mut name = String::new();
                        std::io::stdin().read_line(&mut name).unwrap();
                        name = name.trim().to_string();
                        if name.is_empty() {
                            println!("Chat name cannot be empty");
                            continue;
                        }

                        println!("Description: ");
                        let mut description = String::new();
                        std::io::stdin().read_line(&mut description).unwrap();
                        description = description.trim().to_string();

                        println!("{}", separator);
                        println!("Sending invite inside server: {} to: ", state.server);
                        println!("Phone: {:?}", phone);
                        println!("Email: {:?}", email);
                        println!("Name: {}", name);
                        println!("Description: {}", description);
                        println!("{}", separator);

                        let mut confirm = String::new();
                        println!("Are you sure? (Y/n)");
                        std::io::stdin().read_line(&mut confirm).unwrap();
                        confirm = confirm.trim().to_string();
                        if confirm.to_uppercase() != "Y" && !confirm.is_empty() {
                            println!("Aborted!");
                            continue;
                        }

                        match send_chat_request(
                            &state,
                            email, phone, uid, 
                            name, description).await {
                            Ok(_) => {
                                println!("Invite sent");
                            }
                            Err(e) => {
                                println!("Failed to send invite: {}", e);
                            }
                        }
                    }

                    "show invites" => {
                        let state = state.lock().await;
                        println!("Showing invites...");
                        match show_invites(&state).await {
                            Ok(_) => {
                                println!("Invites shown");
                            }
                            Err(e) => {
                                println!("Failed to show invites: {}", e);
                            }
                        }
                    }

                    "accept invite" => {
                        let state = state.lock().await;
                        println!("Accepting invite...");
                        println!("Invite id: ");
                        let mut invite_id = String::new();
                        std::io::stdin().read_line(&mut invite_id).unwrap();
                        invite_id = invite_id.trim().to_string();
                        if invite_id.is_empty() {
                            println!("Invite id cannot be empty");
                            continue;
                        }
                        match accept_invite(&state, invite_id).await {
                            Ok(_) => {
                                println!("Invite accepted");
                            }
                            Err(e) => {
                                println!("Failed to accept invite: {}", e);
                            }
                        }
                    }
                    "list chats" => {
                        let state = state.lock().await;
                        println!("Listing chats...");
                        match show_chats(&state).await {
                            Ok(_) => {
                                println!("Chats listed");
                            }
                            Err(e) => {
                                println!("Failed to list chats: {}", e);
                            }
                        }
                    }

                    "send message" => {
                        let state = state.lock().await;
                        println!("Sending message...");
                        show_chats(&state).await.unwrap();
                        println!("To(chat id): ");
                        let mut chat_id = String::new();
                        std::io::stdin().read_line(&mut chat_id).unwrap();
                        chat_id = chat_id.trim().to_string();
                        if chat_id.is_empty() {
                            println!("Chat id cannot be empty");
                            continue;
                        }
                        println!("Message: ");
                        let mut message = String::new();
                        std::io::stdin().read_line(&mut message).unwrap();
                        message = message.trim().to_string();
                        if message.is_empty() {
                            println!("Message cannot be empty");
                            continue;
                        }
                        let chat_id = match Uuid::parse_str(&chat_id) {
                            Ok(id) => id,
                            Err(_) => {
                                println!("Invalid chat id");
                                continue;
                            }
                        };
                        match send_message(&state, chat_id, message).await {
                            Ok(_) => {
                                println!("Message sent");
                            }
                            Err(e) => {
                                println!("Failed to send message: {}", e);
                            }
                        }
                    }

                    _ => {
                        println!("Unknown command: {}", buffer);
                    }
                }
            }
            Ok(Signal::CtrlC) => { }
            Ok(Signal::CtrlD) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}

pub fn print_help() {
    println!("Commands:");
    println!("h/help: Show this help");
    println!("clear: Clear the screen");
    println!("status: Show current status");
    println!("server: Show current server address");
    println!("set server: Set the server address");
    println!("login: Login to the server");
    println!("logout: Logout from the server");
    println!("register: Register a new user");
    println!("send invite: Send a request to a user to chat");
    println!("show invites: Show invites");
    println!("accept invite: Accept an invite");
    println!("send message: Send a message to a chat");
    println!("list chats: list chats");
}


pub fn parse_inner<T>(inner: &[u8]) -> T
where
    T: serde::de::DeserializeOwned
{
    serde_json::from_slice(inner).unwrap()
}
