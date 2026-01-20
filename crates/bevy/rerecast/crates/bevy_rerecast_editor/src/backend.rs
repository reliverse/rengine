use bevy::prelude::*;
use bevy_rerecast::{
    debug::{DetailNavmeshGizmo, PolygonNavmeshGizmo},
    prelude::*,
    rerecast::TriMesh,
};

pub(super) fn plugin(app: &mut App) {
    app.set_navmesh_backend(editor_backend);
    app.add_systems(
        Update,
        insert_gizmos.run_if(resource_exists_and_changed::<NavmeshHandle>),
    );
    app.add_observer(build_navmesh);
    app.init_resource::<GlobalNavmeshSettings>()
        .init_resource::<NavmeshHandle>()
        .init_resource::<NavmeshObstacles>();
}

fn editor_backend(_: In<NavmeshSettings>, obstacles: Res<NavmeshObstacles>) -> TriMesh {
    obstacles.0.clone()
}

#[derive(Resource, Deref, DerefMut, Default)]
pub(crate) struct NavmeshObstacles(pub(crate) TriMesh);

#[derive(Event)]
pub(crate) struct BuildNavmesh;

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct GlobalNavmeshSettings(pub(crate) NavmeshSettings);

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct NavmeshHandle(pub(crate) Handle<Navmesh>);

fn build_navmesh(
    _trigger: On<BuildNavmesh>,
    mut commands: Commands,
    config: Res<GlobalNavmeshSettings>,
    mut navmesh_generator: NavmeshGenerator,
) {
    let handle = navmesh_generator.generate(config.0.clone());
    commands.insert_resource(NavmeshHandle(handle));
}

fn insert_gizmos(mut commands: Commands, navmesh: Res<NavmeshHandle>) {
    commands.spawn(PolygonNavmeshGizmo(navmesh.id()));
    commands.spawn(DetailNavmeshGizmo(navmesh.id()));
}
