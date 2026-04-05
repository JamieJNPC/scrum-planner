use crate::app::entities::{Feature, Member, Role, Story};
use crate::app::model::pi::PI;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MainAppData {
    pub members: Vec<Member>,
    pub roles: Vec<Role>,
    pub features: Vec<Feature>,
    pub pis: Vec<PI>,
}

impl MainAppData {
    pub fn get_feature(&self, name: &String) -> Option<&Feature> {
        for f in self.features.iter() {
            if f.name.eq(name.as_str()) {
                return Some(f);
            }
        }
        return None;
    }

    pub fn get_feature_mut(&mut self, name: &String) -> Option<&mut Feature> {
        for f in self.features.iter_mut() {
            if f.name.eq(name.as_str()) {
                return Some(f);
            }
        }
        return None;
    }

    pub fn add_story_to_objective(&mut self, feature_name: &String, objective_name: &String, story: Story) {
        for feature in self.features.iter_mut() {
            if feature_name.eq(&feature.get_title()) {
                feature.add_story_to_objective(objective_name, story);
                return;
            }
        }
    }
}