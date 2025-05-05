use lib::comm::client_instruct::RegisterRequest;
use crate::ClientError;

pub async fn register(
    addr: String,
    phone: Option<String>,
    email: Option<String>,
    username: String,
    passwd: String,
    ) -> Result<(), ClientError> {
    if phone.is_none() && email.is_none() {
        return Err(ClientError::InvalidInput(
                "No user id, email, or phone provided"
                .to_string()
                ));
    }

    let request = RegisterRequest::new(
        email, phone, username, passwd);
    let client = reqwest::Client::new();
    let response = client
        .post(&addr)
        .json(&request)
        .send()
        .await?;
    println!("Response status code: {}", response.status());
    Ok(())
}
