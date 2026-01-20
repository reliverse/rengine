 
use bevy_render::view::RetainedViewEntity;
use  bevy_image::BevyDefault;
use bevy_app::{App, Plugin};
use bevy_asset::{load_internal_asset, Asset, AssetId, Handle};
use bevy_core_pipeline::core_3d::{Transparent3d, CORE_3D_DEPTH_FORMAT};
use bevy_core_pipeline::prepass::{
    DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass,
};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_ecs::query::ROQueryItem;
use bevy_ecs::system::lifetimeless::{Read, SRes};
use bevy_ecs::system::SystemParamItem;
use bevy_image::BevyDefault as _;
use bevy_pbr::{MeshPipeline, MeshPipelineKey, SetMeshViewBindGroup};
use bevy_reflect::{Reflect, TypePath};
use bevy_render::extract_component::ExtractComponent;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_render::prelude::*;
use bevy_asset::RenderAssetUsages;
use bevy_render::render_asset::{
    prepare_assets, PrepareAssetError, RenderAsset, RenderAssetPlugin,
    RenderAssets,
};
use bevy_render::render_phase::{
    AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
    RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
};
use bevy_render::render_resource::{
    BlendState, Buffer, BufferInitDescriptor, BufferUsages, ColorTargetState, ColorWrites,
    CompareFunction, DepthBiasState, DepthStencilState, FragmentState, IndexFormat,
    MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor,
    SpecializedRenderPipeline, SpecializedRenderPipelines, StencilState, TextureFormat,
    VertexAttribute, VertexFormat, VertexState, VertexStepMode,
};
use bevy_render::render_resource::RawVertexBufferLayout;
use bevy_render::renderer::RenderDevice;
use bevy_render::view::{ExtractedView, ViewTarget};
use bevy_render::{Extract, Render, RenderApp};

use bevy::platform::collections::{hash_map::HashMap, hash_set::HashSet};
use bevy::prelude::Shader;

use bytemuck::cast_slice;
use uuid::Uuid;

use crate::GizmoCamera;

const GIZMO_SHADER_HANDLE: Handle<Shader> = Handle::from(AssetId::<Shader>::invalid());

pub(crate) struct TransformGizmoRenderPlugin;

impl Plugin for TransformGizmoRenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, GIZMO_SHADER_HANDLE, "gizmo.wgsl", Shader::from_wgsl);

        app.init_resource::<DrawDataHandles>()
            .add_plugins(RenderAssetPlugin::<GizmoBuffers>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_command::<Transparent3d, DrawGizmo>()
            .init_resource::<SpecializedRenderPipelines<TransformGizmoPipeline>>()
            .add_systems(
                Render,
                queue_transform_gizmos
                    .after(prepare_assets::<GizmoBuffers>),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_systems(ExtractSchedule, extract_gizmo_data)
            .init_resource::<TransformGizmoPipeline>();
    }
}

#[derive(Resource, Default)]
pub(crate) struct DrawDataHandles {
    pub(crate) handles: HashMap<Uuid, GizmoDrawDataHandle>,
}

#[derive(
    Component, Default, Clone, Debug, Deref, DerefMut, Reflect, PartialEq, Eq, Hash, ExtractComponent,
)]
#[reflect(Component)]
pub(crate) struct GizmoDrawDataHandle(pub(crate) Handle<GizmoDrawData>);

impl From<Handle<GizmoDrawData>> for GizmoDrawDataHandle {
    fn from(handle: Handle<GizmoDrawData>) -> Self {
        Self(handle)
    }
}

impl From<GizmoDrawDataHandle> for AssetId<GizmoDrawData> {
    fn from(handle: GizmoDrawDataHandle) -> Self {
        handle.0.id()
    }
}
impl From<&GizmoDrawDataHandle> for AssetId<GizmoDrawData> {
    fn from(handle: &GizmoDrawDataHandle) -> Self {
        handle.0.id()
    }
}

fn extract_gizmo_data(mut commands: Commands, handles: Extract<Res<DrawDataHandles>>) {
    let handle_weak_refs = handles
        .handles
        .values()
        .map(|handle| handle.clone())
        .collect::<HashSet<_>>();

    for handle in handle_weak_refs {
        commands.spawn(GizmoDrawDataHandle(handle));
    }
}

#[derive(Asset, Debug, Default, Clone, TypePath)]
pub(crate) struct GizmoDrawData(pub(crate) transform_gizmo::GizmoDrawData);

#[derive(Debug, Clone)]
pub(crate) struct GizmoBuffers {
    position_buffer: Buffer,
    index_buffer: Buffer,
    color_buffer: Buffer,
    index_count: u32,
}

impl RenderAsset for GizmoBuffers {
    type SourceAsset = GizmoDrawData;
    type Param = SRes<RenderDevice>;

    fn asset_usage(_source_asset: &Self::SourceAsset) -> RenderAssetUsages {
        RenderAssetUsages::all()
    }

    fn prepare_asset(
        source_asset: Self::SourceAsset,
        _asset_id: AssetId< GizmoDrawData >,
        render_device: &mut SystemParamItem<Self::Param>,
        _prepared_assets: Option<&Self>,
    ) -> Result<Self, PrepareAssetError<Self::SourceAsset>> {
        let position_buffer_data = cast_slice(&source_asset.0.vertices);
        let position_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            usage: BufferUsages::VERTEX,
            label: Some("TransformGizmo Position Buffer"),
            contents: position_buffer_data,
        });

        let index_buffer_data = cast_slice(&source_asset.0.indices);
        let index_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            usage: BufferUsages::INDEX,
            label: Some("TransformGizmo Index Buffer"),
            contents: index_buffer_data,
        });

        let color_buffer_data = cast_slice(&source_asset.0.colors);
        let color_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            usage: BufferUsages::VERTEX,
            label: Some("TransformGizmo Color Buffer"),
            contents: color_buffer_data,
        });

        Ok(Self {
            index_buffer,
            position_buffer,
            color_buffer,
            index_count: source_asset.0.indices.len() as u32,
        })
    }
}

struct DrawTransformGizmo;

impl<P: PhaseItem> RenderCommand<P> for DrawTransformGizmo {
    type ViewQuery = ();
    type ItemQuery = Read<GizmoDrawDataHandle>;
    type Param = SRes<RenderAssets<GizmoBuffers>>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: ROQueryItem<'w, 'w, Self::ViewQuery>,
        handle: Option<ROQueryItem<'w, 'w, Self::ItemQuery>>,
        gizmos: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(handle) = handle else {
            return RenderCommandResult::Failure("No GizmoDrawDataHandle component found");
        };

        let Some(gizmo) = gizmos.into_inner().get(handle) else {
            return RenderCommandResult::Failure("No GizmoDrawDataHandle inner found");
        };

        //THIS FIXES MY CRASHING BUG FOR SOME REASON - PLEASE INVESTIGATE 
       if gizmo.index_buffer.size() == 0 {
        return RenderCommandResult::Failure("gizmo.index_buffer empty");
       }

      //  println!(" gizmo.index_buffer.size {}",  gizmo.index_buffer.size() );

        pass.set_index_buffer(gizmo.index_buffer.slice(..), IndexFormat::Uint32);
        pass.set_vertex_buffer(0, gizmo.position_buffer.slice(..));
        pass.set_vertex_buffer(1, gizmo.color_buffer.slice(..));

        pass.draw_indexed(0..gizmo.index_count, 0, 0..1);

        RenderCommandResult::Success
    }
}

#[derive(Clone, Resource)]
struct TransformGizmoPipeline {
    mesh_pipeline: MeshPipeline,
}

impl FromWorld for TransformGizmoPipeline {
    fn from_world(render_world: &mut World) -> Self {
        Self {
            mesh_pipeline: render_world.resource::<MeshPipeline>().clone(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct TransformGizmoPipelineKey {
    view_key: MeshPipelineKey,
    perspective: bool,
}

impl SpecializedRenderPipeline for TransformGizmoPipeline {
    type Key = TransformGizmoPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut shader_defs = vec![
            // TODO: When is this flag actually used?
            // #[cfg(feature = "webgl")]
            // "SIXTEEN_BYTE_ALIGNMENT".into(),
        ];

        if key.perspective {
            shader_defs.push("PERSPECTIVE".into());
        }

        let format = if key.view_key.contains(MeshPipelineKey::HDR) {
            ViewTarget::TEXTURE_FORMAT_HDR
        } else {
            TextureFormat::bevy_default()
        };

        let view_layout = self
            .mesh_pipeline
            .get_view_layout(key.view_key.into())
            .clone();

        RenderPipelineDescriptor {
            label: Some("TransformGizmo Pipeline".into()),
            zero_initialize_workgroup_memory: true, // ?
            vertex: VertexState {
                shader: GIZMO_SHADER_HANDLE,
                entry_point: Some("vertex".into()),
                shader_defs: shader_defs.clone(),
                buffers: vec![
                    RawVertexBufferLayout {
                        array_stride: VertexFormat::Float32x2.size(),
                        step_mode: VertexStepMode::Vertex,
                        attributes: &[VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        }],
                    },
                    RawVertexBufferLayout {
                        array_stride: VertexFormat::Float32x4.size(),
                        step_mode: VertexStepMode::Vertex,
                        attributes: &[VertexAttribute {
                            format: VertexFormat::Float32x4,
                            offset: 0,
                            shader_location: 1,
                        }],
                    },
                ],
            },
            fragment: Some(FragmentState {
                shader: GIZMO_SHADER_HANDLE,
                shader_defs,
                entry_point: Some("fragment".into()),
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: vec![view_layout.main_layout],
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..PrimitiveState::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: CORE_3D_DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Always,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: key.view_key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            push_constant_ranges: vec![],
        }
    }
}

type DrawGizmo = (SetItemPipeline, SetMeshViewBindGroup<0>, DrawTransformGizmo);

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn queue_transform_gizmos(
    draw_functions: Res<DrawFunctions<Transparent3d>>,
    pipeline: Res<TransformGizmoPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<TransformGizmoPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    msaa_q: Query<Option<&Msaa>, With<GizmoCamera>>,
    transform_gizmos: Query<(Entity, &GizmoDrawDataHandle)>,
    transform_gizmo_assets: Res<RenderAssets<GizmoBuffers>>,
    mut views: Query<(
        Entity,
        &ExtractedView,
        Option<&Msaa>,
        (
            Has<NormalPrepass>,
            Has<DepthPrepass>,
            Has<MotionVectorPrepass>,
            Has<DeferredPrepass>,
        ),
    )>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
) {
    let draw_function = draw_functions.read().get_id::<DrawGizmo>().unwrap();
    let camera_msaa = msaa_q.single().ok().flatten();
    for (
        view_entity,
        view,
        entity_msaa,
        (normal_prepass, depth_prepass, motion_vector_prepass, deferred_prepass),
    ) in &mut views
    {       

         let retained_view_entity =  & view.retained_view_entity ; 

        let Some(transparent_phase) = transparent_render_phases.get_mut( retained_view_entity ) else {
           
            continue;
        };


        // entity_msaa > camera_msaa > default
        let msaa_sample_count = entity_msaa.map_or(
            camera_msaa.unwrap_or(&Msaa::default()).samples(),
            |msaa| msaa.samples(),
        );

        let mut view_key = MeshPipelineKey::from_msaa_samples(msaa_sample_count)
            | MeshPipelineKey::from_hdr(view.hdr);

        if normal_prepass {
            view_key |= MeshPipelineKey::NORMAL_PREPASS;
        }

        if depth_prepass {
            view_key |= MeshPipelineKey::DEPTH_PREPASS;
        }

        if motion_vector_prepass {
            view_key |= MeshPipelineKey::MOTION_VECTOR_PREPASS;
        }

        if deferred_prepass {
            view_key |= MeshPipelineKey::DEFERRED_PREPASS;
        }

        for (entity, handle) in &transform_gizmos {
            let Some(_) = transform_gizmo_assets.get(handle.0.id()) else {
                continue;
            };

            let pipeline = pipelines.specialize(
                &pipeline_cache,
                &pipeline,
                TransformGizmoPipelineKey {
                    view_key,
                    perspective: true,
                },
            );

            transparent_phase.add(Transparent3d {
                entity: (entity, Some(view_entity)),
                draw_function,
                pipeline,
                distance: 0.,
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::None,
                indexed: false, // ? 
            });
        }
    }
}
