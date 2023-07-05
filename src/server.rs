use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam::atomic::AtomicCell;
use env_logger::init;
use log::info;
use tonic::{transport::Server, Request, Response, Status};
use ::midibox::meter::Bpm;
use ::midibox::player::{PlayerConfig, try_run_ext};
use ::midibox::scale::{Degree, Interval, Scale};
use ::midibox::sequences::Seq;
use ::midibox::tone::Tone;

use crate::midibox::midibox_player_server::{MidiboxPlayer, MidiboxPlayerServer} ;
use crate::midibox::{GetStatusRequest, GetStatusResponse, MIDIBOX_DESCRIPTOR_SET, PlayRequest, PlayResponse, StopRequest, StopResponse};

pub mod midibox {
    include!("generated/midibox.rs");

    pub(crate) const MIDIBOX_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("midibox");
}


#[derive(Debug, Default)]
pub struct Impl {
    running: Arc<Mutex<HashMap<String, bool>>>,
}

fn play_default_sequence(name: &str, running: &Arc<Mutex<HashMap<String, bool>>>) {
    let scale = Scale::major(Tone::Gb);

    let s1 = Seq::new(vec![
        Tone::G.oct(2)  * 128,
        Tone::B.oct(2)  * 128,
        Tone::E.oct(2)  * 128,
        Tone::D.oct(2)  * 128,
        Tone::C.oct(2)  * 128,
        Tone::E.oct(2)  * 128,
        Tone::B.oct(2)  * 128,
        Tone::C.oct(2)  * 128,
    ]).transpose_down(Interval::Min2);

    try_run_ext(
        name,
        PlayerConfig::for_port(0),
        &Bpm::new(2000),
        &mut vec![
            s1.clone(),
            s1.clone().harmonize_down(&scale, Degree::Fourth),
            s1.clone().harmonize_up(&scale, Degree::Tenth),
            s1.clone().harmonize_up(&scale, Degree::Seventh)
        ].into_iter().map(|seq| seq.midibox()).collect(),
        running
    ).unwrap()
}

#[tonic::async_trait]
impl MidiboxPlayer for Impl {
    async fn get_status(
        &self,
        _: Request<GetStatusRequest>
    ) -> Result<Response<GetStatusResponse>, Status> {
        let reply = GetStatusResponse {
            playing: self.running.lock().unwrap()
                .keys().into_iter().map(|x| x.to_string()).collect()
        };
        Ok(Response::new(reply))
    }

    async fn play(
        &self,
        request: Request<PlayRequest>
    ) -> Result<Response<PlayResponse>, Status> {
        let name = request.get_ref().name.clone();
        let mut status = self.running.lock().unwrap();
        if !*status.get(&name).unwrap_or(&false) {
            status.insert(name.to_string(), true);
            let running = self.running.clone();
            thread::spawn(move || play_default_sequence(&name, &running));
        }

        let reply = PlayResponse {};
        Ok(Response::new(reply))
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        let name = &request.get_ref().name;
        self.running.lock().unwrap().insert(name.to_string(), false);
        let reply = StopResponse {};
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();

    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(MIDIBOX_DESCRIPTOR_SET)
        .build()?;

    let addr = "[::1]:50051".parse()?;
    let player = Impl {
        running: Arc::new(Mutex::new(HashMap::new())),
    };

    info!("Starting player server");

    Server::builder()
        .add_service(reflection_server)
        .add_service(MidiboxPlayerServer::new(player))
        .serve(addr)
        .await?;

    Ok(())
}