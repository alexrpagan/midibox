use std::thread::sleep;
use std::time::Duration;
use client_example::midibox_player_client::MidiboxPlayerClient;
use client_example::{PlayRequest, PlayResponse, StopRequest, StopResponse};

pub mod client_example {
    include!("../../src/generated/midibox.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MidiboxPlayerClient::connect("http://[::1]:50051").await?;

    client.play(tonic::Request::new(PlayRequest {
        name: "foo".into()
    })).await?;

    sleep(Duration::from_secs(10));

    client.stop(tonic::Request::new(StopRequest {
        name: "foo".into()
    })).await?;

    Ok(())
}