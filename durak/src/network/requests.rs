use bevy::prelude::*;
use bevy_mod_reqwest::reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Url,
};
use durak_lib::network::{
    AuthHeader, CreateGameData, CreateGameResponce, JoinGameData, JoinGameResponce,
};

use super::MyRequest;

#[derive(Debug, Component)]
pub struct CreateGameRequest(pub CreateGameData);

impl MyRequest for CreateGameRequest {
    type Responce = CreateGameResponce;

    type Query = [(&'static str, String); 1];

    fn url(&self) -> Url {
        let url = format!("{}/create", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn query(&self) -> Option<Self::Query> {
        Some([("password", self.0.password.clone())])
    }
}

#[derive(Debug, Component)]
pub struct JoinGameRequest(pub JoinGameData);

impl MyRequest for JoinGameRequest {
    type Responce = JoinGameResponce;

    type Query = [(&'static str, String); 2];

    fn url(&self) -> Url {
        let url = format!("{}/join", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn query(&self) -> Option<Self::Query> {
        Some([
            ("id", self.0.id.to_string()),
            ("password", self.0.password.clone()),
        ])
    }
}

#[derive(Debug, Component)]
pub struct LeaveGameRequest(pub AuthHeader);

impl MyRequest for LeaveGameRequest {
    type Responce = ();

    type Query = ();

    fn url(&self) -> Url {
        let url = format!("{}/game/leave", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}
