use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampModel {
    pub id: u32,
    pub radius: f32,
    pub name: String,
    #[serde(rename = "hasCollision")]
    pub has_collision: String, // CSV has "No"/"Yes" as strings
    #[serde(rename = "breaksOnHit")]
    pub breaks_on_hit: String, // CSV has "No"/"Yes" as strings
    #[serde(rename = "visibleByTime")]
    pub visible_by_time: String,
    #[serde(rename = "hasAnimation")]
    pub has_animation: String, // CSV has "No"/"Yes" as strings
    #[serde(rename = "borderBoxLength")]
    pub border_box_length: f32,
    #[serde(rename = "borderBoxWidth")]
    pub border_box_width: f32,
    #[serde(rename = "borderBoxHeight")]
    pub border_box_height: f32,
    pub txd: String,
    pub ide: String,
    pub dff: String,
    pub tags: String,
}

#[derive(Debug)]
pub struct SampModelDatabase {
    models: Vec<SampModel>,
    name_to_id: HashMap<String, u32>,
    id_to_model: HashMap<u32, usize>, // Index into models vector
}

impl SampModelDatabase {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            name_to_id: HashMap::new(),
            id_to_model: HashMap::new(),
        }
    }

    pub fn load_from_csv<P: AsRef<Path>>(path: P) -> Result<Self, ModelError> {
        let mut database = Self::new();
        let file = File::open(path)?;
        let mut reader = csv::Reader::from_reader(file);

        for (index, result) in reader.deserialize().enumerate() {
            let record: SampModel = result?;
            database.models.push(record.clone());
            database.name_to_id.insert(record.name.clone(), record.id);
            database.id_to_model.insert(record.id, index);
        }

        Ok(database)
    }

    pub fn load_from_embedded_csv() -> Result<Self, ModelError> {
        let csv_content = include_str!("../../public/renderware/gta-sa/models.csv");
        let mut database = Self::new();
        let mut reader = csv::Reader::from_reader(csv_content.as_bytes());

        for (index, result) in reader.deserialize().enumerate() {
            let record: SampModel = result?;
            database.models.push(record.clone());
            database.name_to_id.insert(record.name.clone(), record.id);
            database.id_to_model.insert(record.id, index);
        }

        Ok(database)
    }

    pub fn get_model_by_id(&self, id: u32) -> Option<&SampModel> {
        self.id_to_model
            .get(&id)
            .and_then(|&index| self.models.get(index))
    }

    pub fn get_model_by_name(&self, name: &str) -> Option<&SampModel> {
        self.name_to_id
            .get(name)
            .and_then(|&id| self.get_model_by_id(id))
    }

    pub fn find_models_by_name_prefix(&self, prefix: &str, limit: usize) -> Vec<&SampModel> {
        let prefix_lower = prefix.to_lowercase();
        self.models
            .iter()
            .filter(|model| model.name.to_lowercase().starts_with(&prefix_lower))
            .take(limit)
            .collect()
    }

    pub fn find_models_by_name_contains(&self, substring: &str, limit: usize) -> Vec<&SampModel> {
        let substring_lower = substring.to_lowercase();
        self.models
            .iter()
            .filter(|model| model.name.to_lowercase().contains(&substring_lower))
            .take(limit)
            .collect()
    }

    pub fn get_all_models(&self) -> &[SampModel] {
        &self.models
    }

    pub fn len(&self) -> usize {
        self.models.len()
    }

    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }
}

impl Default for SampModelDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_embedded_csv() {
        let database = SampModelDatabase::load_from_embedded_csv().unwrap();
        assert!(!database.is_empty());
        assert!(database.len() > 1000); // Should have many models

        // Test finding a known model
        let model = database.get_model_by_id(1000);
        assert!(model.is_some());
        let model = model.unwrap();
        assert_eq!(model.name, "spl_b_mar_m");
        assert_eq!(model.id, 1000);
    }

    #[test]
    fn test_name_lookup() {
        let database = SampModelDatabase::load_from_embedded_csv().unwrap();

        // Test exact name match
        let model = database.get_model_by_name("spl_b_mar_m");
        assert!(model.is_some());
        assert_eq!(model.unwrap().id, 1000);

        // Test case insensitive prefix search
        let models = database.find_models_by_name_prefix("spl", 5);
        assert!(!models.is_empty());
        assert!(models.iter().all(|m| m.name.to_lowercase().starts_with("spl")));
    }
}