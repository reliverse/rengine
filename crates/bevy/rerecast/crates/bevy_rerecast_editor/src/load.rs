use std::{fs::File, io};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures_lite::future},
};
use bevy_rerecast::Navmesh;
use rfd::FileHandle;
use thiserror::Error;

use crate::backend::{GlobalNavmeshSettings, NavmeshHandle};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ReadTasks>();
    app.add_systems(
        Update,
        (
            poll_load_task.run_if(resource_exists::<LoadTask>),
            poll_read_tasks,
        )
            .chain(),
    );
}

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct LoadTask(pub(crate) Task<Option<FileHandle>>);

fn poll_load_task(
    mut commands: Commands,
    mut task: ResMut<LoadTask>,
    mut read_tasks: ResMut<ReadTasks>,
) {
    let Some(file_handle) = future::block_on(future::poll_once(&mut task.0)) else {
        return;
    };
    commands.remove_resource::<LoadTask>();
    let Some(file) = file_handle else {
        // User canceled the save operation
        return;
    };

    let thread_pool = AsyncComputeTaskPool::get();

    let future = async move {
        let path = file.path();
        let mut file = File::open(path)?;
        let config = bincode::config::standard();
        let content: Navmesh = bincode::serde::decode_from_std_read(&mut file, config)?;
        Ok(content)
    };
    read_tasks.push(thread_pool.spawn(future));
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Failed to open file: {0}")]
    OpenFile(#[from] io::Error),
    #[error("Failed to decode navmesh: {0}")]
    ReadNavmesh(#[from] bincode::error::DecodeError),
}

#[derive(Resource, Default, Deref, DerefMut)]
struct ReadTasks(Vec<Task<Result<Navmesh, LoadError>>>);

fn poll_read_tasks(
    mut read_tasks: ResMut<ReadTasks>,
    mut commands: Commands,
    mut navmeshes: ResMut<Assets<Navmesh>>,
    mut settings: ResMut<GlobalNavmeshSettings>,
) {
    read_tasks.retain_mut(|task| {
        let Some(result) = future::block_on(future::poll_once(task)) else {
            return true;
        };
        match result {
            Ok(navmesh) => {
                settings.0 = navmesh.settings.clone();
                commands.insert_resource(NavmeshHandle(navmeshes.add(navmesh)));
                false
            }
            Err(err) => {
                error!("Failed to load navmesh: {}", err);
                false
            }
        }
    });
}
