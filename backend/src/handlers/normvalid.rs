mod quest;
pub use quest::*;

use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Query},
    http::request::Parts,
};
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use validator::Validate;

use crate::BackendError;

pub trait Normalize {
    fn normalize(&mut self);
}

/// Extractor wrapper that normalizes then validates the inner type.
///
/// Use with `NormValid<Json<T>>` or `NormValid<Query<T>>`.
pub struct NormValid<E>(pub E);

impl<E> Deref for NormValid<E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for NormValid<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> NormValid<E> {
    pub fn into_inner(self) -> E {
        self.0
    }
}

/// Trait for extractors that contain a normalizable and validatable inner type.
pub trait HasNormalizeValidate {
    type Inner: Normalize + Validate;
    fn get_inner_mut(&mut self) -> &mut Self::Inner;
}

impl<T: Normalize + Validate> HasNormalizeValidate for Json<T> {
    type Inner = T;
    fn get_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

impl<T: Normalize + Validate> HasNormalizeValidate for Query<T> {
    type Inner = T;
    fn get_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

impl<State, Extractor> FromRequest<State> for NormValid<Extractor>
where
    State: Send + Sync,
    Extractor: HasNormalizeValidate + FromRequest<State>,
    <Extractor as FromRequest<State>>::Rejection: Display,
{
    type Rejection = BackendError;

    async fn from_request(req: axum::extract::Request, state: &State) -> Result<Self, Self::Rejection> {
        let mut inner = Extractor::from_request(req, state)
            .await
            .map_err(|e| BackendError::BadRequest(e.to_string()))?;
        inner.get_inner_mut().normalize();
        inner.get_inner_mut().validate()?;
        Ok(NormValid(inner))
    }
}

impl<State, Extractor> FromRequestParts<State> for NormValid<Extractor>
where
    State: Send + Sync,
    Extractor: HasNormalizeValidate + FromRequestParts<State>,
    <Extractor as FromRequestParts<State>>::Rejection: Display,
{
    type Rejection = BackendError;

    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let mut inner = Extractor::from_request_parts(parts, state)
            .await
            .map_err(|e| BackendError::BadRequest(e.to_string()))?;
        inner.get_inner_mut().normalize();
        inner.get_inner_mut().validate()?;
        Ok(NormValid(inner))
    }
}
