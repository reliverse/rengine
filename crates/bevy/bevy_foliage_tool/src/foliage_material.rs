use bevy::{asset::embedded_asset, prelude::*};
use bevy::asset::load_internal_binary_asset;
use bevy::asset::VisitAssetDependencies;

use bevy::reflect::TypePath;
use bevy::render::render_resource::*;

use bevy::render::render_asset::RenderAssets;

use bevy::pbr::StandardMaterialFlags;
use bevy::pbr::StandardMaterialUniform;

use bevy::pbr::MaterialExtension;

use bevy::pbr::ExtendedMaterial;

pub type FoliageMaterialExtension = ExtendedMaterial<StandardMaterial, FoliageMaterial>;

pub fn foliage_material_plugin(app: &mut App) {
    embedded_asset!(app, "shaders/foliage.wgsl");



      embedded_asset!(app, "src/", "internal_assets/fog_noise.png" );
       


    app.add_plugins(MaterialPlugin::<FoliageMaterialExtension>::default());
}

#[derive(Asset, AsBindGroup, TypePath, Clone, Debug )]
pub struct FoliageMaterial {



    #[texture(20)]
    #[sampler(21)]
    pub fog_cloud_texture: Option<Handle<Image>>,

    
}
 

impl MaterialExtension for FoliageMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_foliage_tool/shaders/foliage.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "embedded://bevy_foliage_tool/shaders/foliage.wgsl".into()
    }



    fn vertex_shader() -> ShaderRef {
        "embedded://bevy_foliage_tool/shaders/foliage.wgsl".into()
    }

    fn deferred_vertex_shader() -> ShaderRef {
        "embedded://bevy_foliage_tool/shaders/foliage.wgsl".into()
    }

    //important for proper depth testing
    fn prepass_vertex_shader() -> ShaderRef {
        "embedded://bevy_foliage_tool/shaders/foliage.wgsl".into()
    }

     fn prepass_fragment_shader() -> ShaderRef {
        "embedded://bevy_foliage_tool/shaders/foliage.wgsl".into()
    } 
}
