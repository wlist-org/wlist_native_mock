use crate::api::common::exceptions::UniverseError;
use crate::api::common::o2o;
use crate::api::tasks::data::{FTaskListInformation, FTaskStateFilter, FTasksFilter};
use crate::api::tasks::tasks::{FRefreshTask, FTaskBase, FTaskState};
use crate::api::tasks::FTask;

/// Select task by id.
pub async fn tasks_select(id: i64) -> Result<Option<FTask>, UniverseError> {
    wlist_native::tasks::manager::tasks_select(id).await.map(o2o::map_option).map_err(Into::into)
}

/// Select task parents by id.
pub async fn tasks_select_parents(id: i64) -> Result<Vec<FTask>, UniverseError> {
    wlist_native::tasks::manager::tasks_select_parents(id).await.map(o2o::map_vec).map_err(Into::into)
}

/// Select task children by id.
pub async fn tasks_select_children(id: i64, filter: FTasksFilter, state_filter: FTaskStateFilter) -> Result<Vec<FTask>, UniverseError> {
    wlist_native::tasks::manager::tasks_select_children(id, filter.into(), state_filter.into()).await.map(o2o::map_vec).map_err(Into::into)
}

/// Select task list.
pub async fn tasks_select_list(filter: FTasksFilter, state_filter: FTaskStateFilter, offset: u64, limit: usize) -> Result<FTaskListInformation, UniverseError> {
    wlist_native::tasks::manager::tasks_select_list(filter.into(), state_filter.into(), offset, limit).await.map(Into::into).map_err(Into::into)
}

/// Insert tasks.
pub async fn tasks_insert(tasks: Vec<FTask>) -> Result<Vec<FTask>, UniverseError> {
    wlist_native::tasks::manager::tasks_insert(o2o::map_vec(tasks)).await.map(o2o::map_vec).map_err(Into::into)
}

/// Update tasks state.
pub async fn tasks_update(tasks: Vec<(i64, FTaskState)>) -> Result<(), UniverseError> {
    wlist_native::tasks::manager::tasks_update(tasks.into_iter().map(|(a, b)| (a, b.into())).collect()).await.map_err(Into::into)
}

/// Delete tasks.
pub async fn tasks_delete(tasks: Vec<i64>) -> Result<(), UniverseError> {
    wlist_native::tasks::manager::tasks_delete(o2o::map_vec(tasks)).await.map_err(Into::into)
}

/// Delete all tasks.
pub async fn tasks_delete_all(filter: FTasksFilter, state_filter: FTaskStateFilter) -> Result<(), UniverseError> {
    wlist_native::tasks::manager::tasks_delete_all(filter.into(), state_filter.into()).await.map_err(Into::into)
}


/// Select all same refresh tasks.
pub async fn tasks_select_refresh(storage: i64, directory: i64) -> Result<Vec<FTaskBase>, UniverseError> {
    wlist_native::tasks::manager::tasks_select_refresh(storage, directory).await.map(o2o::map_vec).map_err(Into::into)
}