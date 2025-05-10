use lib::comm::client_instruct::{LoginRequest, LoginType};
use reedline::ExternalPrinter;

use crate::Connection;
use crate::{error::ClientError, SharedState, AUTH_SERVER};
use crate::Result;

pub async fn login(
    state: &SharedState, 
    printer: ExternalPrinter<String>
) -> Result<()> {
    println!("Login to server using email: ");
    let mut email = String::new();
    let mut phone = String::new();
    let mut uid = String::new();
    std::io::stdin().read_line(&mut email).unwrap();
    email = email.trim().to_string();
    if email.is_empty() {
        println!("Phone: ");
        std::io::stdin().read_line(&mut phone).unwrap();
        phone = phone.trim().to_string();
    }
    if email.is_empty() && phone.is_empty() {
        println!("User id: ");
        std::io::stdin().read_line(&mut uid).unwrap();
        uid = uid.trim().to_string();
    }
    if email.is_empty() && phone.is_empty() && uid.is_empty() {
        return Err(ClientError::InvalidInput(
            "No user id, email, or phone provided".to_string()
        ));
    }
    println!("Password: ");
    let mut passwd = String::new();
    std::io::stdin().read_line(&mut passwd).unwrap();
    passwd = passwd.trim().to_string();
    if passwd.is_empty() {
        return Err(ClientError::InvalidInput(
            "Password cannot be empty".to_string()
        ));
    }
    let client = reqwest::Client::new();

    let login_request = if !email.is_empty() {
        LoginRequest::new(
            LoginType::ByEmail,
            email,
            passwd
        )
    } else if !phone.is_empty() {
        LoginRequest::new(
            LoginType::ByPhone,
            phone, 
            passwd
        )
    } else {
        LoginRequest::new(
            LoginType::ByUserID,
            uid,
            passwd
        )
    };
    let response = match client.post(AUTH_SERVER)
        .json(&login_request)
        .send()
        .await {
        Ok(response) => response,
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            return Err(ClientError::ConnectionError(
                format!("Failed to connect to auth server: {}", e).to_string()
            ))
        }
    };
    println!("Response status code: {}", response.status());
    let token = match response
        .text()
        .await {
        Ok(token) => token,
        Err(e) => {
            return Err(ClientError::ConnectionError(
                format!("Failed to get token from auth server: {}", e).to_string()
            ))
        }
    };
    println!("Token: {}", token);

    {
        let mut state = state.lock().await;
        state.token = Some(token.clone());
        state.connected = false;
    }

    match Connection::connect(state, printer).await {
        Ok(conn) => {
            let mut state = state.lock().await;
            println!("Connected to server: {}", state.server);
            state.connection = Some(conn);
            Ok(())
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            Err(ClientError::ConnectionError(
                format!("Failed to connect to server: {}", e).to_string()
            ))
        }
    }
}
