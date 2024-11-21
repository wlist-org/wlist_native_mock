pub mod data;
pub mod tasks;
pub mod manager;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum Task {
    Refresh(tasks::RefreshTask),

}