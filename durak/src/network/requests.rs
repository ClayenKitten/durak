use bevy::prelude::*;
use bevy_mod_reqwest::reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Method, Url,
};
use durak_lib::{
    game::card::Card,
    network::{AuthHeader, CreateGameData, CreateGameResponce, JoinGameData, JoinGameResponce},
    status::{GameState, GameStatus},
};

use super::MyRequest;

#[derive(Debug, Component)]
pub struct CreateGameRequest(pub CreateGameData);

impl MyRequest for CreateGameRequest {
    type Responce = CreateGameResponce;

    type Query = [(&'static str, String); 1];

    fn method(&self) -> Method {
        Method::POST
    }

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

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/join", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn query(&self) -> Option<Self::Query> {
        Some([
            ("id", self.0.id.0.to_string()),
            ("password", self.0.password.clone()),
        ])
    }
}

#[derive(Debug, Component)]
pub struct LeaveGameRequest(pub AuthHeader);

impl MyRequest for LeaveGameRequest {
    type Responce = ();

    type Query = ();

    fn method(&self) -> Method {
        Method::POST
    }

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

#[derive(Debug, Component)]
pub struct StateRequest(pub AuthHeader);

impl MyRequest for StateRequest {
    type Responce = GameState;

    type Query = ();

    fn method(&self) -> Method {
        Method::GET
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/state", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}

#[derive(Debug, Component)]
pub struct StatusRequest(pub AuthHeader);

impl MyRequest for StatusRequest {
    type Responce = GameStatus;

    type Query = ();

    fn method(&self) -> Method {
        Method::GET
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/status", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}

#[derive(Debug, Component)]
pub struct StartGameRequest(pub AuthHeader);

impl MyRequest for StartGameRequest {
    type Responce = ();

    type Query = ();

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/start", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}

#[derive(Debug, Component)]
pub struct PlayCardRequest {
    pub auth: AuthHeader,
    pub card: Card,
}

impl MyRequest for PlayCardRequest {
    type Responce = ();

    type Query = Card;

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/play", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn query(&self) -> Option<Self::Query> {
        Some(self.card)
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.auth.into_header());
        map
    }
}
