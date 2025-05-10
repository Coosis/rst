use lib::comm::client_instruct::ShowInvitesRequest;
use lib::comm::ClientMessage;
use crate::LockedState;
use crate::Result;
use crate::ClientError;

pub async fn show_invites(state: &LockedState<'_>)
    -> Result<()> {
    let req: ClientMessage = ShowInvitesRequest::new().try_into()?;
    if let Some(conn) = &state.connection {
        conn.send(req).await?;
    } else {
        return Err(ClientError::ConnectionError(
            "No connection available".to_string(),
        ));
    }

    // read_loop(ws_stream, async move |msg: ServerMessage| {
    //     match msg.instruct {
    //         ServerInstruct::ShowInvitesResponse => {
    //             println!("Show invites response received: ");
    //             let msg = parse_inner::<ShowInvitesResponse>(&msg.content);
    //             for invite in msg.invites {
    //                 println!("{:?}", invite);
    //             }
    //             ControlFlow::Break(ClientError::None)
    //         }
    //         _ => {
    //             println!("Unexpected message type: {:?}", msg.instruct);
    //             ControlFlow::Continue(())
    //         }
    //     }
    // }).await?;
    Ok(())
}
