use itertools::Itertools;
use validator::ValidationError;

use crate::handlers::{
    common,
    normvalid::Normalize,
    quest::{CreateQuestRequest, SearchQuestParams, UpdateQuestRequest},
};

impl Normalize for CreateQuestRequest {
    fn normalize(&mut self) {
        self.title = self.title.trim().to_string();
    }
}
impl Normalize for UpdateQuestRequest {
    fn normalize(&mut self) {
        if let Some(title) = self.title.as_mut() {
            *title = common::trim_whitespace(title);
        }
        if let Some(techs) = self.techs.as_mut() {
            for tech in techs.iter_mut() {
                *tech = common::trim_whitespace(tech);
            }
        }
    }
}

impl Normalize for SearchQuestParams {
    fn normalize(&mut self) {
        if let Some(query) = self.query.as_mut() {
            *query = common::trim_whitespace(query);
        }
        if let Some(techs) = self.techs.as_mut() {
            for tech in techs.iter_mut() {
                *tech = common::trim_whitespace(tech);
            }
        }
    }
}

pub fn techs(techs: &Vec<String>) -> Result<(), ValidationError> {
    if techs.len() > 10 {
        let err = ValidationError::new("invalid_tech_count");
        return Err(err.with_message("There must be no more than 10 Techs".into()));
    }
    if !techs.iter().all_unique() {
        let err = ValidationError::new("techs_not_all_unique");
        return Err(err.with_message("Techs must be all unique".into()));
    }
    for tech in techs {
        if tech.len() > 15 {
            let err = ValidationError::new("tech_invalid_length");
            return Err(err.with_message("Tech must be no longer than 15 characters".into()));
        }
        if !tech.chars().all(|c| c.is_alphanumeric() || c == '-') {
            let err = ValidationError::new("tech_invalid_character");
            return Err(err.with_message("Tech must not contain non-alphanumeric and non-hyphen (-) characters".into()));
        }
    }
    Ok(())
}
