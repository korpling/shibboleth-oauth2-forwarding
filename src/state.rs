use std::sync::Mutex;

use crate::errors::StartupError;
use crate::jwt::JWTIssuer;
use crate::settings::Settings;
use oxide_auth::frontends::simple::endpoint::{Generic, Vacant};
use oxide_auth::primitives::prelude::*;
use oxide_auth::primitives::registrar::RegisteredUrl;

pub struct State {
    registrar: Mutex<ClientMap>,
    authorizer: Mutex<AuthMap<RandomGenerator>>,
    issuer: Mutex<JWTIssuer>,
    pub settings: Settings,
}

impl State {
    pub fn endpoint(&self) -> Generic<impl Registrar + '_, impl Authorizer + '_, impl Issuer + '_> {
        Generic {
            registrar: self.registrar.lock().unwrap(),
            authorizer: self.authorizer.lock().unwrap(),
            issuer: self.issuer.lock().unwrap(),
            solicitor: Vacant,
            scopes: Vacant,
            response: Vacant,
        }
    }

    pub fn new(settings: &Settings) -> Result<Self, StartupError> {
        let additional_redirect_uris: Vec<_> = settings
            .client
            .additional_redirect_uris
            .iter()
            .filter_map(|u| u.parse::<url::Url>().ok())
            .map(|u| RegisteredUrl::Semantic(u))
            .collect();
        let client = if let Some(client_secret) = &settings.client.secret {
            Client::confidential(
                &settings.client.id,
                settings.client.redirect_uri.parse::<url::Url>()?.into(),
                "default-scope".parse()?,
                client_secret.as_bytes(),
            )
            .with_additional_redirect_uris(additional_redirect_uris)
        } else {
            Client::public(
                &settings.client.id,
                settings.client.redirect_uri.parse::<url::Url>()?.into(),
                "default-scope".parse()?,
            )
            .with_additional_redirect_uris(additional_redirect_uris)
        };
        let registrar = vec![client].into_iter().collect();
        let authorizer = AuthMap::new(RandomGenerator::new(16));
        let issuer = JWTIssuer::new(settings.clone());
        let state = State {
            registrar: Mutex::new(registrar),
            issuer: Mutex::new(issuer),
            authorizer: Mutex::new(authorizer),
            settings: settings.clone(),
        };
        Ok(state)
    }
}
