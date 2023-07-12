//! Network requests to server.

use bevy::prelude::*;
use bevy_mod_reqwest::*;
use durak_lib::request::CardPlayed;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ReqwestPlugin).add_event::<CardPlayed>();
    }
}

fn send_card_played_request(mut commands: Commands, mut event_reader: EventReader<CardPlayed>) {
    let Ok(url) = "https://www.boredapi.com/api/activity".try_into() else {
        panic!("Invalid url");
    };
    let req = reqwest::Request::new(reqwest::Method::GET, url);
    let req = ReqwestRequest::new(req);
    commands.spawn(req);
}
