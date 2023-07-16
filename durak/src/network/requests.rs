use bevy::prelude::*;
use bevy_mod_reqwest::reqwest::Url;
use durak_lib::network::{CreateGameData, CreateGameResponce};

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
