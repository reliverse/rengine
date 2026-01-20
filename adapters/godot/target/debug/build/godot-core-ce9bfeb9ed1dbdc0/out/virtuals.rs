#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_imports)]
pub mod AesContext {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AStar2D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const filter_neighbor: (&str, u32) = ("_filter_neighbor", 2522259332u32);
    pub type Sig_filter_neighbor = (i64, i64,);
    pub const estimate_cost: (&str, u32) = ("_estimate_cost", 3085491603u32);
    pub type Sig_estimate_cost = (i64, i64,);
    pub const compute_cost: (&str, u32) = ("_compute_cost", 3085491603u32);
    pub type Sig_compute_cost = (i64, i64,);
    
}
pub mod AStar3D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const filter_neighbor: (&str, u32) = ("_filter_neighbor", 2522259332u32);
    pub type Sig_filter_neighbor = (i64, i64,);
    pub const estimate_cost: (&str, u32) = ("_estimate_cost", 3085491603u32);
    pub type Sig_estimate_cost = (i64, i64,);
    pub const compute_cost: (&str, u32) = ("_compute_cost", 3085491603u32);
    pub type Sig_compute_cost = (i64, i64,);
    
}
pub mod AStarGrid2D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const estimate_cost: (&str, u32) = ("_estimate_cost", 2153177966u32);
    pub type Sig_estimate_cost = (Vector2i, Vector2i,);
    pub const compute_cost: (&str, u32) = ("_compute_cost", 2153177966u32);
    pub type Sig_compute_cost = (Vector2i, Vector2i,);
    
}
pub mod AcceptDialog {
    pub use super::Window::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AimModifier3D {
    pub use super::BoneConstraint3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimatableBody2D {
    pub use super::StaticBody2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimatableBody3D {
    pub use super::StaticBody3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimatedSprite2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimatedSprite3D {
    pub use super::SpriteBase3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimatedTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Animation {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationLibrary {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationMixer {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const post_process_key_value: (&str, u32) = ("_post_process_key_value", 2716908335u32);
    pub type Sig_post_process_key_value = (Option < Gd < crate::classes::Animation > >, i32, Variant, u64, i32,);
    
}
pub mod AnimationNode {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_child_nodes: (&str, u32) = ("_get_child_nodes", 3102165223u32);
    pub type Sig_get_child_nodes = ();
    pub const get_parameter_list: (&str, u32) = ("_get_parameter_list", 3995934104u32);
    pub type Sig_get_parameter_list = ();
    pub const get_child_by_name: (&str, u32) = ("_get_child_by_name", 625644256u32);
    pub type Sig_get_child_by_name = (StringName,);
    pub const get_parameter_default_value: (&str, u32) = ("_get_parameter_default_value", 2760726917u32);
    pub type Sig_get_parameter_default_value = (StringName,);
    pub const is_parameter_read_only: (&str, u32) = ("_is_parameter_read_only", 2619796661u32);
    pub type Sig_is_parameter_read_only = (StringName,);
    pub const process: (&str, u32) = ("_process", 2139827523u32);
    pub type Sig_process = (f64, bool, bool, bool,);
    pub const get_caption: (&str, u32) = ("_get_caption", 201670096u32);
    pub type Sig_get_caption = ();
    pub const has_filter: (&str, u32) = ("_has_filter", 36873697u32);
    pub type Sig_has_filter = ();
    
}
pub mod AnimationNodeAdd2 {
    pub use super::AnimationNodeSync::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeAdd3 {
    pub use super::AnimationNodeSync::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeAnimation {
    pub use super::AnimationRootNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeBlend2 {
    pub use super::AnimationNodeSync::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeBlend3 {
    pub use super::AnimationNodeSync::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeBlendSpace1D {
    pub use super::AnimationRootNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeBlendSpace2D {
    pub use super::AnimationRootNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeBlendTree {
    pub use super::AnimationRootNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeExtension {
    pub use super::AnimationNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const process_animation_node: (&str, u32) = ("_process_animation_node", 912931771u32);
    pub type Sig_process_animation_node = (PackedFloat64Array, bool,);
    
}
pub mod AnimationNodeOneShot {
    pub use super::AnimationNodeSync::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeOutput {
    pub use super::AnimationNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeStateMachine {
    pub use super::AnimationRootNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeStateMachinePlayback {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeStateMachineTransition {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeSub2 {
    pub use super::AnimationNodeSync::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeSync {
    pub use super::AnimationNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeTimeScale {
    pub use super::AnimationNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeTimeSeek {
    pub use super::AnimationNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationNodeTransition {
    pub use super::AnimationNodeSync::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationPlayer {
    pub use super::AnimationMixer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationRootNode {
    pub use super::AnimationNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AnimationTree {
    pub use super::AnimationMixer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Area2D {
    pub use super::CollisionObject2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Area3D {
    pub use super::CollisionObject3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ArrayMesh {
    pub use super::Mesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ArrayOccluder3D {
    pub use super::Occluder3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AspectRatioContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AtlasTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioBusLayout {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffect {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const instantiate: (&str, u32) = ("_instantiate", 1659796816u32);
    pub type Sig_instantiate = ();
    
}
pub mod AudioEffectAmplify {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectBandLimitFilter {
    pub use super::AudioEffectFilter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectBandPassFilter {
    pub use super::AudioEffectFilter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectCapture {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectChorus {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectCompressor {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectDelay {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectDistortion {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectEq {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectEq10 {
    pub use super::AudioEffectEq::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectEq21 {
    pub use super::AudioEffectEq::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectEq6 {
    pub use super::AudioEffectEq::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectFilter {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectHardLimiter {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectHighPassFilter {
    pub use super::AudioEffectFilter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectHighShelfFilter {
    pub use super::AudioEffectFilter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectInstance {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const process_rawptr: (&str, u32) = ("_process", 1649997291u32);
    pub type Sig_process_rawptr = (* const c_void, * mut AudioFrame, i32,);
    pub const process_silence: (&str, u32) = ("_process_silence", 36873697u32);
    pub type Sig_process_silence = ();
    
}
pub mod AudioEffectLimiter {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectLowPassFilter {
    pub use super::AudioEffectFilter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectLowShelfFilter {
    pub use super::AudioEffectFilter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectNotchFilter {
    pub use super::AudioEffectFilter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectPanner {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectPhaser {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectPitchShift {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectRecord {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectReverb {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectSpectrumAnalyzer {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectSpectrumAnalyzerInstance {
    pub use super::AudioEffectInstance::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioEffectStereoEnhance {
    pub use super::AudioEffect::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioListener2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioListener3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioServer {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStream {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const instantiate_playback: (&str, u32) = ("_instantiate_playback", 3093715447u32);
    pub type Sig_instantiate_playback = ();
    pub const get_stream_name: (&str, u32) = ("_get_stream_name", 201670096u32);
    pub type Sig_get_stream_name = ();
    pub const get_length: (&str, u32) = ("_get_length", 1740695150u32);
    pub type Sig_get_length = ();
    pub const is_monophonic: (&str, u32) = ("_is_monophonic", 36873697u32);
    pub type Sig_is_monophonic = ();
    pub const get_bpm: (&str, u32) = ("_get_bpm", 1740695150u32);
    pub type Sig_get_bpm = ();
    pub const get_beat_count: (&str, u32) = ("_get_beat_count", 3905245786u32);
    pub type Sig_get_beat_count = ();
    pub const get_tags: (&str, u32) = ("_get_tags", 3102165223u32);
    pub type Sig_get_tags = ();
    pub const get_parameter_list: (&str, u32) = ("_get_parameter_list", 3995934104u32);
    pub type Sig_get_parameter_list = ();
    pub const has_loop: (&str, u32) = ("_has_loop", 36873697u32);
    pub type Sig_has_loop = ();
    pub const get_bar_beats: (&str, u32) = ("_get_bar_beats", 3905245786u32);
    pub type Sig_get_bar_beats = ();
    
}
pub mod AudioStreamGenerator {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamGeneratorPlayback {
    pub use super::AudioStreamPlaybackResampled::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamInteractive {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamMp3 {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamMicrophone {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamOggVorbis {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlayback {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const start: (&str, u32) = ("_start", 373806689u32);
    pub type Sig_start = (f64,);
    pub const stop: (&str, u32) = ("_stop", 3218959716u32);
    pub type Sig_stop = ();
    pub const is_playing: (&str, u32) = ("_is_playing", 36873697u32);
    pub type Sig_is_playing = ();
    pub const get_loop_count: (&str, u32) = ("_get_loop_count", 3905245786u32);
    pub type Sig_get_loop_count = ();
    pub const get_playback_position: (&str, u32) = ("_get_playback_position", 1740695150u32);
    pub type Sig_get_playback_position = ();
    pub const seek: (&str, u32) = ("_seek", 373806689u32);
    pub type Sig_seek = (f64,);
    pub const mix_rawptr: (&str, u32) = ("_mix", 925936155u32);
    pub type Sig_mix_rawptr = (* mut AudioFrame, f32, i32,);
    pub const tag_used_streams: (&str, u32) = ("_tag_used_streams", 3218959716u32);
    pub type Sig_tag_used_streams = ();
    pub const set_parameter: (&str, u32) = ("_set_parameter", 3776071444u32);
    pub type Sig_set_parameter = (StringName, Variant,);
    pub const get_parameter: (&str, u32) = ("_get_parameter", 2760726917u32);
    pub type Sig_get_parameter = (StringName,);
    
}
pub mod AudioStreamPlaybackInteractive {
    pub use super::AudioStreamPlayback::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlaybackOggVorbis {
    pub use super::AudioStreamPlaybackResampled::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlaybackPlaylist {
    pub use super::AudioStreamPlayback::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlaybackPolyphonic {
    pub use super::AudioStreamPlayback::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlaybackResampled {
    pub use super::AudioStreamPlayback::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const mix_resampled_rawptr: (&str, u32) = ("_mix_resampled", 50157827u32);
    pub type Sig_mix_resampled_rawptr = (* mut AudioFrame, i32,);
    pub const get_stream_sampling_rate: (&str, u32) = ("_get_stream_sampling_rate", 1740695150u32);
    pub type Sig_get_stream_sampling_rate = ();
    
}
pub mod AudioStreamPlaybackSynchronized {
    pub use super::AudioStreamPlayback::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlayer {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlayer2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlayer3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPlaylist {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamPolyphonic {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamRandomizer {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamSynchronized {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod AudioStreamWav {
    pub use super::AudioStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BackBufferCopy {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BaseButton {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const pressed: (&str, u32) = ("_pressed", 3218959716u32);
    pub type Sig_pressed = ();
    pub const toggled: (&str, u32) = ("_toggled", 2586408642u32);
    pub type Sig_toggled = (bool,);
    
}
pub mod BaseMaterial3D {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BitMap {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Bone2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BoneAttachment3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BoneConstraint3D {
    pub use super::SkeletonModifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BoneMap {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BoxContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BoxMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BoxOccluder3D {
    pub use super::Occluder3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod BoxShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Button {
    pub use super::BaseButton::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ButtonGroup {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CpuParticles2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CpuParticles3D {
    pub use super::GeometryInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgBox3D {
    pub use super::CsgPrimitive3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgCombiner3D {
    pub use super::CsgShape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgCylinder3D {
    pub use super::CsgPrimitive3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgMesh3D {
    pub use super::CsgPrimitive3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgPolygon3D {
    pub use super::CsgPrimitive3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgPrimitive3D {
    pub use super::CsgShape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgShape3D {
    pub use super::GeometryInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgSphere3D {
    pub use super::CsgPrimitive3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CsgTorus3D {
    pub use super::CsgPrimitive3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CallbackTweener {
    pub use super::Tweener::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Camera2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Camera3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CameraAttributes {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CameraAttributesPhysical {
    pub use super::CameraAttributes::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CameraAttributesPractical {
    pub use super::CameraAttributes::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CameraFeed {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const activate_feed: (&str, u32) = ("_activate_feed", 2240911060u32);
    pub type Sig_activate_feed = ();
    pub const deactivate_feed: (&str, u32) = ("_deactivate_feed", 3218959716u32);
    pub type Sig_deactivate_feed = ();
    
}
pub mod CameraServer {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CameraTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CanvasGroup {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CanvasItem {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const draw: (&str, u32) = ("_draw", 3218959716u32);
    pub type Sig_draw = ();
    
}
pub mod CanvasItemMaterial {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CanvasLayer {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CanvasModulate {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CanvasTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CapsuleMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CapsuleShape2D {
    pub use super::Shape2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CapsuleShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CenterContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CharFxTransform {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CharacterBody2D {
    pub use super::PhysicsBody2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CharacterBody3D {
    pub use super::PhysicsBody3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CheckBox {
    pub use super::Button::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CheckButton {
    pub use super::Button::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CircleShape2D {
    pub use super::Shape2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ClassDb {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CodeEdit {
    pub use super::TextEdit::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const confirm_code_completion: (&str, u32) = ("_confirm_code_completion", 2586408642u32);
    pub type Sig_confirm_code_completion = (bool,);
    pub const request_code_completion: (&str, u32) = ("_request_code_completion", 2586408642u32);
    pub type Sig_request_code_completion = (bool,);
    pub const filter_code_completion_candidates: (&str, u32) = ("_filter_code_completion_candidates", 2560709669u32);
    pub type Sig_filter_code_completion_candidates = (Array < VarDictionary >,);
    
}
pub mod CodeHighlighter {
    pub use super::SyntaxHighlighter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CollisionObject2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const input_event: (&str, u32) = ("_input_event", 1847696837u32);
    pub type Sig_input_event = (Gd < crate::classes::Viewport >, Gd < crate::classes::InputEvent >, i32,);
    pub const mouse_enter: (&str, u32) = ("_mouse_enter", 3218959716u32);
    pub type Sig_mouse_enter = ();
    pub const mouse_exit: (&str, u32) = ("_mouse_exit", 3218959716u32);
    pub type Sig_mouse_exit = ();
    pub const mouse_shape_enter: (&str, u32) = ("_mouse_shape_enter", 1286410249u32);
    pub type Sig_mouse_shape_enter = (i32,);
    pub const mouse_shape_exit: (&str, u32) = ("_mouse_shape_exit", 1286410249u32);
    pub type Sig_mouse_shape_exit = (i32,);
    
}
pub mod CollisionObject3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const input_event: (&str, u32) = ("_input_event", 2310605070u32);
    pub type Sig_input_event = (Option < Gd < crate::classes::Camera3D > >, Option < Gd < crate::classes::InputEvent > >, Vector3, Vector3, i32,);
    pub const mouse_enter: (&str, u32) = ("_mouse_enter", 3218959716u32);
    pub type Sig_mouse_enter = ();
    pub const mouse_exit: (&str, u32) = ("_mouse_exit", 3218959716u32);
    pub type Sig_mouse_exit = ();
    
}
pub mod CollisionPolygon2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CollisionPolygon3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CollisionShape2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CollisionShape3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ColorPalette {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ColorPicker {
    pub use super::VBoxContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ColorPickerButton {
    pub use super::Button::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ColorRect {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CompressedCubemap {
    pub use super::CompressedTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CompressedCubemapArray {
    pub use super::CompressedTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CompressedTexture2D {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CompressedTexture2DArray {
    pub use super::CompressedTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CompressedTexture3D {
    pub use super::Texture3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CompressedTextureLayered {
    pub use super::TextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ConcavePolygonShape2D {
    pub use super::Shape2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ConcavePolygonShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ConeTwistJoint3D {
    pub use super::Joint3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ConfigFile {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ConfirmationDialog {
    pub use super::AcceptDialog::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Container {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_allowed_size_flags_horizontal: (&str, u32) = ("_get_allowed_size_flags_horizontal", 1930428628u32);
    pub type Sig_get_allowed_size_flags_horizontal = ();
    pub const get_allowed_size_flags_vertical: (&str, u32) = ("_get_allowed_size_flags_vertical", 1930428628u32);
    pub type Sig_get_allowed_size_flags_vertical = ();
    
}
pub mod Control {
    pub use super::CanvasItem::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const has_point: (&str, u32) = ("_has_point", 556197845u32);
    pub type Sig_has_point = (Vector2,);
    pub const structured_text_parser: (&str, u32) = ("_structured_text_parser", 1292548940u32);
    pub type Sig_structured_text_parser = (VarArray, GString,);
    pub const get_minimum_size: (&str, u32) = ("_get_minimum_size", 3341600327u32);
    pub type Sig_get_minimum_size = ();
    pub const get_tooltip: (&str, u32) = ("_get_tooltip", 3674420000u32);
    pub type Sig_get_tooltip = (Vector2,);
    pub const get_drag_data: (&str, u32) = ("_get_drag_data", 2233896889u32);
    pub type Sig_get_drag_data = (Vector2,);
    pub const can_drop_data: (&str, u32) = ("_can_drop_data", 2603004011u32);
    pub type Sig_can_drop_data = (Vector2, Variant,);
    pub const drop_data: (&str, u32) = ("_drop_data", 3699746064u32);
    pub type Sig_drop_data = (Vector2, Variant,);
    pub const make_custom_tooltip: (&str, u32) = ("_make_custom_tooltip", 1976279298u32);
    pub type Sig_make_custom_tooltip = (GString,);
    pub const accessibility_get_contextual_info: (&str, u32) = ("_accessibility_get_contextual_info", 201670096u32);
    pub type Sig_accessibility_get_contextual_info = ();
    pub const get_accessibility_container_name: (&str, u32) = ("_get_accessibility_container_name", 2174079723u32);
    pub type Sig_get_accessibility_container_name = (Option < Gd < crate::classes::Node > >,);
    pub const gui_input: (&str, u32) = ("_gui_input", 3754044979u32);
    pub type Sig_gui_input = (Gd < crate::classes::InputEvent >,);
    
}
pub mod ConvertTransformModifier3D {
    pub use super::BoneConstraint3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ConvexPolygonShape2D {
    pub use super::Shape2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ConvexPolygonShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CopyTransformModifier3D {
    pub use super::BoneConstraint3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Crypto {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CryptoKey {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Cubemap {
    pub use super::ImageTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CubemapArray {
    pub use super::ImageTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Curve {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Curve2D {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Curve3D {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CurveTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CurveXyzTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CylinderMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod CylinderShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod DpiTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod DtlsServer {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod DampedSpringJoint2D {
    pub use super::Joint2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Decal {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod DirAccess {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod DirectionalLight2D {
    pub use super::Light2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod DirectionalLight3D {
    pub use super::Light3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod DisplayServer {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ENetConnection {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ENetMultiplayerPeer {
    pub use super::MultiplayerPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ENetPacketPeer {
    pub use super::PacketPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorCommandPalette {
    pub use super::ConfirmationDialog::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorContextMenuPlugin {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const popup_menu: (&str, u32) = ("_popup_menu", 4015028928u32);
    pub type Sig_popup_menu = (PackedStringArray,);
    
}
pub mod EditorDebuggerPlugin {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const setup_session: (&str, u32) = ("_setup_session", 1286410249u32);
    pub type Sig_setup_session = (i32,);
    pub const has_capture: (&str, u32) = ("_has_capture", 3927539163u32);
    pub type Sig_has_capture = (GString,);
    pub const capture: (&str, u32) = ("_capture", 2607901833u32);
    pub type Sig_capture = (GString, VarArray, i32,);
    pub const goto_script_line: (&str, u32) = ("_goto_script_line", 1208513123u32);
    pub type Sig_goto_script_line = (Option < Gd < crate::classes::Script > >, i32,);
    pub const breakpoints_cleared_in_tree: (&str, u32) = ("_breakpoints_cleared_in_tree", 3218959716u32);
    pub type Sig_breakpoints_cleared_in_tree = ();
    pub const breakpoint_set_in_tree: (&str, u32) = ("_breakpoint_set_in_tree", 2338735218u32);
    pub type Sig_breakpoint_set_in_tree = (Option < Gd < crate::classes::Script > >, i32, bool,);
    
}
pub mod EditorDebuggerSession {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatform {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformAndroid {
    pub use super::EditorExportPlatform::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformAppleEmbedded {
    pub use super::EditorExportPlatform::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformExtension {
    pub use super::EditorExportPlatform::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_preset_features: (&str, u32) = ("_get_preset_features", 1387456631u32);
    pub type Sig_get_preset_features = (Option < Gd < crate::classes::EditorExportPreset > >,);
    pub const is_executable: (&str, u32) = ("_is_executable", 3927539163u32);
    pub type Sig_is_executable = (GString,);
    pub const get_export_options: (&str, u32) = ("_get_export_options", 3995934104u32);
    pub type Sig_get_export_options = ();
    pub const should_update_export_options: (&str, u32) = ("_should_update_export_options", 2240911060u32);
    pub type Sig_should_update_export_options = ();
    pub const get_export_option_visibility: (&str, u32) = ("_get_export_option_visibility", 969350244u32);
    pub type Sig_get_export_option_visibility = (Option < Gd < crate::classes::EditorExportPreset > >, GString,);
    pub const get_export_option_warning: (&str, u32) = ("_get_export_option_warning", 805886795u32);
    pub type Sig_get_export_option_warning = (Option < Gd < crate::classes::EditorExportPreset > >, StringName,);
    pub const get_os_name: (&str, u32) = ("_get_os_name", 201670096u32);
    pub type Sig_get_os_name = ();
    pub const get_name: (&str, u32) = ("_get_name", 201670096u32);
    pub type Sig_get_name = ();
    pub const get_logo: (&str, u32) = ("_get_logo", 3635182373u32);
    pub type Sig_get_logo = ();
    pub const poll_export: (&str, u32) = ("_poll_export", 2240911060u32);
    pub type Sig_poll_export = ();
    pub const get_options_count: (&str, u32) = ("_get_options_count", 3905245786u32);
    pub type Sig_get_options_count = ();
    pub const get_options_tooltip: (&str, u32) = ("_get_options_tooltip", 201670096u32);
    pub type Sig_get_options_tooltip = ();
    pub const get_option_icon: (&str, u32) = ("_get_option_icon", 3536238170u32);
    pub type Sig_get_option_icon = (i32,);
    pub const get_option_label: (&str, u32) = ("_get_option_label", 844755477u32);
    pub type Sig_get_option_label = (i32,);
    pub const get_option_tooltip: (&str, u32) = ("_get_option_tooltip", 844755477u32);
    pub type Sig_get_option_tooltip = (i32,);
    pub const get_device_architecture: (&str, u32) = ("_get_device_architecture", 844755477u32);
    pub type Sig_get_device_architecture = (i32,);
    pub const cleanup: (&str, u32) = ("_cleanup", 3218959716u32);
    pub type Sig_cleanup = ();
    pub const run: (&str, u32) = ("_run", 1726914928u32);
    pub type Sig_run = (Option < Gd < crate::classes::EditorExportPreset > >, i32, crate::classes::editor_export_platform::DebugFlags,);
    pub const get_run_icon: (&str, u32) = ("_get_run_icon", 3635182373u32);
    pub type Sig_get_run_icon = ();
    pub const can_export: (&str, u32) = ("_can_export", 493961987u32);
    pub type Sig_can_export = (Option < Gd < crate::classes::EditorExportPreset > >, bool,);
    pub const has_valid_export_configuration: (&str, u32) = ("_has_valid_export_configuration", 493961987u32);
    pub type Sig_has_valid_export_configuration = (Option < Gd < crate::classes::EditorExportPreset > >, bool,);
    pub const has_valid_project_configuration: (&str, u32) = ("_has_valid_project_configuration", 3117166915u32);
    pub type Sig_has_valid_project_configuration = (Option < Gd < crate::classes::EditorExportPreset > >,);
    pub const get_binary_extensions: (&str, u32) = ("_get_binary_extensions", 1387456631u32);
    pub type Sig_get_binary_extensions = (Option < Gd < crate::classes::EditorExportPreset > >,);
    pub const export_project: (&str, u32) = ("_export_project", 1328957260u32);
    pub type Sig_export_project = (Option < Gd < crate::classes::EditorExportPreset > >, bool, GString, crate::classes::editor_export_platform::DebugFlags,);
    pub const export_pack: (&str, u32) = ("_export_pack", 1328957260u32);
    pub type Sig_export_pack = (Option < Gd < crate::classes::EditorExportPreset > >, bool, GString, crate::classes::editor_export_platform::DebugFlags,);
    pub const export_zip: (&str, u32) = ("_export_zip", 1328957260u32);
    pub type Sig_export_zip = (Option < Gd < crate::classes::EditorExportPreset > >, bool, GString, crate::classes::editor_export_platform::DebugFlags,);
    pub const export_pack_patch: (&str, u32) = ("_export_pack_patch", 454765315u32);
    pub type Sig_export_pack_patch = (Option < Gd < crate::classes::EditorExportPreset > >, bool, GString, PackedStringArray, crate::classes::editor_export_platform::DebugFlags,);
    pub const export_zip_patch: (&str, u32) = ("_export_zip_patch", 454765315u32);
    pub type Sig_export_zip_patch = (Option < Gd < crate::classes::EditorExportPreset > >, bool, GString, PackedStringArray, crate::classes::editor_export_platform::DebugFlags,);
    pub const get_platform_features: (&str, u32) = ("_get_platform_features", 1139954409u32);
    pub type Sig_get_platform_features = ();
    pub const get_debug_protocol: (&str, u32) = ("_get_debug_protocol", 201670096u32);
    pub type Sig_get_debug_protocol = ();
    
}
pub mod EditorExportPlatformIos {
    pub use super::EditorExportPlatformAppleEmbedded::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformLinuxBsd {
    pub use super::EditorExportPlatformPc::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformMacOs {
    pub use super::EditorExportPlatform::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformPc {
    pub use super::EditorExportPlatform::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformVisionOs {
    pub use super::EditorExportPlatformAppleEmbedded::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformWeb {
    pub use super::EditorExportPlatform::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlatformWindows {
    pub use super::EditorExportPlatformPc::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorExportPlugin {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const export_file: (&str, u32) = ("_export_file", 3533781844u32);
    pub type Sig_export_file = (GString, GString, PackedStringArray,);
    pub const export_begin: (&str, u32) = ("_export_begin", 2765511433u32);
    pub type Sig_export_begin = (PackedStringArray, bool, GString, u32,);
    pub const export_end: (&str, u32) = ("_export_end", 3218959716u32);
    pub type Sig_export_end = ();
    pub const begin_customize_resources: (&str, u32) = ("_begin_customize_resources", 1312023292u32);
    pub type Sig_begin_customize_resources = (Option < Gd < crate::classes::EditorExportPlatform > >, PackedStringArray,);
    pub const customize_resource: (&str, u32) = ("_customize_resource", 307917495u32);
    pub type Sig_customize_resource = (Gd < crate::classes::Resource >, GString,);
    pub const begin_customize_scenes: (&str, u32) = ("_begin_customize_scenes", 1312023292u32);
    pub type Sig_begin_customize_scenes = (Option < Gd < crate::classes::EditorExportPlatform > >, PackedStringArray,);
    pub const customize_scene: (&str, u32) = ("_customize_scene", 498701822u32);
    pub type Sig_customize_scene = (Gd < crate::classes::Node >, GString,);
    pub const get_customization_configuration_hash: (&str, u32) = ("_get_customization_configuration_hash", 3905245786u32);
    pub type Sig_get_customization_configuration_hash = ();
    pub const end_customize_scenes: (&str, u32) = ("_end_customize_scenes", 3218959716u32);
    pub type Sig_end_customize_scenes = ();
    pub const end_customize_resources: (&str, u32) = ("_end_customize_resources", 3218959716u32);
    pub type Sig_end_customize_resources = ();
    pub const get_export_options: (&str, u32) = ("_get_export_options", 488349689u32);
    pub type Sig_get_export_options = (Option < Gd < crate::classes::EditorExportPlatform > >,);
    pub const get_export_options_overrides: (&str, u32) = ("_get_export_options_overrides", 2837326714u32);
    pub type Sig_get_export_options_overrides = (Option < Gd < crate::classes::EditorExportPlatform > >,);
    pub const should_update_export_options: (&str, u32) = ("_should_update_export_options", 1866233299u32);
    pub type Sig_should_update_export_options = (Option < Gd < crate::classes::EditorExportPlatform > >,);
    pub const get_export_option_visibility: (&str, u32) = ("_get_export_option_visibility", 3537301980u32);
    pub type Sig_get_export_option_visibility = (Option < Gd < crate::classes::EditorExportPlatform > >, GString,);
    pub const get_export_option_warning: (&str, u32) = ("_get_export_option_warning", 3340251247u32);
    pub type Sig_get_export_option_warning = (Option < Gd < crate::classes::EditorExportPlatform > >, GString,);
    pub const get_export_features: (&str, u32) = ("_get_export_features", 1057664154u32);
    pub type Sig_get_export_features = (Option < Gd < crate::classes::EditorExportPlatform > >, bool,);
    pub const get_name: (&str, u32) = ("_get_name", 201670096u32);
    pub type Sig_get_name = ();
    pub const supports_platform: (&str, u32) = ("_supports_platform", 1866233299u32);
    pub type Sig_supports_platform = (Option < Gd < crate::classes::EditorExportPlatform > >,);
    pub const get_android_dependencies: (&str, u32) = ("_get_android_dependencies", 1057664154u32);
    pub type Sig_get_android_dependencies = (Option < Gd < crate::classes::EditorExportPlatform > >, bool,);
    pub const get_android_dependencies_maven_repos: (&str, u32) = ("_get_android_dependencies_maven_repos", 1057664154u32);
    pub type Sig_get_android_dependencies_maven_repos = (Option < Gd < crate::classes::EditorExportPlatform > >, bool,);
    pub const get_android_libraries: (&str, u32) = ("_get_android_libraries", 1057664154u32);
    pub type Sig_get_android_libraries = (Option < Gd < crate::classes::EditorExportPlatform > >, bool,);
    pub const get_android_manifest_activity_element_contents: (&str, u32) = ("_get_android_manifest_activity_element_contents", 4013372917u32);
    pub type Sig_get_android_manifest_activity_element_contents = (Option < Gd < crate::classes::EditorExportPlatform > >, bool,);
    pub const get_android_manifest_application_element_contents: (&str, u32) = ("_get_android_manifest_application_element_contents", 4013372917u32);
    pub type Sig_get_android_manifest_application_element_contents = (Option < Gd < crate::classes::EditorExportPlatform > >, bool,);
    pub const get_android_manifest_element_contents: (&str, u32) = ("_get_android_manifest_element_contents", 4013372917u32);
    pub type Sig_get_android_manifest_element_contents = (Option < Gd < crate::classes::EditorExportPlatform > >, bool,);
    pub const update_android_prebuilt_manifest: (&str, u32) = ("_update_android_prebuilt_manifest", 3304965187u32);
    pub type Sig_update_android_prebuilt_manifest = (Option < Gd < crate::classes::EditorExportPlatform > >, PackedByteArray,);
    
}
pub mod EditorExportPreset {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorFeatureProfile {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorFileDialog {
    pub use super::ConfirmationDialog::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorFileSystem {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorFileSystemDirectory {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorFileSystemImportFormatSupportQuery {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const is_active: (&str, u32) = ("_is_active", 36873697u32);
    pub type Sig_is_active = ();
    pub const get_file_extensions: (&str, u32) = ("_get_file_extensions", 1139954409u32);
    pub type Sig_get_file_extensions = ();
    pub const query: (&str, u32) = ("_query", 36873697u32);
    pub type Sig_query = ();
    
}
pub mod EditorImportPlugin {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_importer_name: (&str, u32) = ("_get_importer_name", 201670096u32);
    pub type Sig_get_importer_name = ();
    pub const get_visible_name: (&str, u32) = ("_get_visible_name", 201670096u32);
    pub type Sig_get_visible_name = ();
    pub const get_preset_count: (&str, u32) = ("_get_preset_count", 3905245786u32);
    pub type Sig_get_preset_count = ();
    pub const get_preset_name: (&str, u32) = ("_get_preset_name", 844755477u32);
    pub type Sig_get_preset_name = (i32,);
    pub const get_recognized_extensions: (&str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub type Sig_get_recognized_extensions = ();
    pub const get_import_options: (&str, u32) = ("_get_import_options", 520498173u32);
    pub type Sig_get_import_options = (GString, i32,);
    pub const get_save_extension: (&str, u32) = ("_get_save_extension", 201670096u32);
    pub type Sig_get_save_extension = ();
    pub const get_resource_type: (&str, u32) = ("_get_resource_type", 201670096u32);
    pub type Sig_get_resource_type = ();
    pub const get_priority: (&str, u32) = ("_get_priority", 1740695150u32);
    pub type Sig_get_priority = ();
    pub const get_import_order: (&str, u32) = ("_get_import_order", 3905245786u32);
    pub type Sig_get_import_order = ();
    pub const get_format_version: (&str, u32) = ("_get_format_version", 3905245786u32);
    pub type Sig_get_format_version = ();
    pub const get_option_visibility: (&str, u32) = ("_get_option_visibility", 240466755u32);
    pub type Sig_get_option_visibility = (GString, StringName, VarDictionary,);
    pub const import: (&str, u32) = ("_import", 4108152118u32);
    pub type Sig_import = (GString, GString, VarDictionary, Array < GString >, Array < GString >,);
    pub const can_import_threaded: (&str, u32) = ("_can_import_threaded", 36873697u32);
    pub type Sig_can_import_threaded = ();
    
}
pub mod EditorInspector {
    pub use super::ScrollContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorInspectorPlugin {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const can_handle: (&str, u32) = ("_can_handle", 397768994u32);
    pub type Sig_can_handle = (Option < Gd < crate::classes::Object > >,);
    pub const parse_begin: (&str, u32) = ("_parse_begin", 3975164845u32);
    pub type Sig_parse_begin = (Option < Gd < crate::classes::Object > >,);
    pub const parse_category: (&str, u32) = ("_parse_category", 357144787u32);
    pub type Sig_parse_category = (Option < Gd < crate::classes::Object > >, GString,);
    pub const parse_group: (&str, u32) = ("_parse_group", 357144787u32);
    pub type Sig_parse_group = (Option < Gd < crate::classes::Object > >, GString,);
    pub const parse_property: (&str, u32) = ("_parse_property", 1087679910u32);
    pub type Sig_parse_property = (Option < Gd < crate::classes::Object > >, VariantType, GString, crate::global::PropertyHint, GString, crate::global::PropertyUsageFlags, bool,);
    pub const parse_end: (&str, u32) = ("_parse_end", 3975164845u32);
    pub type Sig_parse_end = (Option < Gd < crate::classes::Object > >,);
    
}
pub mod EditorInterface {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorNode3DGizmo {
    pub use super::Node3DGizmo::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const redraw: (&str, u32) = ("_redraw", 3218959716u32);
    pub type Sig_redraw = ();
    pub const get_handle_name: (&str, u32) = ("_get_handle_name", 1868713439u32);
    pub type Sig_get_handle_name = (i32, bool,);
    pub const is_handle_highlighted: (&str, u32) = ("_is_handle_highlighted", 361316320u32);
    pub type Sig_is_handle_highlighted = (i32, bool,);
    pub const get_handle_value: (&str, u32) = ("_get_handle_value", 2144196525u32);
    pub type Sig_get_handle_value = (i32, bool,);
    pub const begin_handle_action: (&str, u32) = ("_begin_handle_action", 300928843u32);
    pub type Sig_begin_handle_action = (i32, bool,);
    pub const set_handle: (&str, u32) = ("_set_handle", 2210262157u32);
    pub type Sig_set_handle = (i32, bool, Option < Gd < crate::classes::Camera3D > >, Vector2,);
    pub const commit_handle: (&str, u32) = ("_commit_handle", 3655739840u32);
    pub type Sig_commit_handle = (i32, bool, Variant, bool,);
    pub const subgizmos_intersect_ray: (&str, u32) = ("_subgizmos_intersect_ray", 2055005479u32);
    pub type Sig_subgizmos_intersect_ray = (Option < Gd < crate::classes::Camera3D > >, Vector2,);
    pub const subgizmos_intersect_frustum: (&str, u32) = ("_subgizmos_intersect_frustum", 1653813165u32);
    pub type Sig_subgizmos_intersect_frustum = (Option < Gd < crate::classes::Camera3D > >, Array < Plane >,);
    pub const set_subgizmo_transform: (&str, u32) = ("_set_subgizmo_transform", 3616898986u32);
    pub type Sig_set_subgizmo_transform = (i32, Transform3D,);
    pub const get_subgizmo_transform: (&str, u32) = ("_get_subgizmo_transform", 1965739696u32);
    pub type Sig_get_subgizmo_transform = (i32,);
    pub const commit_subgizmos: (&str, u32) = ("_commit_subgizmos", 3411059856u32);
    pub type Sig_commit_subgizmos = (PackedInt32Array, Array < Transform3D >, bool,);
    
}
pub mod EditorNode3DGizmoPlugin {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const has_gizmo: (&str, u32) = ("_has_gizmo", 1905827158u32);
    pub type Sig_has_gizmo = (Option < Gd < crate::classes::Node3D > >,);
    pub const create_gizmo: (&str, u32) = ("_create_gizmo", 1418965287u32);
    pub type Sig_create_gizmo = (Option < Gd < crate::classes::Node3D > >,);
    pub const get_gizmo_name: (&str, u32) = ("_get_gizmo_name", 201670096u32);
    pub type Sig_get_gizmo_name = ();
    pub const get_priority: (&str, u32) = ("_get_priority", 3905245786u32);
    pub type Sig_get_priority = ();
    pub const can_be_hidden: (&str, u32) = ("_can_be_hidden", 36873697u32);
    pub type Sig_can_be_hidden = ();
    pub const is_selectable_when_hidden: (&str, u32) = ("_is_selectable_when_hidden", 36873697u32);
    pub type Sig_is_selectable_when_hidden = ();
    pub const redraw: (&str, u32) = ("_redraw", 173330131u32);
    pub type Sig_redraw = (Option < Gd < crate::classes::EditorNode3DGizmo > >,);
    pub const get_handle_name: (&str, u32) = ("_get_handle_name", 3888674840u32);
    pub type Sig_get_handle_name = (Option < Gd < crate::classes::EditorNode3DGizmo > >, i32, bool,);
    pub const is_handle_highlighted: (&str, u32) = ("_is_handle_highlighted", 2665780718u32);
    pub type Sig_is_handle_highlighted = (Option < Gd < crate::classes::EditorNode3DGizmo > >, i32, bool,);
    pub const get_handle_value: (&str, u32) = ("_get_handle_value", 2887724832u32);
    pub type Sig_get_handle_value = (Option < Gd < crate::classes::EditorNode3DGizmo > >, i32, bool,);
    pub const begin_handle_action: (&str, u32) = ("_begin_handle_action", 3363704593u32);
    pub type Sig_begin_handle_action = (Option < Gd < crate::classes::EditorNode3DGizmo > >, i32, bool,);
    pub const set_handle: (&str, u32) = ("_set_handle", 1249646868u32);
    pub type Sig_set_handle = (Option < Gd < crate::classes::EditorNode3DGizmo > >, i32, bool, Option < Gd < crate::classes::Camera3D > >, Vector2,);
    pub const commit_handle: (&str, u32) = ("_commit_handle", 1939863962u32);
    pub type Sig_commit_handle = (Option < Gd < crate::classes::EditorNode3DGizmo > >, i32, bool, Variant, bool,);
    pub const subgizmos_intersect_ray: (&str, u32) = ("_subgizmos_intersect_ray", 1781916302u32);
    pub type Sig_subgizmos_intersect_ray = (Option < Gd < crate::classes::EditorNode3DGizmo > >, Option < Gd < crate::classes::Camera3D > >, Vector2,);
    pub const subgizmos_intersect_frustum: (&str, u32) = ("_subgizmos_intersect_frustum", 3514748524u32);
    pub type Sig_subgizmos_intersect_frustum = (Option < Gd < crate::classes::EditorNode3DGizmo > >, Option < Gd < crate::classes::Camera3D > >, Array < Plane >,);
    pub const get_subgizmo_transform: (&str, u32) = ("_get_subgizmo_transform", 3700343508u32);
    pub type Sig_get_subgizmo_transform = (Option < Gd < crate::classes::EditorNode3DGizmo > >, i32,);
    pub const set_subgizmo_transform: (&str, u32) = ("_set_subgizmo_transform", 2435388792u32);
    pub type Sig_set_subgizmo_transform = (Option < Gd < crate::classes::EditorNode3DGizmo > >, i32, Transform3D,);
    pub const commit_subgizmos: (&str, u32) = ("_commit_subgizmos", 2282018236u32);
    pub type Sig_commit_subgizmos = (Option < Gd < crate::classes::EditorNode3DGizmo > >, PackedInt32Array, Array < Transform3D >, bool,);
    
}
pub mod EditorPaths {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorPlugin {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const forward_canvas_gui_input: (&str, u32) = ("_forward_canvas_gui_input", 1062211774u32);
    pub type Sig_forward_canvas_gui_input = (Option < Gd < crate::classes::InputEvent > >,);
    pub const forward_canvas_draw_over_viewport: (&str, u32) = ("_forward_canvas_draw_over_viewport", 1496901182u32);
    pub type Sig_forward_canvas_draw_over_viewport = (Option < Gd < crate::classes::Control > >,);
    pub const forward_canvas_force_draw_over_viewport: (&str, u32) = ("_forward_canvas_force_draw_over_viewport", 1496901182u32);
    pub type Sig_forward_canvas_force_draw_over_viewport = (Option < Gd < crate::classes::Control > >,);
    pub const forward_3d_gui_input: (&str, u32) = ("_forward_3d_gui_input", 1018736637u32);
    pub type Sig_forward_3d_gui_input = (Option < Gd < crate::classes::Camera3D > >, Option < Gd < crate::classes::InputEvent > >,);
    pub const forward_3d_draw_over_viewport: (&str, u32) = ("_forward_3d_draw_over_viewport", 1496901182u32);
    pub type Sig_forward_3d_draw_over_viewport = (Option < Gd < crate::classes::Control > >,);
    pub const forward_3d_force_draw_over_viewport: (&str, u32) = ("_forward_3d_force_draw_over_viewport", 1496901182u32);
    pub type Sig_forward_3d_force_draw_over_viewport = (Option < Gd < crate::classes::Control > >,);
    pub const get_plugin_name: (&str, u32) = ("_get_plugin_name", 201670096u32);
    pub type Sig_get_plugin_name = ();
    pub const get_plugin_icon: (&str, u32) = ("_get_plugin_icon", 3635182373u32);
    pub type Sig_get_plugin_icon = ();
    pub const has_main_screen: (&str, u32) = ("_has_main_screen", 36873697u32);
    pub type Sig_has_main_screen = ();
    pub const make_visible: (&str, u32) = ("_make_visible", 2586408642u32);
    pub type Sig_make_visible = (bool,);
    pub const edit: (&str, u32) = ("_edit", 3975164845u32);
    pub type Sig_edit = (Option < Gd < crate::classes::Object > >,);
    pub const handles: (&str, u32) = ("_handles", 397768994u32);
    pub type Sig_handles = (Gd < crate::classes::Object >,);
    pub const get_state: (&str, u32) = ("_get_state", 3102165223u32);
    pub type Sig_get_state = ();
    pub const set_state: (&str, u32) = ("_set_state", 4155329257u32);
    pub type Sig_set_state = (VarDictionary,);
    pub const clear: (&str, u32) = ("_clear", 3218959716u32);
    pub type Sig_clear = ();
    pub const get_unsaved_status: (&str, u32) = ("_get_unsaved_status", 3135753539u32);
    pub type Sig_get_unsaved_status = (GString,);
    pub const save_external_data: (&str, u32) = ("_save_external_data", 3218959716u32);
    pub type Sig_save_external_data = ();
    pub const apply_changes: (&str, u32) = ("_apply_changes", 3218959716u32);
    pub type Sig_apply_changes = ();
    pub const get_breakpoints: (&str, u32) = ("_get_breakpoints", 1139954409u32);
    pub type Sig_get_breakpoints = ();
    pub const set_window_layout: (&str, u32) = ("_set_window_layout", 853519107u32);
    pub type Sig_set_window_layout = (Option < Gd < crate::classes::ConfigFile > >,);
    pub const get_window_layout: (&str, u32) = ("_get_window_layout", 853519107u32);
    pub type Sig_get_window_layout = (Option < Gd < crate::classes::ConfigFile > >,);
    pub const build: (&str, u32) = ("_build", 2240911060u32);
    pub type Sig_build = ();
    pub const enable_plugin: (&str, u32) = ("_enable_plugin", 3218959716u32);
    pub type Sig_enable_plugin = ();
    pub const disable_plugin: (&str, u32) = ("_disable_plugin", 3218959716u32);
    pub type Sig_disable_plugin = ();
    
}
pub mod EditorProperty {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const update_property: (&str, u32) = ("_update_property", 3218959716u32);
    pub type Sig_update_property = ();
    pub const set_read_only: (&str, u32) = ("_set_read_only", 2586408642u32);
    pub type Sig_set_read_only = (bool,);
    
}
pub mod EditorResourceConversionPlugin {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const converts_to: (&str, u32) = ("_converts_to", 201670096u32);
    pub type Sig_converts_to = ();
    pub const handles: (&str, u32) = ("_handles", 3190994482u32);
    pub type Sig_handles = (Option < Gd < crate::classes::Resource > >,);
    pub const convert: (&str, u32) = ("_convert", 325183270u32);
    pub type Sig_convert = (Option < Gd < crate::classes::Resource > >,);
    
}
pub mod EditorResourcePicker {
    pub use super::HBoxContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const set_create_options: (&str, u32) = ("_set_create_options", 3975164845u32);
    pub type Sig_set_create_options = (Option < Gd < crate::classes::Object > >,);
    pub const handle_menu_selected: (&str, u32) = ("_handle_menu_selected", 3067735520u32);
    pub type Sig_handle_menu_selected = (i32,);
    
}
pub mod EditorResourcePreview {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorResourcePreviewGenerator {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const handles: (&str, u32) = ("_handles", 3927539163u32);
    pub type Sig_handles = (GString,);
    pub const generate: (&str, u32) = ("_generate", 255939159u32);
    pub type Sig_generate = (Option < Gd < crate::classes::Resource > >, Vector2i, VarDictionary,);
    pub const generate_from_path: (&str, u32) = ("_generate_from_path", 1601192835u32);
    pub type Sig_generate_from_path = (GString, Vector2i, VarDictionary,);
    pub const generate_small_preview_automatically: (&str, u32) = ("_generate_small_preview_automatically", 36873697u32);
    pub type Sig_generate_small_preview_automatically = ();
    pub const can_generate_small_preview: (&str, u32) = ("_can_generate_small_preview", 36873697u32);
    pub type Sig_can_generate_small_preview = ();
    
}
pub mod EditorResourceTooltipPlugin {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const handles: (&str, u32) = ("_handles", 3927539163u32);
    pub type Sig_handles = (GString,);
    pub const make_tooltip_for_path: (&str, u32) = ("_make_tooltip_for_path", 4100114520u32);
    pub type Sig_make_tooltip_for_path = (GString, VarDictionary, Option < Gd < crate::classes::Control > >,);
    
}
pub mod EditorSceneFormatImporter {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_extensions: (&str, u32) = ("_get_extensions", 1139954409u32);
    pub type Sig_get_extensions = ();
    pub const import_scene: (&str, u32) = ("_import_scene", 3749238728u32);
    pub type Sig_import_scene = (GString, u32, VarDictionary,);
    pub const get_import_options: (&str, u32) = ("_get_import_options", 83702148u32);
    pub type Sig_get_import_options = (GString,);
    pub const get_option_visibility: (&str, u32) = ("_get_option_visibility", 298836892u32);
    pub type Sig_get_option_visibility = (GString, bool, GString,);
    
}
pub mod EditorSceneFormatImporterBlend {
    pub use super::EditorSceneFormatImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorSceneFormatImporterFbx2gltf {
    pub use super::EditorSceneFormatImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorSceneFormatImporterGltf {
    pub use super::EditorSceneFormatImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorSceneFormatImporterUfbx {
    pub use super::EditorSceneFormatImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorScenePostImport {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const post_import: (&str, u32) = ("_post_import", 134930648u32);
    pub type Sig_post_import = (Option < Gd < crate::classes::Node > >,);
    
}
pub mod EditorScenePostImportPlugin {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_internal_import_options: (&str, u32) = ("_get_internal_import_options", 1286410249u32);
    pub type Sig_get_internal_import_options = (i32,);
    pub const get_internal_option_visibility: (&str, u32) = ("_get_internal_option_visibility", 3811255416u32);
    pub type Sig_get_internal_option_visibility = (i32, bool, GString,);
    pub const get_internal_option_update_view_required: (&str, u32) = ("_get_internal_option_update_view_required", 3957349696u32);
    pub type Sig_get_internal_option_update_view_required = (i32, GString,);
    pub const internal_process: (&str, u32) = ("_internal_process", 3641982463u32);
    pub type Sig_internal_process = (i32, Option < Gd < crate::classes::Node > >, Option < Gd < crate::classes::Node > >, Option < Gd < crate::classes::Resource > >,);
    pub const get_import_options: (&str, u32) = ("_get_import_options", 83702148u32);
    pub type Sig_get_import_options = (GString,);
    pub const get_option_visibility: (&str, u32) = ("_get_option_visibility", 298836892u32);
    pub type Sig_get_option_visibility = (GString, bool, GString,);
    pub const pre_process: (&str, u32) = ("_pre_process", 1078189570u32);
    pub type Sig_pre_process = (Option < Gd < crate::classes::Node > >,);
    pub const post_process: (&str, u32) = ("_post_process", 1078189570u32);
    pub type Sig_post_process = (Option < Gd < crate::classes::Node > >,);
    
}
pub mod EditorScript {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const run: (&str, u32) = ("_run", 3218959716u32);
    pub type Sig_run = ();
    
}
pub mod EditorScriptPicker {
    pub use super::EditorResourcePicker::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorSelection {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorSettings {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorSpinSlider {
    pub use super::Range::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorSyntaxHighlighter {
    pub use super::SyntaxHighlighter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_name: (&str, u32) = ("_get_name", 201670096u32);
    pub type Sig_get_name = ();
    pub const get_supported_languages: (&str, u32) = ("_get_supported_languages", 1139954409u32);
    pub type Sig_get_supported_languages = ();
    pub const create: (&str, u32) = ("_create", 3789807118u32);
    pub type Sig_create = ();
    
}
pub mod EditorToaster {
    pub use super::HBoxContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorTranslationParserPlugin {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const parse_file: (&str, u32) = ("_parse_file", 1576865988u32);
    pub type Sig_parse_file = (GString,);
    pub const get_recognized_extensions: (&str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub type Sig_get_recognized_extensions = ();
    
}
pub mod EditorUndoRedoManager {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EditorVcsInterface {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const initialize: (&str, u32) = ("_initialize", 2323990056u32);
    pub type Sig_initialize = (GString,);
    pub const set_credentials: (&str, u32) = ("_set_credentials", 1336744649u32);
    pub type Sig_set_credentials = (GString, GString, GString, GString, GString,);
    pub const get_modified_files_data: (&str, u32) = ("_get_modified_files_data", 2915620761u32);
    pub type Sig_get_modified_files_data = ();
    pub const stage_file: (&str, u32) = ("_stage_file", 83702148u32);
    pub type Sig_stage_file = (GString,);
    pub const unstage_file: (&str, u32) = ("_unstage_file", 83702148u32);
    pub type Sig_unstage_file = (GString,);
    pub const discard_file: (&str, u32) = ("_discard_file", 83702148u32);
    pub type Sig_discard_file = (GString,);
    pub const commit: (&str, u32) = ("_commit", 83702148u32);
    pub type Sig_commit = (GString,);
    pub const get_diff: (&str, u32) = ("_get_diff", 1366379175u32);
    pub type Sig_get_diff = (GString, i32,);
    pub const shut_down: (&str, u32) = ("_shut_down", 2240911060u32);
    pub type Sig_shut_down = ();
    pub const get_vcs_name: (&str, u32) = ("_get_vcs_name", 2841200299u32);
    pub type Sig_get_vcs_name = ();
    pub const get_previous_commits: (&str, u32) = ("_get_previous_commits", 1171824711u32);
    pub type Sig_get_previous_commits = (i32,);
    pub const get_branch_list: (&str, u32) = ("_get_branch_list", 2915620761u32);
    pub type Sig_get_branch_list = ();
    pub const get_remotes: (&str, u32) = ("_get_remotes", 2915620761u32);
    pub type Sig_get_remotes = ();
    pub const create_branch: (&str, u32) = ("_create_branch", 83702148u32);
    pub type Sig_create_branch = (GString,);
    pub const remove_branch: (&str, u32) = ("_remove_branch", 83702148u32);
    pub type Sig_remove_branch = (GString,);
    pub const create_remote: (&str, u32) = ("_create_remote", 3186203200u32);
    pub type Sig_create_remote = (GString, GString,);
    pub const remove_remote: (&str, u32) = ("_remove_remote", 83702148u32);
    pub type Sig_remove_remote = (GString,);
    pub const get_current_branch_name: (&str, u32) = ("_get_current_branch_name", 2841200299u32);
    pub type Sig_get_current_branch_name = ();
    pub const checkout_branch: (&str, u32) = ("_checkout_branch", 2323990056u32);
    pub type Sig_checkout_branch = (GString,);
    pub const pull: (&str, u32) = ("_pull", 83702148u32);
    pub type Sig_pull = (GString,);
    pub const push: (&str, u32) = ("_push", 2678287736u32);
    pub type Sig_push = (GString, bool,);
    pub const fetch: (&str, u32) = ("_fetch", 83702148u32);
    pub type Sig_fetch = (GString,);
    pub const get_line_diff: (&str, u32) = ("_get_line_diff", 2796572089u32);
    pub type Sig_get_line_diff = (GString, GString,);
    
}
pub mod EncodedObjectAsId {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Engine {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EngineDebugger {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod EngineProfiler {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const toggle: (&str, u32) = ("_toggle", 1157301103u32);
    pub type Sig_toggle = (bool, VarArray,);
    pub const add_frame: (&str, u32) = ("_add_frame", 381264803u32);
    pub type Sig_add_frame = (VarArray,);
    pub const tick: (&str, u32) = ("_tick", 3948312143u32);
    pub type Sig_tick = (f64, f64, f64, f64,);
    
}
pub mod Environment {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Expression {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ExternalTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FbxDocument {
    pub use super::GltfDocument::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FbxState {
    pub use super::GltfState::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FastNoiseLite {
    pub use super::Noise::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FileAccess {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FileDialog {
    pub use super::ConfirmationDialog::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FileSystemDock {
    pub use super::VBoxContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FlowContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FogMaterial {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FogVolume {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FoldableContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FoldableGroup {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Font {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FontFile {
    pub use super::Font::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FontVariation {
    pub use super::Font::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod FramebufferCacheRd {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GDExtension {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GDExtensionManager {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GDScript {
    pub use super::Script::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GDScriptSyntaxHighlighter {
    pub use super::EditorSyntaxHighlighter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfAccessor {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfAnimation {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfBufferView {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfCamera {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfDocument {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfDocumentExtension {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const import_preflight: (&str, u32) = ("_import_preflight", 412946943u32);
    pub type Sig_import_preflight = (Option < Gd < crate::classes::GltfState > >, PackedStringArray,);
    pub const get_supported_extensions: (&str, u32) = ("_get_supported_extensions", 2981934095u32);
    pub type Sig_get_supported_extensions = ();
    pub const parse_node_extensions: (&str, u32) = ("_parse_node_extensions", 2067053794u32);
    pub type Sig_parse_node_extensions = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::GltfNode > >, VarDictionary,);
    pub const parse_image_data: (&str, u32) = ("_parse_image_data", 3201673288u32);
    pub type Sig_parse_image_data = (Option < Gd < crate::classes::GltfState > >, PackedByteArray, GString, Option < Gd < crate::classes::Image > >,);
    pub const get_image_file_extension: (&str, u32) = ("_get_image_file_extension", 2841200299u32);
    pub type Sig_get_image_file_extension = ();
    pub const parse_texture_json: (&str, u32) = ("_parse_texture_json", 1624327185u32);
    pub type Sig_parse_texture_json = (Option < Gd < crate::classes::GltfState > >, VarDictionary, Option < Gd < crate::classes::GltfTexture > >,);
    pub const import_object_model_property: (&str, u32) = ("_import_object_model_property", 1446147484u32);
    pub type Sig_import_object_model_property = (Option < Gd < crate::classes::GltfState > >, PackedStringArray, Array < NodePath >,);
    pub const import_post_parse: (&str, u32) = ("_import_post_parse", 1704600462u32);
    pub type Sig_import_post_parse = (Option < Gd < crate::classes::GltfState > >,);
    pub const import_pre_generate: (&str, u32) = ("_import_pre_generate", 1704600462u32);
    pub type Sig_import_pre_generate = (Option < Gd < crate::classes::GltfState > >,);
    pub const generate_scene_node: (&str, u32) = ("_generate_scene_node", 3810899026u32);
    pub type Sig_generate_scene_node = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::GltfNode > >, Option < Gd < crate::classes::Node > >,);
    pub const import_node: (&str, u32) = ("_import_node", 4064279746u32);
    pub type Sig_import_node = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::GltfNode > >, VarDictionary, Option < Gd < crate::classes::Node > >,);
    pub const import_post: (&str, u32) = ("_import_post", 295478427u32);
    pub type Sig_import_post = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::Node > >,);
    pub const export_preflight: (&str, u32) = ("_export_preflight", 295478427u32);
    pub type Sig_export_preflight = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::Node > >,);
    pub const convert_scene_node: (&str, u32) = ("_convert_scene_node", 147612932u32);
    pub type Sig_convert_scene_node = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::GltfNode > >, Option < Gd < crate::classes::Node > >,);
    pub const export_post_convert: (&str, u32) = ("_export_post_convert", 295478427u32);
    pub type Sig_export_post_convert = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::Node > >,);
    pub const export_preserialize: (&str, u32) = ("_export_preserialize", 1704600462u32);
    pub type Sig_export_preserialize = (Option < Gd < crate::classes::GltfState > >,);
    pub const export_object_model_property: (&str, u32) = ("_export_object_model_property", 4111022730u32);
    pub type Sig_export_object_model_property = (Option < Gd < crate::classes::GltfState > >, NodePath, Option < Gd < crate::classes::Node > >, i32, Option < Gd < crate::classes::Object > >, i32,);
    pub const get_saveable_image_formats: (&str, u32) = ("_get_saveable_image_formats", 2981934095u32);
    pub type Sig_get_saveable_image_formats = ();
    pub const serialize_image_to_bytes: (&str, u32) = ("_serialize_image_to_bytes", 276886664u32);
    pub type Sig_serialize_image_to_bytes = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::Image > >, VarDictionary, GString, f32,);
    pub const save_image_at_path: (&str, u32) = ("_save_image_at_path", 1844337242u32);
    pub type Sig_save_image_at_path = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::Image > >, GString, GString, f32,);
    pub const serialize_texture_json: (&str, u32) = ("_serialize_texture_json", 2565166506u32);
    pub type Sig_serialize_texture_json = (Option < Gd < crate::classes::GltfState > >, VarDictionary, Option < Gd < crate::classes::GltfTexture > >, GString,);
    pub const export_node: (&str, u32) = ("_export_node", 4064279746u32);
    pub type Sig_export_node = (Option < Gd < crate::classes::GltfState > >, Option < Gd < crate::classes::GltfNode > >, VarDictionary, Option < Gd < crate::classes::Node > >,);
    pub const export_post: (&str, u32) = ("_export_post", 1704600462u32);
    pub type Sig_export_post = (Option < Gd < crate::classes::GltfState > >,);
    
}
pub mod GltfDocumentExtensionConvertImporterMesh {
    pub use super::GltfDocumentExtension::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfLight {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfMesh {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfNode {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfObjectModelProperty {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfPhysicsBody {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfPhysicsShape {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfSkeleton {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfSkin {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfSpecGloss {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfState {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfTexture {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GltfTextureSampler {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticles2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticles3D {
    pub use super::GeometryInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesAttractor3D {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesAttractorBox3D {
    pub use super::GpuParticlesAttractor3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesAttractorSphere3D {
    pub use super::GpuParticlesAttractor3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesAttractorVectorField3D {
    pub use super::GpuParticlesAttractor3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesCollision3D {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesCollisionBox3D {
    pub use super::GpuParticlesCollision3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesCollisionHeightField3D {
    pub use super::GpuParticlesCollision3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesCollisionSdf3d {
    pub use super::GpuParticlesCollision3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GpuParticlesCollisionSphere3D {
    pub use super::GpuParticlesCollision3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Generic6DofJoint3D {
    pub use super::Joint3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Geometry2D {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Geometry3D {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GeometryInstance3D {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Gradient {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GradientTexture1D {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GradientTexture2D {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GridContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GridMap {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GridMapEditorPlugin {
    pub use super::EditorPlugin::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod GrooveJoint2D {
    pub use super::Joint2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HBoxContainer {
    pub use super::BoxContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HFlowContainer {
    pub use super::FlowContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HmacContext {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HScrollBar {
    pub use super::ScrollBar::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HSeparator {
    pub use super::Separator::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HSlider {
    pub use super::Slider::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HSplitContainer {
    pub use super::SplitContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HttpClient {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HttpRequest {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HashingContext {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HeightMapShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod HingeJoint3D {
    pub use super::Joint3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Ip {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Image {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ImageFormatLoader {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ImageFormatLoaderExtension {
    pub use super::ImageFormatLoader::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_recognized_extensions: (&str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub type Sig_get_recognized_extensions = ();
    pub const load_image: (&str, u32) = ("_load_image", 3760540541u32);
    pub type Sig_load_image = (Option < Gd < crate::classes::Image > >, Option < Gd < crate::classes::FileAccess > >, crate::classes::image_format_loader::LoaderFlags, f32,);
    
}
pub mod ImageTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ImageTexture3D {
    pub use super::Texture3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ImageTextureLayered {
    pub use super::TextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ImmediateMesh {
    pub use super::Mesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ImporterMesh {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ImporterMeshInstance3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Input {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEvent {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventAction {
    pub use super::InputEvent::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventFromWindow {
    pub use super::InputEvent::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventGesture {
    pub use super::InputEventWithModifiers::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventJoypadButton {
    pub use super::InputEvent::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventJoypadMotion {
    pub use super::InputEvent::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventKey {
    pub use super::InputEventWithModifiers::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventMidi {
    pub use super::InputEvent::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventMagnifyGesture {
    pub use super::InputEventGesture::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventMouse {
    pub use super::InputEventWithModifiers::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventMouseButton {
    pub use super::InputEventMouse::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventMouseMotion {
    pub use super::InputEventMouse::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventPanGesture {
    pub use super::InputEventGesture::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventScreenDrag {
    pub use super::InputEventFromWindow::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventScreenTouch {
    pub use super::InputEventFromWindow::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventShortcut {
    pub use super::InputEvent::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputEventWithModifiers {
    pub use super::InputEventFromWindow::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InputMap {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod InstancePlaceholder {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod IntervalTweener {
    pub use super::Tweener::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ItemList {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Json {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod JsonRpc {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Joint2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Joint3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod KinematicCollision2D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod KinematicCollision3D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Label {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Label3D {
    pub use super::GeometryInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod LabelSettings {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Light2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Light3D {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod LightOccluder2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod LightmapGi {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod LightmapGiData {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod LightmapProbe {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Lightmapper {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod LightmapperRd {
    pub use super::Lightmapper::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Line2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod LineEdit {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod LinkButton {
    pub use super::BaseButton::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Logger {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const log_error: (&str, u32) = ("_log_error", 27079556u32);
    pub type Sig_log_error = (GString, GString, i32, GString, GString, bool, i32, Array < Gd < crate::classes::ScriptBacktrace > >,);
    pub const log_message: (&str, u32) = ("_log_message", 2678287736u32);
    pub type Sig_log_message = (GString, bool,);
    
}
pub mod LookAtModifier3D {
    pub use super::SkeletonModifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MainLoop {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const initialize: (&str, u32) = ("_initialize", 3218959716u32);
    pub type Sig_initialize = ();
    pub const physics_process: (&str, u32) = ("_physics_process", 330693286u32);
    pub type Sig_physics_process = (f64,);
    pub const process: (&str, u32) = ("_process", 330693286u32);
    pub type Sig_process = (f64,);
    pub const finalize: (&str, u32) = ("_finalize", 3218959716u32);
    pub type Sig_finalize = ();
    
}
pub mod MarginContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Marker2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Marker3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Marshalls {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Material {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_shader_rid: (&str, u32) = ("_get_shader_rid", 2944877500u32);
    pub type Sig_get_shader_rid = ();
    pub const get_shader_mode: (&str, u32) = ("_get_shader_mode", 3392948163u32);
    pub type Sig_get_shader_mode = ();
    pub const can_do_next_pass: (&str, u32) = ("_can_do_next_pass", 36873697u32);
    pub type Sig_can_do_next_pass = ();
    pub const can_use_render_priority: (&str, u32) = ("_can_use_render_priority", 36873697u32);
    pub type Sig_can_use_render_priority = ();
    
}
pub mod MenuBar {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MenuButton {
    pub use super::Button::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Mesh {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_surface_count: (&str, u32) = ("_get_surface_count", 3905245786u32);
    pub type Sig_get_surface_count = ();
    pub const surface_get_array_len: (&str, u32) = ("_surface_get_array_len", 923996154u32);
    pub type Sig_surface_get_array_len = (i32,);
    pub const surface_get_array_index_len: (&str, u32) = ("_surface_get_array_index_len", 923996154u32);
    pub type Sig_surface_get_array_index_len = (i32,);
    pub const surface_get_arrays: (&str, u32) = ("_surface_get_arrays", 663333327u32);
    pub type Sig_surface_get_arrays = (i32,);
    pub const surface_get_blend_shape_arrays: (&str, u32) = ("_surface_get_blend_shape_arrays", 663333327u32);
    pub type Sig_surface_get_blend_shape_arrays = (i32,);
    pub const surface_get_lods: (&str, u32) = ("_surface_get_lods", 3485342025u32);
    pub type Sig_surface_get_lods = (i32,);
    pub const surface_get_format: (&str, u32) = ("_surface_get_format", 923996154u32);
    pub type Sig_surface_get_format = (i32,);
    pub const surface_get_primitive_type: (&str, u32) = ("_surface_get_primitive_type", 923996154u32);
    pub type Sig_surface_get_primitive_type = (i32,);
    pub const surface_set_material: (&str, u32) = ("_surface_set_material", 3671737478u32);
    pub type Sig_surface_set_material = (i32, Option < Gd < crate::classes::Material > >,);
    pub const surface_get_material: (&str, u32) = ("_surface_get_material", 2897466400u32);
    pub type Sig_surface_get_material = (i32,);
    pub const get_blend_shape_count: (&str, u32) = ("_get_blend_shape_count", 3905245786u32);
    pub type Sig_get_blend_shape_count = ();
    pub const get_blend_shape_name: (&str, u32) = ("_get_blend_shape_name", 659327637u32);
    pub type Sig_get_blend_shape_name = (i32,);
    pub const set_blend_shape_name: (&str, u32) = ("_set_blend_shape_name", 3780747571u32);
    pub type Sig_set_blend_shape_name = (i32, StringName,);
    pub const get_aabb: (&str, u32) = ("_get_aabb", 1068685055u32);
    pub type Sig_get_aabb = ();
    
}
pub mod MeshConvexDecompositionSettings {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MeshDataTool {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MeshInstance2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MeshInstance3D {
    pub use super::GeometryInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MeshLibrary {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MeshTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MethodTweener {
    pub use super::Tweener::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MissingNode {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MissingResource {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MobileVrInterface {
    pub use super::XrInterface::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ModifierBoneTarget3D {
    pub use super::SkeletonModifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MovieWriter {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_audio_mix_rate: (&str, u32) = ("_get_audio_mix_rate", 3905245786u32);
    pub type Sig_get_audio_mix_rate = ();
    pub const get_audio_speaker_mode: (&str, u32) = ("_get_audio_speaker_mode", 2549190337u32);
    pub type Sig_get_audio_speaker_mode = ();
    pub const handles_file: (&str, u32) = ("_handles_file", 3927539163u32);
    pub type Sig_handles_file = (GString,);
    pub const write_begin: (&str, u32) = ("_write_begin", 1866453460u32);
    pub type Sig_write_begin = (Vector2i, u32, GString,);
    pub const write_frame_rawptr: (&str, u32) = ("_write_frame", 2784607037u32);
    pub type Sig_write_frame_rawptr = (Option < Gd < crate::classes::Image > >, * const c_void,);
    pub const write_end: (&str, u32) = ("_write_end", 3218959716u32);
    pub type Sig_write_end = ();
    
}
pub mod MultiMesh {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MultiMeshInstance2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MultiMeshInstance3D {
    pub use super::GeometryInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MultiplayerApi {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MultiplayerApiExtension {
    pub use super::MultiplayerApi::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const poll: (&str, u32) = ("_poll", 166280745u32);
    pub type Sig_poll = ();
    pub const set_multiplayer_peer: (&str, u32) = ("_set_multiplayer_peer", 3694835298u32);
    pub type Sig_set_multiplayer_peer = (Option < Gd < crate::classes::MultiplayerPeer > >,);
    pub const get_multiplayer_peer: (&str, u32) = ("_get_multiplayer_peer", 3223692825u32);
    pub type Sig_get_multiplayer_peer = ();
    pub const get_unique_id: (&str, u32) = ("_get_unique_id", 3905245786u32);
    pub type Sig_get_unique_id = ();
    pub const get_peer_ids: (&str, u32) = ("_get_peer_ids", 1930428628u32);
    pub type Sig_get_peer_ids = ();
    pub const rpc: (&str, u32) = ("_rpc", 3673574758u32);
    pub type Sig_rpc = (i32, Option < Gd < crate::classes::Object > >, StringName, VarArray,);
    pub const get_remote_sender_id: (&str, u32) = ("_get_remote_sender_id", 3905245786u32);
    pub type Sig_get_remote_sender_id = ();
    pub const object_configuration_add: (&str, u32) = ("_object_configuration_add", 1171879464u32);
    pub type Sig_object_configuration_add = (Option < Gd < crate::classes::Object > >, Variant,);
    pub const object_configuration_remove: (&str, u32) = ("_object_configuration_remove", 1171879464u32);
    pub type Sig_object_configuration_remove = (Option < Gd < crate::classes::Object > >, Variant,);
    
}
pub mod MultiplayerPeer {
    pub use super::PacketPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MultiplayerPeerExtension {
    pub use super::MultiplayerPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_packet_rawptr: (&str, u32) = ("_get_packet", 3099858825u32);
    pub type Sig_get_packet_rawptr = (* mut * const u8, * mut i32,);
    pub const put_packet_rawptr: (&str, u32) = ("_put_packet", 3099858825u32);
    pub type Sig_put_packet_rawptr = (* const u8, i32,);
    pub const get_available_packet_count: (&str, u32) = ("_get_available_packet_count", 3905245786u32);
    pub type Sig_get_available_packet_count = ();
    pub const get_max_packet_size: (&str, u32) = ("_get_max_packet_size", 3905245786u32);
    pub type Sig_get_max_packet_size = ();
    pub const get_packet_script: (&str, u32) = ("_get_packet_script", 2115431945u32);
    pub type Sig_get_packet_script = ();
    pub const put_packet_script: (&str, u32) = ("_put_packet_script", 680677267u32);
    pub type Sig_put_packet_script = (PackedByteArray,);
    pub const get_packet_channel: (&str, u32) = ("_get_packet_channel", 3905245786u32);
    pub type Sig_get_packet_channel = ();
    pub const get_packet_mode: (&str, u32) = ("_get_packet_mode", 3369852622u32);
    pub type Sig_get_packet_mode = ();
    pub const set_transfer_channel: (&str, u32) = ("_set_transfer_channel", 1286410249u32);
    pub type Sig_set_transfer_channel = (i32,);
    pub const get_transfer_channel: (&str, u32) = ("_get_transfer_channel", 3905245786u32);
    pub type Sig_get_transfer_channel = ();
    pub const set_transfer_mode: (&str, u32) = ("_set_transfer_mode", 950411049u32);
    pub type Sig_set_transfer_mode = (crate::classes::multiplayer_peer::TransferMode,);
    pub const get_transfer_mode: (&str, u32) = ("_get_transfer_mode", 3369852622u32);
    pub type Sig_get_transfer_mode = ();
    pub const set_target_peer: (&str, u32) = ("_set_target_peer", 1286410249u32);
    pub type Sig_set_target_peer = (i32,);
    pub const get_packet_peer: (&str, u32) = ("_get_packet_peer", 3905245786u32);
    pub type Sig_get_packet_peer = ();
    pub const is_server: (&str, u32) = ("_is_server", 36873697u32);
    pub type Sig_is_server = ();
    pub const poll: (&str, u32) = ("_poll", 3218959716u32);
    pub type Sig_poll = ();
    pub const close: (&str, u32) = ("_close", 3218959716u32);
    pub type Sig_close = ();
    pub const disconnect_peer: (&str, u32) = ("_disconnect_peer", 300928843u32);
    pub type Sig_disconnect_peer = (i32, bool,);
    pub const get_unique_id: (&str, u32) = ("_get_unique_id", 3905245786u32);
    pub type Sig_get_unique_id = ();
    pub const set_refuse_new_connections: (&str, u32) = ("_set_refuse_new_connections", 2586408642u32);
    pub type Sig_set_refuse_new_connections = (bool,);
    pub const is_refusing_new_connections: (&str, u32) = ("_is_refusing_new_connections", 36873697u32);
    pub type Sig_is_refusing_new_connections = ();
    pub const is_server_relay_supported: (&str, u32) = ("_is_server_relay_supported", 36873697u32);
    pub type Sig_is_server_relay_supported = ();
    pub const get_connection_status: (&str, u32) = ("_get_connection_status", 2147374275u32);
    pub type Sig_get_connection_status = ();
    
}
pub mod MultiplayerSpawner {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod MultiplayerSynchronizer {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod NativeMenu {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod NavigationMeshGenerator {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod NinePatchRect {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Node {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const process: (&str, u32) = ("_process", 373806689u32);
    pub type Sig_process = (f64,);
    pub const physics_process: (&str, u32) = ("_physics_process", 373806689u32);
    pub type Sig_physics_process = (f64,);
    pub const enter_tree: (&str, u32) = ("_enter_tree", 3218959716u32);
    pub type Sig_enter_tree = ();
    pub const exit_tree: (&str, u32) = ("_exit_tree", 3218959716u32);
    pub type Sig_exit_tree = ();
    pub const ready: (&str, u32) = ("_ready", 3218959716u32);
    pub type Sig_ready = ();
    pub const get_configuration_warnings: (&str, u32) = ("_get_configuration_warnings", 1139954409u32);
    pub type Sig_get_configuration_warnings = ();
    pub const get_accessibility_configuration_warnings: (&str, u32) = ("_get_accessibility_configuration_warnings", 1139954409u32);
    pub type Sig_get_accessibility_configuration_warnings = ();
    pub const input: (&str, u32) = ("_input", 3754044979u32);
    pub type Sig_input = (Gd < crate::classes::InputEvent >,);
    pub const shortcut_input: (&str, u32) = ("_shortcut_input", 3754044979u32);
    pub type Sig_shortcut_input = (Gd < crate::classes::InputEvent >,);
    pub const unhandled_input: (&str, u32) = ("_unhandled_input", 3754044979u32);
    pub type Sig_unhandled_input = (Gd < crate::classes::InputEvent >,);
    pub const unhandled_key_input: (&str, u32) = ("_unhandled_key_input", 3754044979u32);
    pub type Sig_unhandled_key_input = (Gd < crate::classes::InputEvent >,);
    pub const get_focused_accessibility_element: (&str, u32) = ("_get_focused_accessibility_element", 2944877500u32);
    pub type Sig_get_focused_accessibility_element = ();
    
}
pub mod Node2D {
    pub use super::CanvasItem::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Node3D {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Node3DGizmo {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Noise {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod NoiseTexture2D {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod NoiseTexture3D {
    pub use super::Texture3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OrmMaterial3D {
    pub use super::BaseMaterial3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Os {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Object {
    
}
pub mod Occluder3D {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OccluderInstance3D {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OccluderPolygon2D {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OfflineMultiplayerPeer {
    pub use super::MultiplayerPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OggPacketSequence {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OggPacketSequencePlayback {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OmniLight3D {
    pub use super::Light3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrApiExtension {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrAction {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrActionBindingModifier {
    pub use super::OpenXrBindingModifier::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrActionMap {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrActionSet {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrAnalogThresholdModifier {
    pub use super::OpenXrActionBindingModifier::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrBindingModifier {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_description: (&str, u32) = ("_get_description", 201670096u32);
    pub type Sig_get_description = ();
    pub const get_ip_modification: (&str, u32) = ("_get_ip_modification", 2115431945u32);
    pub type Sig_get_ip_modification = ();
    
}
pub mod OpenXrBindingModifierEditor {
    pub use super::PanelContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrCompositionLayer {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrCompositionLayerCylinder {
    pub use super::OpenXrCompositionLayer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrCompositionLayerEquirect {
    pub use super::OpenXrCompositionLayer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrCompositionLayerQuad {
    pub use super::OpenXrCompositionLayer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrDpadBindingModifier {
    pub use super::OpenXripBindingModifier::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrExtensionWrapper {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_requested_extensions: (&str, u32) = ("_get_requested_extensions", 2382534195u32);
    pub type Sig_get_requested_extensions = ();
    pub const set_system_properties_and_get_next_pointer_rawptr: (&str, u32) = ("_set_system_properties_and_get_next_pointer", 3744713108u32);
    pub type Sig_set_system_properties_and_get_next_pointer_rawptr = (* mut c_void,);
    pub const set_instance_create_info_and_get_next_pointer_rawptr: (&str, u32) = ("_set_instance_create_info_and_get_next_pointer", 3744713108u32);
    pub type Sig_set_instance_create_info_and_get_next_pointer_rawptr = (* mut c_void,);
    pub const set_session_create_and_get_next_pointer_rawptr: (&str, u32) = ("_set_session_create_and_get_next_pointer", 3744713108u32);
    pub type Sig_set_session_create_and_get_next_pointer_rawptr = (* mut c_void,);
    pub const set_swapchain_create_info_and_get_next_pointer_rawptr: (&str, u32) = ("_set_swapchain_create_info_and_get_next_pointer", 3744713108u32);
    pub type Sig_set_swapchain_create_info_and_get_next_pointer_rawptr = (* mut c_void,);
    pub const set_hand_joint_locations_and_get_next_pointer_rawptr: (&str, u32) = ("_set_hand_joint_locations_and_get_next_pointer", 50157827u32);
    pub type Sig_set_hand_joint_locations_and_get_next_pointer_rawptr = (i32, * mut c_void,);
    pub const set_projection_views_and_get_next_pointer_rawptr: (&str, u32) = ("_set_projection_views_and_get_next_pointer", 50157827u32);
    pub type Sig_set_projection_views_and_get_next_pointer_rawptr = (i32, * mut c_void,);
    pub const set_frame_wait_info_and_get_next_pointer_rawptr: (&str, u32) = ("_set_frame_wait_info_and_get_next_pointer", 3744713108u32);
    pub type Sig_set_frame_wait_info_and_get_next_pointer_rawptr = (* mut c_void,);
    pub const set_frame_end_info_and_get_next_pointer_rawptr: (&str, u32) = ("_set_frame_end_info_and_get_next_pointer", 3744713108u32);
    pub type Sig_set_frame_end_info_and_get_next_pointer_rawptr = (* mut c_void,);
    pub const set_view_locate_info_and_get_next_pointer_rawptr: (&str, u32) = ("_set_view_locate_info_and_get_next_pointer", 3744713108u32);
    pub type Sig_set_view_locate_info_and_get_next_pointer_rawptr = (* mut c_void,);
    pub const set_reference_space_create_info_and_get_next_pointer_rawptr: (&str, u32) = ("_set_reference_space_create_info_and_get_next_pointer", 50157827u32);
    pub type Sig_set_reference_space_create_info_and_get_next_pointer_rawptr = (i32, * mut c_void,);
    pub const get_composition_layer_count: (&str, u32) = ("_get_composition_layer_count", 2455072627u32);
    pub type Sig_get_composition_layer_count = ();
    pub const get_composition_layer: (&str, u32) = ("_get_composition_layer", 3744713108u32);
    pub type Sig_get_composition_layer = (i32,);
    pub const get_composition_layer_order: (&str, u32) = ("_get_composition_layer_order", 3744713108u32);
    pub type Sig_get_composition_layer_order = (i32,);
    pub const get_suggested_tracker_names: (&str, u32) = ("_get_suggested_tracker_names", 2981934095u32);
    pub type Sig_get_suggested_tracker_names = ();
    pub const on_register_metadata: (&str, u32) = ("_on_register_metadata", 3218959716u32);
    pub type Sig_on_register_metadata = ();
    pub const on_before_instance_created: (&str, u32) = ("_on_before_instance_created", 3218959716u32);
    pub type Sig_on_before_instance_created = ();
    pub const on_instance_created: (&str, u32) = ("_on_instance_created", 1286410249u32);
    pub type Sig_on_instance_created = (u64,);
    pub const on_instance_destroyed: (&str, u32) = ("_on_instance_destroyed", 3218959716u32);
    pub type Sig_on_instance_destroyed = ();
    pub const on_session_created: (&str, u32) = ("_on_session_created", 1286410249u32);
    pub type Sig_on_session_created = (u64,);
    pub const on_process: (&str, u32) = ("_on_process", 3218959716u32);
    pub type Sig_on_process = ();
    pub const on_sync_actions: (&str, u32) = ("_on_sync_actions", 3218959716u32);
    pub type Sig_on_sync_actions = ();
    pub const on_pre_render: (&str, u32) = ("_on_pre_render", 3218959716u32);
    pub type Sig_on_pre_render = ();
    pub const on_main_swapchains_created: (&str, u32) = ("_on_main_swapchains_created", 3218959716u32);
    pub type Sig_on_main_swapchains_created = ();
    pub const on_pre_draw_viewport: (&str, u32) = ("_on_pre_draw_viewport", 2722037293u32);
    pub type Sig_on_pre_draw_viewport = (Rid,);
    pub const on_post_draw_viewport: (&str, u32) = ("_on_post_draw_viewport", 2722037293u32);
    pub type Sig_on_post_draw_viewport = (Rid,);
    pub const on_session_destroyed: (&str, u32) = ("_on_session_destroyed", 3218959716u32);
    pub type Sig_on_session_destroyed = ();
    pub const on_state_idle: (&str, u32) = ("_on_state_idle", 3218959716u32);
    pub type Sig_on_state_idle = ();
    pub const on_state_ready: (&str, u32) = ("_on_state_ready", 3218959716u32);
    pub type Sig_on_state_ready = ();
    pub const on_state_synchronized: (&str, u32) = ("_on_state_synchronized", 3218959716u32);
    pub type Sig_on_state_synchronized = ();
    pub const on_state_visible: (&str, u32) = ("_on_state_visible", 3218959716u32);
    pub type Sig_on_state_visible = ();
    pub const on_state_focused: (&str, u32) = ("_on_state_focused", 3218959716u32);
    pub type Sig_on_state_focused = ();
    pub const on_state_stopping: (&str, u32) = ("_on_state_stopping", 3218959716u32);
    pub type Sig_on_state_stopping = ();
    pub const on_state_loss_pending: (&str, u32) = ("_on_state_loss_pending", 3218959716u32);
    pub type Sig_on_state_loss_pending = ();
    pub const on_state_exiting: (&str, u32) = ("_on_state_exiting", 3218959716u32);
    pub type Sig_on_state_exiting = ();
    pub const on_event_polled_rawptr: (&str, u32) = ("_on_event_polled", 3067735520u32);
    pub type Sig_on_event_polled_rawptr = (* const c_void,);
    pub const set_viewport_composition_layer_and_get_next_pointer_rawptr: (&str, u32) = ("_set_viewport_composition_layer_and_get_next_pointer", 2250464348u32);
    pub type Sig_set_viewport_composition_layer_and_get_next_pointer_rawptr = (* const c_void, VarDictionary, * mut c_void,);
    pub const get_viewport_composition_layer_extension_properties: (&str, u32) = ("_get_viewport_composition_layer_extension_properties", 2915620761u32);
    pub type Sig_get_viewport_composition_layer_extension_properties = ();
    pub const get_viewport_composition_layer_extension_property_defaults: (&str, u32) = ("_get_viewport_composition_layer_extension_property_defaults", 2382534195u32);
    pub type Sig_get_viewport_composition_layer_extension_property_defaults = ();
    pub const on_viewport_composition_layer_destroyed_rawptr: (&str, u32) = ("_on_viewport_composition_layer_destroyed", 1286410249u32);
    pub type Sig_on_viewport_composition_layer_destroyed_rawptr = (* const c_void,);
    pub const set_android_surface_swapchain_create_info_and_get_next_pointer_rawptr: (&str, u32) = ("_set_android_surface_swapchain_create_info_and_get_next_pointer", 3726637545u32);
    pub type Sig_set_android_surface_swapchain_create_info_and_get_next_pointer_rawptr = (VarDictionary, * mut c_void,);
    
}
pub mod OpenXrExtensionWrapperExtension {
    pub use super::OpenXrExtensionWrapper::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrFutureExtension {
    pub use super::OpenXrExtensionWrapper::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrFutureResult {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrHand {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrHapticBase {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrHapticVibration {
    pub use super::OpenXrHapticBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrIpBinding {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXripBindingModifier {
    pub use super::OpenXrBindingModifier::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrInteractionProfile {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrInteractionProfileEditor {
    pub use super::OpenXrInteractionProfileEditorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrInteractionProfileEditorBase {
    pub use super::HBoxContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrInteractionProfileMetadata {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrInterface {
    pub use super::XrInterface::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrRenderModel {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrRenderModelExtension {
    pub use super::OpenXrExtensionWrapper::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrRenderModelManager {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OpenXrVisibilityMask {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OptimizedTranslation {
    pub use super::Translation::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod OptionButton {
    pub use super::Button::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PckPacker {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PackedDataContainer {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PackedDataContainerRef {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PackedScene {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PacketPeer {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PacketPeerDtls {
    pub use super::PacketPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PacketPeerExtension {
    pub use super::PacketPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_packet_rawptr: (&str, u32) = ("_get_packet", 3099858825u32);
    pub type Sig_get_packet_rawptr = (* mut * const u8, * mut i32,);
    pub const put_packet_rawptr: (&str, u32) = ("_put_packet", 3099858825u32);
    pub type Sig_put_packet_rawptr = (* const u8, i32,);
    pub const get_available_packet_count: (&str, u32) = ("_get_available_packet_count", 3905245786u32);
    pub type Sig_get_available_packet_count = ();
    pub const get_max_packet_size: (&str, u32) = ("_get_max_packet_size", 3905245786u32);
    pub type Sig_get_max_packet_size = ();
    
}
pub mod PacketPeerStream {
    pub use super::PacketPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PacketPeerUdp {
    pub use super::PacketPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Panel {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PanelContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PanoramaSkyMaterial {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ParallaxBackground {
    pub use super::CanvasLayer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ParallaxLayer {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ParticleProcessMaterial {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Path2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Path3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PathFollow2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PathFollow3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Performance {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicalBone2D {
    pub use super::RigidBody2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicalBone3D {
    pub use super::PhysicsBody3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const integrate_forces: (&str, u32) = ("_integrate_forces", 420958145u32);
    pub type Sig_integrate_forces = (Option < Gd < crate::classes::PhysicsDirectBodyState3D > >,);
    
}
pub mod PhysicalBoneSimulator3D {
    pub use super::SkeletonModifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicalSkyMaterial {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsBody2D {
    pub use super::CollisionObject2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsBody3D {
    pub use super::CollisionObject3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsDirectBodyState2D {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsDirectBodyState2DExtension {
    pub use super::PhysicsDirectBodyState2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_total_gravity: (&str, u32) = ("_get_total_gravity", 3341600327u32);
    pub type Sig_get_total_gravity = ();
    pub const get_total_linear_damp: (&str, u32) = ("_get_total_linear_damp", 1740695150u32);
    pub type Sig_get_total_linear_damp = ();
    pub const get_total_angular_damp: (&str, u32) = ("_get_total_angular_damp", 1740695150u32);
    pub type Sig_get_total_angular_damp = ();
    pub const get_center_of_mass: (&str, u32) = ("_get_center_of_mass", 3341600327u32);
    pub type Sig_get_center_of_mass = ();
    pub const get_center_of_mass_local: (&str, u32) = ("_get_center_of_mass_local", 3341600327u32);
    pub type Sig_get_center_of_mass_local = ();
    pub const get_inverse_mass: (&str, u32) = ("_get_inverse_mass", 1740695150u32);
    pub type Sig_get_inverse_mass = ();
    pub const get_inverse_inertia: (&str, u32) = ("_get_inverse_inertia", 1740695150u32);
    pub type Sig_get_inverse_inertia = ();
    pub const set_linear_velocity: (&str, u32) = ("_set_linear_velocity", 743155724u32);
    pub type Sig_set_linear_velocity = (Vector2,);
    pub const get_linear_velocity: (&str, u32) = ("_get_linear_velocity", 3341600327u32);
    pub type Sig_get_linear_velocity = ();
    pub const set_angular_velocity: (&str, u32) = ("_set_angular_velocity", 373806689u32);
    pub type Sig_set_angular_velocity = (f32,);
    pub const get_angular_velocity: (&str, u32) = ("_get_angular_velocity", 1740695150u32);
    pub type Sig_get_angular_velocity = ();
    pub const set_transform: (&str, u32) = ("_set_transform", 2761652528u32);
    pub type Sig_set_transform = (Transform2D,);
    pub const get_transform: (&str, u32) = ("_get_transform", 3814499831u32);
    pub type Sig_get_transform = ();
    pub const get_velocity_at_local_position: (&str, u32) = ("_get_velocity_at_local_position", 2656412154u32);
    pub type Sig_get_velocity_at_local_position = (Vector2,);
    pub const apply_central_impulse: (&str, u32) = ("_apply_central_impulse", 743155724u32);
    pub type Sig_apply_central_impulse = (Vector2,);
    pub const apply_impulse: (&str, u32) = ("_apply_impulse", 3108078480u32);
    pub type Sig_apply_impulse = (Vector2, Vector2,);
    pub const apply_torque_impulse: (&str, u32) = ("_apply_torque_impulse", 373806689u32);
    pub type Sig_apply_torque_impulse = (f32,);
    pub const apply_central_force: (&str, u32) = ("_apply_central_force", 743155724u32);
    pub type Sig_apply_central_force = (Vector2,);
    pub const apply_force: (&str, u32) = ("_apply_force", 3108078480u32);
    pub type Sig_apply_force = (Vector2, Vector2,);
    pub const apply_torque: (&str, u32) = ("_apply_torque", 373806689u32);
    pub type Sig_apply_torque = (f32,);
    pub const add_constant_central_force: (&str, u32) = ("_add_constant_central_force", 743155724u32);
    pub type Sig_add_constant_central_force = (Vector2,);
    pub const add_constant_force: (&str, u32) = ("_add_constant_force", 3108078480u32);
    pub type Sig_add_constant_force = (Vector2, Vector2,);
    pub const add_constant_torque: (&str, u32) = ("_add_constant_torque", 373806689u32);
    pub type Sig_add_constant_torque = (f32,);
    pub const set_constant_force: (&str, u32) = ("_set_constant_force", 743155724u32);
    pub type Sig_set_constant_force = (Vector2,);
    pub const get_constant_force: (&str, u32) = ("_get_constant_force", 3341600327u32);
    pub type Sig_get_constant_force = ();
    pub const set_constant_torque: (&str, u32) = ("_set_constant_torque", 373806689u32);
    pub type Sig_set_constant_torque = (f32,);
    pub const get_constant_torque: (&str, u32) = ("_get_constant_torque", 1740695150u32);
    pub type Sig_get_constant_torque = ();
    pub const set_sleep_state: (&str, u32) = ("_set_sleep_state", 2586408642u32);
    pub type Sig_set_sleep_state = (bool,);
    pub const is_sleeping: (&str, u32) = ("_is_sleeping", 36873697u32);
    pub type Sig_is_sleeping = ();
    pub const set_collision_layer: (&str, u32) = ("_set_collision_layer", 1286410249u32);
    pub type Sig_set_collision_layer = (u32,);
    pub const get_collision_layer: (&str, u32) = ("_get_collision_layer", 3905245786u32);
    pub type Sig_get_collision_layer = ();
    pub const set_collision_mask: (&str, u32) = ("_set_collision_mask", 1286410249u32);
    pub type Sig_set_collision_mask = (u32,);
    pub const get_collision_mask: (&str, u32) = ("_get_collision_mask", 3905245786u32);
    pub type Sig_get_collision_mask = ();
    pub const get_contact_count: (&str, u32) = ("_get_contact_count", 3905245786u32);
    pub type Sig_get_contact_count = ();
    pub const get_contact_local_position: (&str, u32) = ("_get_contact_local_position", 2299179447u32);
    pub type Sig_get_contact_local_position = (i32,);
    pub const get_contact_local_normal: (&str, u32) = ("_get_contact_local_normal", 2299179447u32);
    pub type Sig_get_contact_local_normal = (i32,);
    pub const get_contact_local_shape: (&str, u32) = ("_get_contact_local_shape", 923996154u32);
    pub type Sig_get_contact_local_shape = (i32,);
    pub const get_contact_local_velocity_at_position: (&str, u32) = ("_get_contact_local_velocity_at_position", 2299179447u32);
    pub type Sig_get_contact_local_velocity_at_position = (i32,);
    pub const get_contact_collider: (&str, u32) = ("_get_contact_collider", 495598643u32);
    pub type Sig_get_contact_collider = (i32,);
    pub const get_contact_collider_position: (&str, u32) = ("_get_contact_collider_position", 2299179447u32);
    pub type Sig_get_contact_collider_position = (i32,);
    pub const get_contact_collider_id: (&str, u32) = ("_get_contact_collider_id", 923996154u32);
    pub type Sig_get_contact_collider_id = (i32,);
    pub const get_contact_collider_object: (&str, u32) = ("_get_contact_collider_object", 3332903315u32);
    pub type Sig_get_contact_collider_object = (i32,);
    pub const get_contact_collider_shape: (&str, u32) = ("_get_contact_collider_shape", 923996154u32);
    pub type Sig_get_contact_collider_shape = (i32,);
    pub const get_contact_collider_velocity_at_position: (&str, u32) = ("_get_contact_collider_velocity_at_position", 2299179447u32);
    pub type Sig_get_contact_collider_velocity_at_position = (i32,);
    pub const get_contact_impulse: (&str, u32) = ("_get_contact_impulse", 2299179447u32);
    pub type Sig_get_contact_impulse = (i32,);
    pub const get_step: (&str, u32) = ("_get_step", 1740695150u32);
    pub type Sig_get_step = ();
    pub const integrate_forces: (&str, u32) = ("_integrate_forces", 3218959716u32);
    pub type Sig_integrate_forces = ();
    pub const get_space_state: (&str, u32) = ("_get_space_state", 2506717822u32);
    pub type Sig_get_space_state = ();
    
}
pub mod PhysicsDirectBodyState3D {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsDirectBodyState3DExtension {
    pub use super::PhysicsDirectBodyState3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_total_gravity: (&str, u32) = ("_get_total_gravity", 3360562783u32);
    pub type Sig_get_total_gravity = ();
    pub const get_total_linear_damp: (&str, u32) = ("_get_total_linear_damp", 1740695150u32);
    pub type Sig_get_total_linear_damp = ();
    pub const get_total_angular_damp: (&str, u32) = ("_get_total_angular_damp", 1740695150u32);
    pub type Sig_get_total_angular_damp = ();
    pub const get_center_of_mass: (&str, u32) = ("_get_center_of_mass", 3360562783u32);
    pub type Sig_get_center_of_mass = ();
    pub const get_center_of_mass_local: (&str, u32) = ("_get_center_of_mass_local", 3360562783u32);
    pub type Sig_get_center_of_mass_local = ();
    pub const get_principal_inertia_axes: (&str, u32) = ("_get_principal_inertia_axes", 2716978435u32);
    pub type Sig_get_principal_inertia_axes = ();
    pub const get_inverse_mass: (&str, u32) = ("_get_inverse_mass", 1740695150u32);
    pub type Sig_get_inverse_mass = ();
    pub const get_inverse_inertia: (&str, u32) = ("_get_inverse_inertia", 3360562783u32);
    pub type Sig_get_inverse_inertia = ();
    pub const get_inverse_inertia_tensor: (&str, u32) = ("_get_inverse_inertia_tensor", 2716978435u32);
    pub type Sig_get_inverse_inertia_tensor = ();
    pub const set_linear_velocity: (&str, u32) = ("_set_linear_velocity", 3460891852u32);
    pub type Sig_set_linear_velocity = (Vector3,);
    pub const get_linear_velocity: (&str, u32) = ("_get_linear_velocity", 3360562783u32);
    pub type Sig_get_linear_velocity = ();
    pub const set_angular_velocity: (&str, u32) = ("_set_angular_velocity", 3460891852u32);
    pub type Sig_set_angular_velocity = (Vector3,);
    pub const get_angular_velocity: (&str, u32) = ("_get_angular_velocity", 3360562783u32);
    pub type Sig_get_angular_velocity = ();
    pub const set_transform: (&str, u32) = ("_set_transform", 2952846383u32);
    pub type Sig_set_transform = (Transform3D,);
    pub const get_transform: (&str, u32) = ("_get_transform", 3229777777u32);
    pub type Sig_get_transform = ();
    pub const get_velocity_at_local_position: (&str, u32) = ("_get_velocity_at_local_position", 192990374u32);
    pub type Sig_get_velocity_at_local_position = (Vector3,);
    pub const apply_central_impulse: (&str, u32) = ("_apply_central_impulse", 3460891852u32);
    pub type Sig_apply_central_impulse = (Vector3,);
    pub const apply_impulse: (&str, u32) = ("_apply_impulse", 1714681797u32);
    pub type Sig_apply_impulse = (Vector3, Vector3,);
    pub const apply_torque_impulse: (&str, u32) = ("_apply_torque_impulse", 3460891852u32);
    pub type Sig_apply_torque_impulse = (Vector3,);
    pub const apply_central_force: (&str, u32) = ("_apply_central_force", 3460891852u32);
    pub type Sig_apply_central_force = (Vector3,);
    pub const apply_force: (&str, u32) = ("_apply_force", 1714681797u32);
    pub type Sig_apply_force = (Vector3, Vector3,);
    pub const apply_torque: (&str, u32) = ("_apply_torque", 3460891852u32);
    pub type Sig_apply_torque = (Vector3,);
    pub const add_constant_central_force: (&str, u32) = ("_add_constant_central_force", 3460891852u32);
    pub type Sig_add_constant_central_force = (Vector3,);
    pub const add_constant_force: (&str, u32) = ("_add_constant_force", 1714681797u32);
    pub type Sig_add_constant_force = (Vector3, Vector3,);
    pub const add_constant_torque: (&str, u32) = ("_add_constant_torque", 3460891852u32);
    pub type Sig_add_constant_torque = (Vector3,);
    pub const set_constant_force: (&str, u32) = ("_set_constant_force", 3460891852u32);
    pub type Sig_set_constant_force = (Vector3,);
    pub const get_constant_force: (&str, u32) = ("_get_constant_force", 3360562783u32);
    pub type Sig_get_constant_force = ();
    pub const set_constant_torque: (&str, u32) = ("_set_constant_torque", 3460891852u32);
    pub type Sig_set_constant_torque = (Vector3,);
    pub const get_constant_torque: (&str, u32) = ("_get_constant_torque", 3360562783u32);
    pub type Sig_get_constant_torque = ();
    pub const set_sleep_state: (&str, u32) = ("_set_sleep_state", 2586408642u32);
    pub type Sig_set_sleep_state = (bool,);
    pub const is_sleeping: (&str, u32) = ("_is_sleeping", 36873697u32);
    pub type Sig_is_sleeping = ();
    pub const set_collision_layer: (&str, u32) = ("_set_collision_layer", 1286410249u32);
    pub type Sig_set_collision_layer = (u32,);
    pub const get_collision_layer: (&str, u32) = ("_get_collision_layer", 3905245786u32);
    pub type Sig_get_collision_layer = ();
    pub const set_collision_mask: (&str, u32) = ("_set_collision_mask", 1286410249u32);
    pub type Sig_set_collision_mask = (u32,);
    pub const get_collision_mask: (&str, u32) = ("_get_collision_mask", 3905245786u32);
    pub type Sig_get_collision_mask = ();
    pub const get_contact_count: (&str, u32) = ("_get_contact_count", 3905245786u32);
    pub type Sig_get_contact_count = ();
    pub const get_contact_local_position: (&str, u32) = ("_get_contact_local_position", 711720468u32);
    pub type Sig_get_contact_local_position = (i32,);
    pub const get_contact_local_normal: (&str, u32) = ("_get_contact_local_normal", 711720468u32);
    pub type Sig_get_contact_local_normal = (i32,);
    pub const get_contact_impulse: (&str, u32) = ("_get_contact_impulse", 711720468u32);
    pub type Sig_get_contact_impulse = (i32,);
    pub const get_contact_local_shape: (&str, u32) = ("_get_contact_local_shape", 923996154u32);
    pub type Sig_get_contact_local_shape = (i32,);
    pub const get_contact_local_velocity_at_position: (&str, u32) = ("_get_contact_local_velocity_at_position", 711720468u32);
    pub type Sig_get_contact_local_velocity_at_position = (i32,);
    pub const get_contact_collider: (&str, u32) = ("_get_contact_collider", 495598643u32);
    pub type Sig_get_contact_collider = (i32,);
    pub const get_contact_collider_position: (&str, u32) = ("_get_contact_collider_position", 711720468u32);
    pub type Sig_get_contact_collider_position = (i32,);
    pub const get_contact_collider_id: (&str, u32) = ("_get_contact_collider_id", 923996154u32);
    pub type Sig_get_contact_collider_id = (i32,);
    pub const get_contact_collider_object: (&str, u32) = ("_get_contact_collider_object", 3332903315u32);
    pub type Sig_get_contact_collider_object = (i32,);
    pub const get_contact_collider_shape: (&str, u32) = ("_get_contact_collider_shape", 923996154u32);
    pub type Sig_get_contact_collider_shape = (i32,);
    pub const get_contact_collider_velocity_at_position: (&str, u32) = ("_get_contact_collider_velocity_at_position", 711720468u32);
    pub type Sig_get_contact_collider_velocity_at_position = (i32,);
    pub const get_step: (&str, u32) = ("_get_step", 1740695150u32);
    pub type Sig_get_step = ();
    pub const integrate_forces: (&str, u32) = ("_integrate_forces", 3218959716u32);
    pub type Sig_integrate_forces = ();
    pub const get_space_state: (&str, u32) = ("_get_space_state", 2069328350u32);
    pub type Sig_get_space_state = ();
    
}
pub mod PhysicsDirectSpaceState2D {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsDirectSpaceState2DExtension {
    pub use super::PhysicsDirectSpaceState2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const intersect_ray_rawptr: (&str, u32) = ("_intersect_ray", 2840492092u32);
    pub type Sig_intersect_ray_rawptr = (Vector2, Vector2, u32, bool, bool, bool, * mut PhysicsServer2DExtensionRayResult,);
    pub const intersect_point_rawptr: (&str, u32) = ("_intersect_point", 522407812u32);
    pub type Sig_intersect_point_rawptr = (Vector2, u64, u32, bool, bool, * mut PhysicsServer2DExtensionShapeResult, i32,);
    pub const intersect_shape_rawptr: (&str, u32) = ("_intersect_shape", 1584897015u32);
    pub type Sig_intersect_shape_rawptr = (Rid, Transform2D, Vector2, f32, u32, bool, bool, * mut PhysicsServer2DExtensionShapeResult, i32,);
    pub const cast_motion_rawptr: (&str, u32) = ("_cast_motion", 1410701151u32);
    pub type Sig_cast_motion_rawptr = (Rid, Transform2D, Vector2, f32, u32, bool, bool, * mut f64, * mut f64,);
    pub const collide_shape_rawptr: (&str, u32) = ("_collide_shape", 871510130u32);
    pub type Sig_collide_shape_rawptr = (Rid, Transform2D, Vector2, f32, u32, bool, bool, * mut c_void, i32, * mut i32,);
    pub const rest_info_rawptr: (&str, u32) = ("_rest_info", 772675997u32);
    pub type Sig_rest_info_rawptr = (Rid, Transform2D, Vector2, f32, u32, bool, bool, * mut PhysicsServer2DExtensionShapeRestInfo,);
    
}
pub mod PhysicsDirectSpaceState3D {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsDirectSpaceState3DExtension {
    pub use super::PhysicsDirectSpaceState3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const intersect_ray_rawptr: (&str, u32) = ("_intersect_ray", 2022529123u32);
    pub type Sig_intersect_ray_rawptr = (Vector3, Vector3, u32, bool, bool, bool, bool, bool, * mut PhysicsServer3DExtensionRayResult,);
    pub const intersect_point_rawptr: (&str, u32) = ("_intersect_point", 3378904092u32);
    pub type Sig_intersect_point_rawptr = (Vector3, u32, bool, bool, * mut PhysicsServer3DExtensionShapeResult, i32,);
    pub const intersect_shape_rawptr: (&str, u32) = ("_intersect_shape", 728953575u32);
    pub type Sig_intersect_shape_rawptr = (Rid, Transform3D, Vector3, f32, u32, bool, bool, * mut PhysicsServer3DExtensionShapeResult, i32,);
    pub const cast_motion_rawptr: (&str, u32) = ("_cast_motion", 2320624824u32);
    pub type Sig_cast_motion_rawptr = (Rid, Transform3D, Vector3, f32, u32, bool, bool, * mut f64, * mut f64, * mut PhysicsServer3DExtensionShapeRestInfo,);
    pub const collide_shape_rawptr: (&str, u32) = ("_collide_shape", 2320624824u32);
    pub type Sig_collide_shape_rawptr = (Rid, Transform3D, Vector3, f32, u32, bool, bool, * mut c_void, i32, * mut i32,);
    pub const rest_info_rawptr: (&str, u32) = ("_rest_info", 856242757u32);
    pub type Sig_rest_info_rawptr = (Rid, Transform3D, Vector3, f32, u32, bool, bool, * mut PhysicsServer3DExtensionShapeRestInfo,);
    pub const get_closest_point_to_object_volume: (&str, u32) = ("_get_closest_point_to_object_volume", 2056183332u32);
    pub type Sig_get_closest_point_to_object_volume = (Rid, Vector3,);
    
}
pub mod PhysicsMaterial {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsPointQueryParameters2D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsPointQueryParameters3D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsRayQueryParameters2D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsRayQueryParameters3D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsServer2D {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsServer2DExtension {
    pub use super::PhysicsServer2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const world_boundary_shape_create: (&str, u32) = ("_world_boundary_shape_create", 529393457u32);
    pub type Sig_world_boundary_shape_create = ();
    pub const separation_ray_shape_create: (&str, u32) = ("_separation_ray_shape_create", 529393457u32);
    pub type Sig_separation_ray_shape_create = ();
    pub const segment_shape_create: (&str, u32) = ("_segment_shape_create", 529393457u32);
    pub type Sig_segment_shape_create = ();
    pub const circle_shape_create: (&str, u32) = ("_circle_shape_create", 529393457u32);
    pub type Sig_circle_shape_create = ();
    pub const rectangle_shape_create: (&str, u32) = ("_rectangle_shape_create", 529393457u32);
    pub type Sig_rectangle_shape_create = ();
    pub const capsule_shape_create: (&str, u32) = ("_capsule_shape_create", 529393457u32);
    pub type Sig_capsule_shape_create = ();
    pub const convex_polygon_shape_create: (&str, u32) = ("_convex_polygon_shape_create", 529393457u32);
    pub type Sig_convex_polygon_shape_create = ();
    pub const concave_polygon_shape_create: (&str, u32) = ("_concave_polygon_shape_create", 529393457u32);
    pub type Sig_concave_polygon_shape_create = ();
    pub const shape_set_data: (&str, u32) = ("_shape_set_data", 3175752987u32);
    pub type Sig_shape_set_data = (Rid, Variant,);
    pub const shape_set_custom_solver_bias: (&str, u32) = ("_shape_set_custom_solver_bias", 1794382983u32);
    pub type Sig_shape_set_custom_solver_bias = (Rid, f32,);
    pub const shape_get_type: (&str, u32) = ("_shape_get_type", 1240598777u32);
    pub type Sig_shape_get_type = (Rid,);
    pub const shape_get_data: (&str, u32) = ("_shape_get_data", 4171304767u32);
    pub type Sig_shape_get_data = (Rid,);
    pub const shape_get_custom_solver_bias: (&str, u32) = ("_shape_get_custom_solver_bias", 866169185u32);
    pub type Sig_shape_get_custom_solver_bias = (Rid,);
    pub const shape_collide_rawptr: (&str, u32) = ("_shape_collide", 738864683u32);
    pub type Sig_shape_collide_rawptr = (Rid, Transform2D, Vector2, Rid, Transform2D, Vector2, * mut c_void, i32, * mut i32,);
    pub const space_create: (&str, u32) = ("_space_create", 529393457u32);
    pub type Sig_space_create = ();
    pub const space_set_active: (&str, u32) = ("_space_set_active", 1265174801u32);
    pub type Sig_space_set_active = (Rid, bool,);
    pub const space_is_active: (&str, u32) = ("_space_is_active", 4155700596u32);
    pub type Sig_space_is_active = (Rid,);
    pub const space_set_param: (&str, u32) = ("_space_set_param", 949194586u32);
    pub type Sig_space_set_param = (Rid, crate::classes::physics_server_2d::SpaceParameter, f32,);
    pub const space_get_param: (&str, u32) = ("_space_get_param", 874111783u32);
    pub type Sig_space_get_param = (Rid, crate::classes::physics_server_2d::SpaceParameter,);
    pub const space_get_direct_state: (&str, u32) = ("_space_get_direct_state", 3160173886u32);
    pub type Sig_space_get_direct_state = (Rid,);
    pub const space_set_debug_contacts: (&str, u32) = ("_space_set_debug_contacts", 3411492887u32);
    pub type Sig_space_set_debug_contacts = (Rid, i32,);
    pub const space_get_contacts: (&str, u32) = ("_space_get_contacts", 2222557395u32);
    pub type Sig_space_get_contacts = (Rid,);
    pub const space_get_contact_count: (&str, u32) = ("_space_get_contact_count", 2198884583u32);
    pub type Sig_space_get_contact_count = (Rid,);
    pub const area_create: (&str, u32) = ("_area_create", 529393457u32);
    pub type Sig_area_create = ();
    pub const area_set_space: (&str, u32) = ("_area_set_space", 395945892u32);
    pub type Sig_area_set_space = (Rid, Rid,);
    pub const area_get_space: (&str, u32) = ("_area_get_space", 3814569979u32);
    pub type Sig_area_get_space = (Rid,);
    pub const area_add_shape: (&str, u32) = ("_area_add_shape", 888317420u32);
    pub type Sig_area_add_shape = (Rid, Rid, Transform2D, bool,);
    pub const area_set_shape: (&str, u32) = ("_area_set_shape", 2310537182u32);
    pub type Sig_area_set_shape = (Rid, i32, Rid,);
    pub const area_set_shape_transform: (&str, u32) = ("_area_set_shape_transform", 736082694u32);
    pub type Sig_area_set_shape_transform = (Rid, i32, Transform2D,);
    pub const area_set_shape_disabled: (&str, u32) = ("_area_set_shape_disabled", 2658558584u32);
    pub type Sig_area_set_shape_disabled = (Rid, i32, bool,);
    pub const area_get_shape_count: (&str, u32) = ("_area_get_shape_count", 2198884583u32);
    pub type Sig_area_get_shape_count = (Rid,);
    pub const area_get_shape: (&str, u32) = ("_area_get_shape", 1066463050u32);
    pub type Sig_area_get_shape = (Rid, i32,);
    pub const area_get_shape_transform: (&str, u32) = ("_area_get_shape_transform", 1324854622u32);
    pub type Sig_area_get_shape_transform = (Rid, i32,);
    pub const area_remove_shape: (&str, u32) = ("_area_remove_shape", 3411492887u32);
    pub type Sig_area_remove_shape = (Rid, i32,);
    pub const area_clear_shapes: (&str, u32) = ("_area_clear_shapes", 2722037293u32);
    pub type Sig_area_clear_shapes = (Rid,);
    pub const area_attach_object_instance_id: (&str, u32) = ("_area_attach_object_instance_id", 3411492887u32);
    pub type Sig_area_attach_object_instance_id = (Rid, u64,);
    pub const area_get_object_instance_id: (&str, u32) = ("_area_get_object_instance_id", 2198884583u32);
    pub type Sig_area_get_object_instance_id = (Rid,);
    pub const area_attach_canvas_instance_id: (&str, u32) = ("_area_attach_canvas_instance_id", 3411492887u32);
    pub type Sig_area_attach_canvas_instance_id = (Rid, u64,);
    pub const area_get_canvas_instance_id: (&str, u32) = ("_area_get_canvas_instance_id", 2198884583u32);
    pub type Sig_area_get_canvas_instance_id = (Rid,);
    pub const area_set_param: (&str, u32) = ("_area_set_param", 1257146028u32);
    pub type Sig_area_set_param = (Rid, crate::classes::physics_server_2d::AreaParameter, Variant,);
    pub const area_set_transform: (&str, u32) = ("_area_set_transform", 1246044741u32);
    pub type Sig_area_set_transform = (Rid, Transform2D,);
    pub const area_get_param: (&str, u32) = ("_area_get_param", 3047435120u32);
    pub type Sig_area_get_param = (Rid, crate::classes::physics_server_2d::AreaParameter,);
    pub const area_get_transform: (&str, u32) = ("_area_get_transform", 213527486u32);
    pub type Sig_area_get_transform = (Rid,);
    pub const area_set_collision_layer: (&str, u32) = ("_area_set_collision_layer", 3411492887u32);
    pub type Sig_area_set_collision_layer = (Rid, u32,);
    pub const area_get_collision_layer: (&str, u32) = ("_area_get_collision_layer", 2198884583u32);
    pub type Sig_area_get_collision_layer = (Rid,);
    pub const area_set_collision_mask: (&str, u32) = ("_area_set_collision_mask", 3411492887u32);
    pub type Sig_area_set_collision_mask = (Rid, u32,);
    pub const area_get_collision_mask: (&str, u32) = ("_area_get_collision_mask", 2198884583u32);
    pub type Sig_area_get_collision_mask = (Rid,);
    pub const area_set_monitorable: (&str, u32) = ("_area_set_monitorable", 1265174801u32);
    pub type Sig_area_set_monitorable = (Rid, bool,);
    pub const area_set_pickable: (&str, u32) = ("_area_set_pickable", 1265174801u32);
    pub type Sig_area_set_pickable = (Rid, bool,);
    pub const area_set_monitor_callback: (&str, u32) = ("_area_set_monitor_callback", 3379118538u32);
    pub type Sig_area_set_monitor_callback = (Rid, Callable,);
    pub const area_set_area_monitor_callback: (&str, u32) = ("_area_set_area_monitor_callback", 3379118538u32);
    pub type Sig_area_set_area_monitor_callback = (Rid, Callable,);
    pub const body_create: (&str, u32) = ("_body_create", 529393457u32);
    pub type Sig_body_create = ();
    pub const body_set_space: (&str, u32) = ("_body_set_space", 395945892u32);
    pub type Sig_body_set_space = (Rid, Rid,);
    pub const body_get_space: (&str, u32) = ("_body_get_space", 3814569979u32);
    pub type Sig_body_get_space = (Rid,);
    pub const body_set_mode: (&str, u32) = ("_body_set_mode", 1658067650u32);
    pub type Sig_body_set_mode = (Rid, crate::classes::physics_server_2d::BodyMode,);
    pub const body_get_mode: (&str, u32) = ("_body_get_mode", 3261702585u32);
    pub type Sig_body_get_mode = (Rid,);
    pub const body_add_shape: (&str, u32) = ("_body_add_shape", 888317420u32);
    pub type Sig_body_add_shape = (Rid, Rid, Transform2D, bool,);
    pub const body_set_shape: (&str, u32) = ("_body_set_shape", 2310537182u32);
    pub type Sig_body_set_shape = (Rid, i32, Rid,);
    pub const body_set_shape_transform: (&str, u32) = ("_body_set_shape_transform", 736082694u32);
    pub type Sig_body_set_shape_transform = (Rid, i32, Transform2D,);
    pub const body_get_shape_count: (&str, u32) = ("_body_get_shape_count", 2198884583u32);
    pub type Sig_body_get_shape_count = (Rid,);
    pub const body_get_shape: (&str, u32) = ("_body_get_shape", 1066463050u32);
    pub type Sig_body_get_shape = (Rid, i32,);
    pub const body_get_shape_transform: (&str, u32) = ("_body_get_shape_transform", 1324854622u32);
    pub type Sig_body_get_shape_transform = (Rid, i32,);
    pub const body_set_shape_disabled: (&str, u32) = ("_body_set_shape_disabled", 2658558584u32);
    pub type Sig_body_set_shape_disabled = (Rid, i32, bool,);
    pub const body_set_shape_as_one_way_collision: (&str, u32) = ("_body_set_shape_as_one_way_collision", 2556489974u32);
    pub type Sig_body_set_shape_as_one_way_collision = (Rid, i32, bool, f32,);
    pub const body_remove_shape: (&str, u32) = ("_body_remove_shape", 3411492887u32);
    pub type Sig_body_remove_shape = (Rid, i32,);
    pub const body_clear_shapes: (&str, u32) = ("_body_clear_shapes", 2722037293u32);
    pub type Sig_body_clear_shapes = (Rid,);
    pub const body_attach_object_instance_id: (&str, u32) = ("_body_attach_object_instance_id", 3411492887u32);
    pub type Sig_body_attach_object_instance_id = (Rid, u64,);
    pub const body_get_object_instance_id: (&str, u32) = ("_body_get_object_instance_id", 2198884583u32);
    pub type Sig_body_get_object_instance_id = (Rid,);
    pub const body_attach_canvas_instance_id: (&str, u32) = ("_body_attach_canvas_instance_id", 3411492887u32);
    pub type Sig_body_attach_canvas_instance_id = (Rid, u64,);
    pub const body_get_canvas_instance_id: (&str, u32) = ("_body_get_canvas_instance_id", 2198884583u32);
    pub type Sig_body_get_canvas_instance_id = (Rid,);
    pub const body_set_continuous_collision_detection_mode: (&str, u32) = ("_body_set_continuous_collision_detection_mode", 1882257015u32);
    pub type Sig_body_set_continuous_collision_detection_mode = (Rid, crate::classes::physics_server_2d::CcdMode,);
    pub const body_get_continuous_collision_detection_mode: (&str, u32) = ("_body_get_continuous_collision_detection_mode", 2661282217u32);
    pub type Sig_body_get_continuous_collision_detection_mode = (Rid,);
    pub const body_set_collision_layer: (&str, u32) = ("_body_set_collision_layer", 3411492887u32);
    pub type Sig_body_set_collision_layer = (Rid, u32,);
    pub const body_get_collision_layer: (&str, u32) = ("_body_get_collision_layer", 2198884583u32);
    pub type Sig_body_get_collision_layer = (Rid,);
    pub const body_set_collision_mask: (&str, u32) = ("_body_set_collision_mask", 3411492887u32);
    pub type Sig_body_set_collision_mask = (Rid, u32,);
    pub const body_get_collision_mask: (&str, u32) = ("_body_get_collision_mask", 2198884583u32);
    pub type Sig_body_get_collision_mask = (Rid,);
    pub const body_set_collision_priority: (&str, u32) = ("_body_set_collision_priority", 1794382983u32);
    pub type Sig_body_set_collision_priority = (Rid, f32,);
    pub const body_get_collision_priority: (&str, u32) = ("_body_get_collision_priority", 866169185u32);
    pub type Sig_body_get_collision_priority = (Rid,);
    pub const body_set_param: (&str, u32) = ("_body_set_param", 2715630609u32);
    pub type Sig_body_set_param = (Rid, crate::classes::physics_server_2d::BodyParameter, Variant,);
    pub const body_get_param: (&str, u32) = ("_body_get_param", 3208033526u32);
    pub type Sig_body_get_param = (Rid, crate::classes::physics_server_2d::BodyParameter,);
    pub const body_reset_mass_properties: (&str, u32) = ("_body_reset_mass_properties", 2722037293u32);
    pub type Sig_body_reset_mass_properties = (Rid,);
    pub const body_set_state: (&str, u32) = ("_body_set_state", 1706355209u32);
    pub type Sig_body_set_state = (Rid, crate::classes::physics_server_2d::BodyState, Variant,);
    pub const body_get_state: (&str, u32) = ("_body_get_state", 4036367961u32);
    pub type Sig_body_get_state = (Rid, crate::classes::physics_server_2d::BodyState,);
    pub const body_apply_central_impulse: (&str, u32) = ("_body_apply_central_impulse", 3201125042u32);
    pub type Sig_body_apply_central_impulse = (Rid, Vector2,);
    pub const body_apply_torque_impulse: (&str, u32) = ("_body_apply_torque_impulse", 1794382983u32);
    pub type Sig_body_apply_torque_impulse = (Rid, f32,);
    pub const body_apply_impulse: (&str, u32) = ("_body_apply_impulse", 2762675110u32);
    pub type Sig_body_apply_impulse = (Rid, Vector2, Vector2,);
    pub const body_apply_central_force: (&str, u32) = ("_body_apply_central_force", 3201125042u32);
    pub type Sig_body_apply_central_force = (Rid, Vector2,);
    pub const body_apply_force: (&str, u32) = ("_body_apply_force", 2762675110u32);
    pub type Sig_body_apply_force = (Rid, Vector2, Vector2,);
    pub const body_apply_torque: (&str, u32) = ("_body_apply_torque", 1794382983u32);
    pub type Sig_body_apply_torque = (Rid, f32,);
    pub const body_add_constant_central_force: (&str, u32) = ("_body_add_constant_central_force", 3201125042u32);
    pub type Sig_body_add_constant_central_force = (Rid, Vector2,);
    pub const body_add_constant_force: (&str, u32) = ("_body_add_constant_force", 2762675110u32);
    pub type Sig_body_add_constant_force = (Rid, Vector2, Vector2,);
    pub const body_add_constant_torque: (&str, u32) = ("_body_add_constant_torque", 1794382983u32);
    pub type Sig_body_add_constant_torque = (Rid, f32,);
    pub const body_set_constant_force: (&str, u32) = ("_body_set_constant_force", 3201125042u32);
    pub type Sig_body_set_constant_force = (Rid, Vector2,);
    pub const body_get_constant_force: (&str, u32) = ("_body_get_constant_force", 2440833711u32);
    pub type Sig_body_get_constant_force = (Rid,);
    pub const body_set_constant_torque: (&str, u32) = ("_body_set_constant_torque", 1794382983u32);
    pub type Sig_body_set_constant_torque = (Rid, f32,);
    pub const body_get_constant_torque: (&str, u32) = ("_body_get_constant_torque", 866169185u32);
    pub type Sig_body_get_constant_torque = (Rid,);
    pub const body_set_axis_velocity: (&str, u32) = ("_body_set_axis_velocity", 3201125042u32);
    pub type Sig_body_set_axis_velocity = (Rid, Vector2,);
    pub const body_add_collision_exception: (&str, u32) = ("_body_add_collision_exception", 395945892u32);
    pub type Sig_body_add_collision_exception = (Rid, Rid,);
    pub const body_remove_collision_exception: (&str, u32) = ("_body_remove_collision_exception", 395945892u32);
    pub type Sig_body_remove_collision_exception = (Rid, Rid,);
    pub const body_get_collision_exceptions: (&str, u32) = ("_body_get_collision_exceptions", 2684255073u32);
    pub type Sig_body_get_collision_exceptions = (Rid,);
    pub const body_set_max_contacts_reported: (&str, u32) = ("_body_set_max_contacts_reported", 3411492887u32);
    pub type Sig_body_set_max_contacts_reported = (Rid, i32,);
    pub const body_get_max_contacts_reported: (&str, u32) = ("_body_get_max_contacts_reported", 2198884583u32);
    pub type Sig_body_get_max_contacts_reported = (Rid,);
    pub const body_set_contacts_reported_depth_threshold: (&str, u32) = ("_body_set_contacts_reported_depth_threshold", 1794382983u32);
    pub type Sig_body_set_contacts_reported_depth_threshold = (Rid, f32,);
    pub const body_get_contacts_reported_depth_threshold: (&str, u32) = ("_body_get_contacts_reported_depth_threshold", 866169185u32);
    pub type Sig_body_get_contacts_reported_depth_threshold = (Rid,);
    pub const body_set_omit_force_integration: (&str, u32) = ("_body_set_omit_force_integration", 1265174801u32);
    pub type Sig_body_set_omit_force_integration = (Rid, bool,);
    pub const body_is_omitting_force_integration: (&str, u32) = ("_body_is_omitting_force_integration", 4155700596u32);
    pub type Sig_body_is_omitting_force_integration = (Rid,);
    pub const body_set_state_sync_callback: (&str, u32) = ("_body_set_state_sync_callback", 3379118538u32);
    pub type Sig_body_set_state_sync_callback = (Rid, Callable,);
    pub const body_set_force_integration_callback: (&str, u32) = ("_body_set_force_integration_callback", 2828036238u32);
    pub type Sig_body_set_force_integration_callback = (Rid, Callable, Variant,);
    pub const body_collide_shape_rawptr: (&str, u32) = ("_body_collide_shape", 2131476465u32);
    pub type Sig_body_collide_shape_rawptr = (Rid, i32, Rid, Transform2D, Vector2, * mut c_void, i32, * mut i32,);
    pub const body_set_pickable: (&str, u32) = ("_body_set_pickable", 1265174801u32);
    pub type Sig_body_set_pickable = (Rid, bool,);
    pub const body_get_direct_state: (&str, u32) = ("_body_get_direct_state", 1191931871u32);
    pub type Sig_body_get_direct_state = (Rid,);
    pub const body_test_motion_rawptr: (&str, u32) = ("_body_test_motion", 104979818u32);
    pub type Sig_body_test_motion_rawptr = (Rid, Transform2D, Vector2, f32, bool, bool, * mut PhysicsServer2DExtensionMotionResult,);
    pub const joint_create: (&str, u32) = ("_joint_create", 529393457u32);
    pub type Sig_joint_create = ();
    pub const joint_clear: (&str, u32) = ("_joint_clear", 2722037293u32);
    pub type Sig_joint_clear = (Rid,);
    pub const joint_set_param: (&str, u32) = ("_joint_set_param", 3972556514u32);
    pub type Sig_joint_set_param = (Rid, crate::classes::physics_server_2d::JointParam, f32,);
    pub const joint_get_param: (&str, u32) = ("_joint_get_param", 4016448949u32);
    pub type Sig_joint_get_param = (Rid, crate::classes::physics_server_2d::JointParam,);
    pub const joint_disable_collisions_between_bodies: (&str, u32) = ("_joint_disable_collisions_between_bodies", 1265174801u32);
    pub type Sig_joint_disable_collisions_between_bodies = (Rid, bool,);
    pub const joint_is_disabled_collisions_between_bodies: (&str, u32) = ("_joint_is_disabled_collisions_between_bodies", 4155700596u32);
    pub type Sig_joint_is_disabled_collisions_between_bodies = (Rid,);
    pub const joint_make_pin: (&str, u32) = ("_joint_make_pin", 2607799521u32);
    pub type Sig_joint_make_pin = (Rid, Vector2, Rid, Rid,);
    pub const joint_make_groove: (&str, u32) = ("_joint_make_groove", 438649616u32);
    pub type Sig_joint_make_groove = (Rid, Vector2, Vector2, Vector2, Rid, Rid,);
    pub const joint_make_damped_spring: (&str, u32) = ("_joint_make_damped_spring", 1276049561u32);
    pub type Sig_joint_make_damped_spring = (Rid, Vector2, Vector2, Rid, Rid,);
    pub const pin_joint_set_flag: (&str, u32) = ("_pin_joint_set_flag", 3520002352u32);
    pub type Sig_pin_joint_set_flag = (Rid, crate::classes::physics_server_2d::PinJointFlag, bool,);
    pub const pin_joint_get_flag: (&str, u32) = ("_pin_joint_get_flag", 2647867364u32);
    pub type Sig_pin_joint_get_flag = (Rid, crate::classes::physics_server_2d::PinJointFlag,);
    pub const pin_joint_set_param: (&str, u32) = ("_pin_joint_set_param", 550574241u32);
    pub type Sig_pin_joint_set_param = (Rid, crate::classes::physics_server_2d::PinJointParam, f32,);
    pub const pin_joint_get_param: (&str, u32) = ("_pin_joint_get_param", 348281383u32);
    pub type Sig_pin_joint_get_param = (Rid, crate::classes::physics_server_2d::PinJointParam,);
    pub const damped_spring_joint_set_param: (&str, u32) = ("_damped_spring_joint_set_param", 220564071u32);
    pub type Sig_damped_spring_joint_set_param = (Rid, crate::classes::physics_server_2d::DampedSpringParam, f32,);
    pub const damped_spring_joint_get_param: (&str, u32) = ("_damped_spring_joint_get_param", 2075871277u32);
    pub type Sig_damped_spring_joint_get_param = (Rid, crate::classes::physics_server_2d::DampedSpringParam,);
    pub const joint_get_type: (&str, u32) = ("_joint_get_type", 4262502231u32);
    pub type Sig_joint_get_type = (Rid,);
    pub const free_rid: (&str, u32) = ("_free_rid", 2722037293u32);
    pub type Sig_free_rid = (Rid,);
    pub const set_active: (&str, u32) = ("_set_active", 2586408642u32);
    pub type Sig_set_active = (bool,);
    pub const init_ext: (&str, u32) = ("_init", 3218959716u32);
    pub type Sig_init_ext = ();
    pub const step: (&str, u32) = ("_step", 373806689u32);
    pub type Sig_step = (f32,);
    pub const sync: (&str, u32) = ("_sync", 3218959716u32);
    pub type Sig_sync = ();
    pub const flush_queries: (&str, u32) = ("_flush_queries", 3218959716u32);
    pub type Sig_flush_queries = ();
    pub const end_sync: (&str, u32) = ("_end_sync", 3218959716u32);
    pub type Sig_end_sync = ();
    pub const finish: (&str, u32) = ("_finish", 3218959716u32);
    pub type Sig_finish = ();
    pub const is_flushing_queries: (&str, u32) = ("_is_flushing_queries", 36873697u32);
    pub type Sig_is_flushing_queries = ();
    pub const get_process_info: (&str, u32) = ("_get_process_info", 576496006u32);
    pub type Sig_get_process_info = (crate::classes::physics_server_2d::ProcessInfo,);
    
}
pub mod PhysicsServer2DManager {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsServer3D {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsServer3DExtension {
    pub use super::PhysicsServer3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const world_boundary_shape_create: (&str, u32) = ("_world_boundary_shape_create", 529393457u32);
    pub type Sig_world_boundary_shape_create = ();
    pub const separation_ray_shape_create: (&str, u32) = ("_separation_ray_shape_create", 529393457u32);
    pub type Sig_separation_ray_shape_create = ();
    pub const sphere_shape_create: (&str, u32) = ("_sphere_shape_create", 529393457u32);
    pub type Sig_sphere_shape_create = ();
    pub const box_shape_create: (&str, u32) = ("_box_shape_create", 529393457u32);
    pub type Sig_box_shape_create = ();
    pub const capsule_shape_create: (&str, u32) = ("_capsule_shape_create", 529393457u32);
    pub type Sig_capsule_shape_create = ();
    pub const cylinder_shape_create: (&str, u32) = ("_cylinder_shape_create", 529393457u32);
    pub type Sig_cylinder_shape_create = ();
    pub const convex_polygon_shape_create: (&str, u32) = ("_convex_polygon_shape_create", 529393457u32);
    pub type Sig_convex_polygon_shape_create = ();
    pub const concave_polygon_shape_create: (&str, u32) = ("_concave_polygon_shape_create", 529393457u32);
    pub type Sig_concave_polygon_shape_create = ();
    pub const heightmap_shape_create: (&str, u32) = ("_heightmap_shape_create", 529393457u32);
    pub type Sig_heightmap_shape_create = ();
    pub const custom_shape_create: (&str, u32) = ("_custom_shape_create", 529393457u32);
    pub type Sig_custom_shape_create = ();
    pub const shape_set_data: (&str, u32) = ("_shape_set_data", 3175752987u32);
    pub type Sig_shape_set_data = (Rid, Variant,);
    pub const shape_set_custom_solver_bias: (&str, u32) = ("_shape_set_custom_solver_bias", 1794382983u32);
    pub type Sig_shape_set_custom_solver_bias = (Rid, f32,);
    pub const shape_set_margin: (&str, u32) = ("_shape_set_margin", 1794382983u32);
    pub type Sig_shape_set_margin = (Rid, f32,);
    pub const shape_get_margin: (&str, u32) = ("_shape_get_margin", 866169185u32);
    pub type Sig_shape_get_margin = (Rid,);
    pub const shape_get_type: (&str, u32) = ("_shape_get_type", 3418923367u32);
    pub type Sig_shape_get_type = (Rid,);
    pub const shape_get_data: (&str, u32) = ("_shape_get_data", 4171304767u32);
    pub type Sig_shape_get_data = (Rid,);
    pub const shape_get_custom_solver_bias: (&str, u32) = ("_shape_get_custom_solver_bias", 866169185u32);
    pub type Sig_shape_get_custom_solver_bias = (Rid,);
    pub const space_create: (&str, u32) = ("_space_create", 529393457u32);
    pub type Sig_space_create = ();
    pub const space_set_active: (&str, u32) = ("_space_set_active", 1265174801u32);
    pub type Sig_space_set_active = (Rid, bool,);
    pub const space_is_active: (&str, u32) = ("_space_is_active", 4155700596u32);
    pub type Sig_space_is_active = (Rid,);
    pub const space_set_param: (&str, u32) = ("_space_set_param", 2406017470u32);
    pub type Sig_space_set_param = (Rid, crate::classes::physics_server_3d::SpaceParameter, f32,);
    pub const space_get_param: (&str, u32) = ("_space_get_param", 1523206731u32);
    pub type Sig_space_get_param = (Rid, crate::classes::physics_server_3d::SpaceParameter,);
    pub const space_get_direct_state: (&str, u32) = ("_space_get_direct_state", 2048616813u32);
    pub type Sig_space_get_direct_state = (Rid,);
    pub const space_set_debug_contacts: (&str, u32) = ("_space_set_debug_contacts", 3411492887u32);
    pub type Sig_space_set_debug_contacts = (Rid, i32,);
    pub const space_get_contacts: (&str, u32) = ("_space_get_contacts", 808965560u32);
    pub type Sig_space_get_contacts = (Rid,);
    pub const space_get_contact_count: (&str, u32) = ("_space_get_contact_count", 2198884583u32);
    pub type Sig_space_get_contact_count = (Rid,);
    pub const area_create: (&str, u32) = ("_area_create", 529393457u32);
    pub type Sig_area_create = ();
    pub const area_set_space: (&str, u32) = ("_area_set_space", 395945892u32);
    pub type Sig_area_set_space = (Rid, Rid,);
    pub const area_get_space: (&str, u32) = ("_area_get_space", 3814569979u32);
    pub type Sig_area_get_space = (Rid,);
    pub const area_add_shape: (&str, u32) = ("_area_add_shape", 2153848567u32);
    pub type Sig_area_add_shape = (Rid, Rid, Transform3D, bool,);
    pub const area_set_shape: (&str, u32) = ("_area_set_shape", 2310537182u32);
    pub type Sig_area_set_shape = (Rid, i32, Rid,);
    pub const area_set_shape_transform: (&str, u32) = ("_area_set_shape_transform", 675327471u32);
    pub type Sig_area_set_shape_transform = (Rid, i32, Transform3D,);
    pub const area_set_shape_disabled: (&str, u32) = ("_area_set_shape_disabled", 2658558584u32);
    pub type Sig_area_set_shape_disabled = (Rid, i32, bool,);
    pub const area_get_shape_count: (&str, u32) = ("_area_get_shape_count", 2198884583u32);
    pub type Sig_area_get_shape_count = (Rid,);
    pub const area_get_shape: (&str, u32) = ("_area_get_shape", 1066463050u32);
    pub type Sig_area_get_shape = (Rid, i32,);
    pub const area_get_shape_transform: (&str, u32) = ("_area_get_shape_transform", 1050775521u32);
    pub type Sig_area_get_shape_transform = (Rid, i32,);
    pub const area_remove_shape: (&str, u32) = ("_area_remove_shape", 3411492887u32);
    pub type Sig_area_remove_shape = (Rid, i32,);
    pub const area_clear_shapes: (&str, u32) = ("_area_clear_shapes", 2722037293u32);
    pub type Sig_area_clear_shapes = (Rid,);
    pub const area_attach_object_instance_id: (&str, u32) = ("_area_attach_object_instance_id", 3411492887u32);
    pub type Sig_area_attach_object_instance_id = (Rid, u64,);
    pub const area_get_object_instance_id: (&str, u32) = ("_area_get_object_instance_id", 2198884583u32);
    pub type Sig_area_get_object_instance_id = (Rid,);
    pub const area_set_param: (&str, u32) = ("_area_set_param", 2980114638u32);
    pub type Sig_area_set_param = (Rid, crate::classes::physics_server_3d::AreaParameter, Variant,);
    pub const area_set_transform: (&str, u32) = ("_area_set_transform", 3935195649u32);
    pub type Sig_area_set_transform = (Rid, Transform3D,);
    pub const area_get_param: (&str, u32) = ("_area_get_param", 890056067u32);
    pub type Sig_area_get_param = (Rid, crate::classes::physics_server_3d::AreaParameter,);
    pub const area_get_transform: (&str, u32) = ("_area_get_transform", 1128465797u32);
    pub type Sig_area_get_transform = (Rid,);
    pub const area_set_collision_layer: (&str, u32) = ("_area_set_collision_layer", 3411492887u32);
    pub type Sig_area_set_collision_layer = (Rid, u32,);
    pub const area_get_collision_layer: (&str, u32) = ("_area_get_collision_layer", 2198884583u32);
    pub type Sig_area_get_collision_layer = (Rid,);
    pub const area_set_collision_mask: (&str, u32) = ("_area_set_collision_mask", 3411492887u32);
    pub type Sig_area_set_collision_mask = (Rid, u32,);
    pub const area_get_collision_mask: (&str, u32) = ("_area_get_collision_mask", 2198884583u32);
    pub type Sig_area_get_collision_mask = (Rid,);
    pub const area_set_monitorable: (&str, u32) = ("_area_set_monitorable", 1265174801u32);
    pub type Sig_area_set_monitorable = (Rid, bool,);
    pub const area_set_ray_pickable: (&str, u32) = ("_area_set_ray_pickable", 1265174801u32);
    pub type Sig_area_set_ray_pickable = (Rid, bool,);
    pub const area_set_monitor_callback: (&str, u32) = ("_area_set_monitor_callback", 3379118538u32);
    pub type Sig_area_set_monitor_callback = (Rid, Callable,);
    pub const area_set_area_monitor_callback: (&str, u32) = ("_area_set_area_monitor_callback", 3379118538u32);
    pub type Sig_area_set_area_monitor_callback = (Rid, Callable,);
    pub const body_create: (&str, u32) = ("_body_create", 529393457u32);
    pub type Sig_body_create = ();
    pub const body_set_space: (&str, u32) = ("_body_set_space", 395945892u32);
    pub type Sig_body_set_space = (Rid, Rid,);
    pub const body_get_space: (&str, u32) = ("_body_get_space", 3814569979u32);
    pub type Sig_body_get_space = (Rid,);
    pub const body_set_mode: (&str, u32) = ("_body_set_mode", 606803466u32);
    pub type Sig_body_set_mode = (Rid, crate::classes::physics_server_3d::BodyMode,);
    pub const body_get_mode: (&str, u32) = ("_body_get_mode", 2488819728u32);
    pub type Sig_body_get_mode = (Rid,);
    pub const body_add_shape: (&str, u32) = ("_body_add_shape", 2153848567u32);
    pub type Sig_body_add_shape = (Rid, Rid, Transform3D, bool,);
    pub const body_set_shape: (&str, u32) = ("_body_set_shape", 2310537182u32);
    pub type Sig_body_set_shape = (Rid, i32, Rid,);
    pub const body_set_shape_transform: (&str, u32) = ("_body_set_shape_transform", 675327471u32);
    pub type Sig_body_set_shape_transform = (Rid, i32, Transform3D,);
    pub const body_set_shape_disabled: (&str, u32) = ("_body_set_shape_disabled", 2658558584u32);
    pub type Sig_body_set_shape_disabled = (Rid, i32, bool,);
    pub const body_get_shape_count: (&str, u32) = ("_body_get_shape_count", 2198884583u32);
    pub type Sig_body_get_shape_count = (Rid,);
    pub const body_get_shape: (&str, u32) = ("_body_get_shape", 1066463050u32);
    pub type Sig_body_get_shape = (Rid, i32,);
    pub const body_get_shape_transform: (&str, u32) = ("_body_get_shape_transform", 1050775521u32);
    pub type Sig_body_get_shape_transform = (Rid, i32,);
    pub const body_remove_shape: (&str, u32) = ("_body_remove_shape", 3411492887u32);
    pub type Sig_body_remove_shape = (Rid, i32,);
    pub const body_clear_shapes: (&str, u32) = ("_body_clear_shapes", 2722037293u32);
    pub type Sig_body_clear_shapes = (Rid,);
    pub const body_attach_object_instance_id: (&str, u32) = ("_body_attach_object_instance_id", 3411492887u32);
    pub type Sig_body_attach_object_instance_id = (Rid, u64,);
    pub const body_get_object_instance_id: (&str, u32) = ("_body_get_object_instance_id", 2198884583u32);
    pub type Sig_body_get_object_instance_id = (Rid,);
    pub const body_set_enable_continuous_collision_detection: (&str, u32) = ("_body_set_enable_continuous_collision_detection", 1265174801u32);
    pub type Sig_body_set_enable_continuous_collision_detection = (Rid, bool,);
    pub const body_is_continuous_collision_detection_enabled: (&str, u32) = ("_body_is_continuous_collision_detection_enabled", 4155700596u32);
    pub type Sig_body_is_continuous_collision_detection_enabled = (Rid,);
    pub const body_set_collision_layer: (&str, u32) = ("_body_set_collision_layer", 3411492887u32);
    pub type Sig_body_set_collision_layer = (Rid, u32,);
    pub const body_get_collision_layer: (&str, u32) = ("_body_get_collision_layer", 2198884583u32);
    pub type Sig_body_get_collision_layer = (Rid,);
    pub const body_set_collision_mask: (&str, u32) = ("_body_set_collision_mask", 3411492887u32);
    pub type Sig_body_set_collision_mask = (Rid, u32,);
    pub const body_get_collision_mask: (&str, u32) = ("_body_get_collision_mask", 2198884583u32);
    pub type Sig_body_get_collision_mask = (Rid,);
    pub const body_set_collision_priority: (&str, u32) = ("_body_set_collision_priority", 1794382983u32);
    pub type Sig_body_set_collision_priority = (Rid, f32,);
    pub const body_get_collision_priority: (&str, u32) = ("_body_get_collision_priority", 866169185u32);
    pub type Sig_body_get_collision_priority = (Rid,);
    pub const body_set_user_flags: (&str, u32) = ("_body_set_user_flags", 3411492887u32);
    pub type Sig_body_set_user_flags = (Rid, u32,);
    pub const body_get_user_flags: (&str, u32) = ("_body_get_user_flags", 2198884583u32);
    pub type Sig_body_get_user_flags = (Rid,);
    pub const body_set_param: (&str, u32) = ("_body_set_param", 910941953u32);
    pub type Sig_body_set_param = (Rid, crate::classes::physics_server_3d::BodyParameter, Variant,);
    pub const body_get_param: (&str, u32) = ("_body_get_param", 3385027841u32);
    pub type Sig_body_get_param = (Rid, crate::classes::physics_server_3d::BodyParameter,);
    pub const body_reset_mass_properties: (&str, u32) = ("_body_reset_mass_properties", 2722037293u32);
    pub type Sig_body_reset_mass_properties = (Rid,);
    pub const body_set_state: (&str, u32) = ("_body_set_state", 599977762u32);
    pub type Sig_body_set_state = (Rid, crate::classes::physics_server_3d::BodyState, Variant,);
    pub const body_get_state: (&str, u32) = ("_body_get_state", 1850449534u32);
    pub type Sig_body_get_state = (Rid, crate::classes::physics_server_3d::BodyState,);
    pub const body_apply_central_impulse: (&str, u32) = ("_body_apply_central_impulse", 3227306858u32);
    pub type Sig_body_apply_central_impulse = (Rid, Vector3,);
    pub const body_apply_impulse: (&str, u32) = ("_body_apply_impulse", 3214966418u32);
    pub type Sig_body_apply_impulse = (Rid, Vector3, Vector3,);
    pub const body_apply_torque_impulse: (&str, u32) = ("_body_apply_torque_impulse", 3227306858u32);
    pub type Sig_body_apply_torque_impulse = (Rid, Vector3,);
    pub const body_apply_central_force: (&str, u32) = ("_body_apply_central_force", 3227306858u32);
    pub type Sig_body_apply_central_force = (Rid, Vector3,);
    pub const body_apply_force: (&str, u32) = ("_body_apply_force", 3214966418u32);
    pub type Sig_body_apply_force = (Rid, Vector3, Vector3,);
    pub const body_apply_torque: (&str, u32) = ("_body_apply_torque", 3227306858u32);
    pub type Sig_body_apply_torque = (Rid, Vector3,);
    pub const body_add_constant_central_force: (&str, u32) = ("_body_add_constant_central_force", 3227306858u32);
    pub type Sig_body_add_constant_central_force = (Rid, Vector3,);
    pub const body_add_constant_force: (&str, u32) = ("_body_add_constant_force", 3214966418u32);
    pub type Sig_body_add_constant_force = (Rid, Vector3, Vector3,);
    pub const body_add_constant_torque: (&str, u32) = ("_body_add_constant_torque", 3227306858u32);
    pub type Sig_body_add_constant_torque = (Rid, Vector3,);
    pub const body_set_constant_force: (&str, u32) = ("_body_set_constant_force", 3227306858u32);
    pub type Sig_body_set_constant_force = (Rid, Vector3,);
    pub const body_get_constant_force: (&str, u32) = ("_body_get_constant_force", 531438156u32);
    pub type Sig_body_get_constant_force = (Rid,);
    pub const body_set_constant_torque: (&str, u32) = ("_body_set_constant_torque", 3227306858u32);
    pub type Sig_body_set_constant_torque = (Rid, Vector3,);
    pub const body_get_constant_torque: (&str, u32) = ("_body_get_constant_torque", 531438156u32);
    pub type Sig_body_get_constant_torque = (Rid,);
    pub const body_set_axis_velocity: (&str, u32) = ("_body_set_axis_velocity", 3227306858u32);
    pub type Sig_body_set_axis_velocity = (Rid, Vector3,);
    pub const body_set_axis_lock: (&str, u32) = ("_body_set_axis_lock", 2020836892u32);
    pub type Sig_body_set_axis_lock = (Rid, crate::classes::physics_server_3d::BodyAxis, bool,);
    pub const body_is_axis_locked: (&str, u32) = ("_body_is_axis_locked", 587853580u32);
    pub type Sig_body_is_axis_locked = (Rid, crate::classes::physics_server_3d::BodyAxis,);
    pub const body_add_collision_exception: (&str, u32) = ("_body_add_collision_exception", 395945892u32);
    pub type Sig_body_add_collision_exception = (Rid, Rid,);
    pub const body_remove_collision_exception: (&str, u32) = ("_body_remove_collision_exception", 395945892u32);
    pub type Sig_body_remove_collision_exception = (Rid, Rid,);
    pub const body_get_collision_exceptions: (&str, u32) = ("_body_get_collision_exceptions", 2684255073u32);
    pub type Sig_body_get_collision_exceptions = (Rid,);
    pub const body_set_max_contacts_reported: (&str, u32) = ("_body_set_max_contacts_reported", 3411492887u32);
    pub type Sig_body_set_max_contacts_reported = (Rid, i32,);
    pub const body_get_max_contacts_reported: (&str, u32) = ("_body_get_max_contacts_reported", 2198884583u32);
    pub type Sig_body_get_max_contacts_reported = (Rid,);
    pub const body_set_contacts_reported_depth_threshold: (&str, u32) = ("_body_set_contacts_reported_depth_threshold", 1794382983u32);
    pub type Sig_body_set_contacts_reported_depth_threshold = (Rid, f32,);
    pub const body_get_contacts_reported_depth_threshold: (&str, u32) = ("_body_get_contacts_reported_depth_threshold", 866169185u32);
    pub type Sig_body_get_contacts_reported_depth_threshold = (Rid,);
    pub const body_set_omit_force_integration: (&str, u32) = ("_body_set_omit_force_integration", 1265174801u32);
    pub type Sig_body_set_omit_force_integration = (Rid, bool,);
    pub const body_is_omitting_force_integration: (&str, u32) = ("_body_is_omitting_force_integration", 4155700596u32);
    pub type Sig_body_is_omitting_force_integration = (Rid,);
    pub const body_set_state_sync_callback: (&str, u32) = ("_body_set_state_sync_callback", 3379118538u32);
    pub type Sig_body_set_state_sync_callback = (Rid, Callable,);
    pub const body_set_force_integration_callback: (&str, u32) = ("_body_set_force_integration_callback", 2828036238u32);
    pub type Sig_body_set_force_integration_callback = (Rid, Callable, Variant,);
    pub const body_set_ray_pickable: (&str, u32) = ("_body_set_ray_pickable", 1265174801u32);
    pub type Sig_body_set_ray_pickable = (Rid, bool,);
    pub const body_test_motion_rawptr: (&str, u32) = ("_body_test_motion", 3627463434u32);
    pub type Sig_body_test_motion_rawptr = (Rid, Transform3D, Vector3, f32, i32, bool, bool, * mut PhysicsServer3DExtensionMotionResult,);
    pub const body_get_direct_state: (&str, u32) = ("_body_get_direct_state", 3029727957u32);
    pub type Sig_body_get_direct_state = (Rid,);
    pub const soft_body_create: (&str, u32) = ("_soft_body_create", 529393457u32);
    pub type Sig_soft_body_create = ();
    pub const soft_body_update_rendering_server: (&str, u32) = ("_soft_body_update_rendering_server", 2218179753u32);
    pub type Sig_soft_body_update_rendering_server = (Rid, Option < Gd < crate::classes::PhysicsServer3DRenderingServerHandler > >,);
    pub const soft_body_set_space: (&str, u32) = ("_soft_body_set_space", 395945892u32);
    pub type Sig_soft_body_set_space = (Rid, Rid,);
    pub const soft_body_get_space: (&str, u32) = ("_soft_body_get_space", 3814569979u32);
    pub type Sig_soft_body_get_space = (Rid,);
    pub const soft_body_set_ray_pickable: (&str, u32) = ("_soft_body_set_ray_pickable", 1265174801u32);
    pub type Sig_soft_body_set_ray_pickable = (Rid, bool,);
    pub const soft_body_set_collision_layer: (&str, u32) = ("_soft_body_set_collision_layer", 3411492887u32);
    pub type Sig_soft_body_set_collision_layer = (Rid, u32,);
    pub const soft_body_get_collision_layer: (&str, u32) = ("_soft_body_get_collision_layer", 2198884583u32);
    pub type Sig_soft_body_get_collision_layer = (Rid,);
    pub const soft_body_set_collision_mask: (&str, u32) = ("_soft_body_set_collision_mask", 3411492887u32);
    pub type Sig_soft_body_set_collision_mask = (Rid, u32,);
    pub const soft_body_get_collision_mask: (&str, u32) = ("_soft_body_get_collision_mask", 2198884583u32);
    pub type Sig_soft_body_get_collision_mask = (Rid,);
    pub const soft_body_add_collision_exception: (&str, u32) = ("_soft_body_add_collision_exception", 395945892u32);
    pub type Sig_soft_body_add_collision_exception = (Rid, Rid,);
    pub const soft_body_remove_collision_exception: (&str, u32) = ("_soft_body_remove_collision_exception", 395945892u32);
    pub type Sig_soft_body_remove_collision_exception = (Rid, Rid,);
    pub const soft_body_get_collision_exceptions: (&str, u32) = ("_soft_body_get_collision_exceptions", 2684255073u32);
    pub type Sig_soft_body_get_collision_exceptions = (Rid,);
    pub const soft_body_set_state: (&str, u32) = ("_soft_body_set_state", 599977762u32);
    pub type Sig_soft_body_set_state = (Rid, crate::classes::physics_server_3d::BodyState, Variant,);
    pub const soft_body_get_state: (&str, u32) = ("_soft_body_get_state", 1850449534u32);
    pub type Sig_soft_body_get_state = (Rid, crate::classes::physics_server_3d::BodyState,);
    pub const soft_body_set_transform: (&str, u32) = ("_soft_body_set_transform", 3935195649u32);
    pub type Sig_soft_body_set_transform = (Rid, Transform3D,);
    pub const soft_body_set_simulation_precision: (&str, u32) = ("_soft_body_set_simulation_precision", 3411492887u32);
    pub type Sig_soft_body_set_simulation_precision = (Rid, i32,);
    pub const soft_body_get_simulation_precision: (&str, u32) = ("_soft_body_get_simulation_precision", 2198884583u32);
    pub type Sig_soft_body_get_simulation_precision = (Rid,);
    pub const soft_body_set_total_mass: (&str, u32) = ("_soft_body_set_total_mass", 1794382983u32);
    pub type Sig_soft_body_set_total_mass = (Rid, f32,);
    pub const soft_body_get_total_mass: (&str, u32) = ("_soft_body_get_total_mass", 866169185u32);
    pub type Sig_soft_body_get_total_mass = (Rid,);
    pub const soft_body_set_linear_stiffness: (&str, u32) = ("_soft_body_set_linear_stiffness", 1794382983u32);
    pub type Sig_soft_body_set_linear_stiffness = (Rid, f32,);
    pub const soft_body_get_linear_stiffness: (&str, u32) = ("_soft_body_get_linear_stiffness", 866169185u32);
    pub type Sig_soft_body_get_linear_stiffness = (Rid,);
    pub const soft_body_set_shrinking_factor: (&str, u32) = ("_soft_body_set_shrinking_factor", 1794382983u32);
    pub type Sig_soft_body_set_shrinking_factor = (Rid, f32,);
    pub const soft_body_get_shrinking_factor: (&str, u32) = ("_soft_body_get_shrinking_factor", 866169185u32);
    pub type Sig_soft_body_get_shrinking_factor = (Rid,);
    pub const soft_body_set_pressure_coefficient: (&str, u32) = ("_soft_body_set_pressure_coefficient", 1794382983u32);
    pub type Sig_soft_body_set_pressure_coefficient = (Rid, f32,);
    pub const soft_body_get_pressure_coefficient: (&str, u32) = ("_soft_body_get_pressure_coefficient", 866169185u32);
    pub type Sig_soft_body_get_pressure_coefficient = (Rid,);
    pub const soft_body_set_damping_coefficient: (&str, u32) = ("_soft_body_set_damping_coefficient", 1794382983u32);
    pub type Sig_soft_body_set_damping_coefficient = (Rid, f32,);
    pub const soft_body_get_damping_coefficient: (&str, u32) = ("_soft_body_get_damping_coefficient", 866169185u32);
    pub type Sig_soft_body_get_damping_coefficient = (Rid,);
    pub const soft_body_set_drag_coefficient: (&str, u32) = ("_soft_body_set_drag_coefficient", 1794382983u32);
    pub type Sig_soft_body_set_drag_coefficient = (Rid, f32,);
    pub const soft_body_get_drag_coefficient: (&str, u32) = ("_soft_body_get_drag_coefficient", 866169185u32);
    pub type Sig_soft_body_get_drag_coefficient = (Rid,);
    pub const soft_body_set_mesh: (&str, u32) = ("_soft_body_set_mesh", 395945892u32);
    pub type Sig_soft_body_set_mesh = (Rid, Rid,);
    pub const soft_body_get_bounds: (&str, u32) = ("_soft_body_get_bounds", 974181306u32);
    pub type Sig_soft_body_get_bounds = (Rid,);
    pub const soft_body_move_point: (&str, u32) = ("_soft_body_move_point", 831953689u32);
    pub type Sig_soft_body_move_point = (Rid, i32, Vector3,);
    pub const soft_body_get_point_global_position: (&str, u32) = ("_soft_body_get_point_global_position", 3440143363u32);
    pub type Sig_soft_body_get_point_global_position = (Rid, i32,);
    pub const soft_body_remove_all_pinned_points: (&str, u32) = ("_soft_body_remove_all_pinned_points", 2722037293u32);
    pub type Sig_soft_body_remove_all_pinned_points = (Rid,);
    pub const soft_body_pin_point: (&str, u32) = ("_soft_body_pin_point", 2658558584u32);
    pub type Sig_soft_body_pin_point = (Rid, i32, bool,);
    pub const soft_body_is_point_pinned: (&str, u32) = ("_soft_body_is_point_pinned", 3120086654u32);
    pub type Sig_soft_body_is_point_pinned = (Rid, i32,);
    pub const soft_body_apply_point_impulse: (&str, u32) = ("_soft_body_apply_point_impulse", 831953689u32);
    pub type Sig_soft_body_apply_point_impulse = (Rid, i32, Vector3,);
    pub const soft_body_apply_point_force: (&str, u32) = ("_soft_body_apply_point_force", 831953689u32);
    pub type Sig_soft_body_apply_point_force = (Rid, i32, Vector3,);
    pub const soft_body_apply_central_impulse: (&str, u32) = ("_soft_body_apply_central_impulse", 3227306858u32);
    pub type Sig_soft_body_apply_central_impulse = (Rid, Vector3,);
    pub const soft_body_apply_central_force: (&str, u32) = ("_soft_body_apply_central_force", 3227306858u32);
    pub type Sig_soft_body_apply_central_force = (Rid, Vector3,);
    pub const joint_create: (&str, u32) = ("_joint_create", 529393457u32);
    pub type Sig_joint_create = ();
    pub const joint_clear: (&str, u32) = ("_joint_clear", 2722037293u32);
    pub type Sig_joint_clear = (Rid,);
    pub const joint_make_pin: (&str, u32) = ("_joint_make_pin", 4280171926u32);
    pub type Sig_joint_make_pin = (Rid, Rid, Vector3, Rid, Vector3,);
    pub const pin_joint_set_param: (&str, u32) = ("_pin_joint_set_param", 810685294u32);
    pub type Sig_pin_joint_set_param = (Rid, crate::classes::physics_server_3d::PinJointParam, f32,);
    pub const pin_joint_get_param: (&str, u32) = ("_pin_joint_get_param", 2817972347u32);
    pub type Sig_pin_joint_get_param = (Rid, crate::classes::physics_server_3d::PinJointParam,);
    pub const pin_joint_set_local_a: (&str, u32) = ("_pin_joint_set_local_a", 3227306858u32);
    pub type Sig_pin_joint_set_local_a = (Rid, Vector3,);
    pub const pin_joint_get_local_a: (&str, u32) = ("_pin_joint_get_local_a", 531438156u32);
    pub type Sig_pin_joint_get_local_a = (Rid,);
    pub const pin_joint_set_local_b: (&str, u32) = ("_pin_joint_set_local_b", 3227306858u32);
    pub type Sig_pin_joint_set_local_b = (Rid, Vector3,);
    pub const pin_joint_get_local_b: (&str, u32) = ("_pin_joint_get_local_b", 531438156u32);
    pub type Sig_pin_joint_get_local_b = (Rid,);
    pub const joint_make_hinge: (&str, u32) = ("_joint_make_hinge", 1684107643u32);
    pub type Sig_joint_make_hinge = (Rid, Rid, Transform3D, Rid, Transform3D,);
    pub const joint_make_hinge_simple: (&str, u32) = ("_joint_make_hinge_simple", 4069547571u32);
    pub type Sig_joint_make_hinge_simple = (Rid, Rid, Vector3, Vector3, Rid, Vector3, Vector3,);
    pub const hinge_joint_set_param: (&str, u32) = ("_hinge_joint_set_param", 3165502333u32);
    pub type Sig_hinge_joint_set_param = (Rid, crate::classes::physics_server_3d::HingeJointParam, f32,);
    pub const hinge_joint_get_param: (&str, u32) = ("_hinge_joint_get_param", 2129207581u32);
    pub type Sig_hinge_joint_get_param = (Rid, crate::classes::physics_server_3d::HingeJointParam,);
    pub const hinge_joint_set_flag: (&str, u32) = ("_hinge_joint_set_flag", 1601626188u32);
    pub type Sig_hinge_joint_set_flag = (Rid, crate::classes::physics_server_3d::HingeJointFlag, bool,);
    pub const hinge_joint_get_flag: (&str, u32) = ("_hinge_joint_get_flag", 4165147865u32);
    pub type Sig_hinge_joint_get_flag = (Rid, crate::classes::physics_server_3d::HingeJointFlag,);
    pub const joint_make_slider: (&str, u32) = ("_joint_make_slider", 1684107643u32);
    pub type Sig_joint_make_slider = (Rid, Rid, Transform3D, Rid, Transform3D,);
    pub const slider_joint_set_param: (&str, u32) = ("_slider_joint_set_param", 2264833593u32);
    pub type Sig_slider_joint_set_param = (Rid, crate::classes::physics_server_3d::SliderJointParam, f32,);
    pub const slider_joint_get_param: (&str, u32) = ("_slider_joint_get_param", 3498644957u32);
    pub type Sig_slider_joint_get_param = (Rid, crate::classes::physics_server_3d::SliderJointParam,);
    pub const joint_make_cone_twist: (&str, u32) = ("_joint_make_cone_twist", 1684107643u32);
    pub type Sig_joint_make_cone_twist = (Rid, Rid, Transform3D, Rid, Transform3D,);
    pub const cone_twist_joint_set_param: (&str, u32) = ("_cone_twist_joint_set_param", 808587618u32);
    pub type Sig_cone_twist_joint_set_param = (Rid, crate::classes::physics_server_3d::ConeTwistJointParam, f32,);
    pub const cone_twist_joint_get_param: (&str, u32) = ("_cone_twist_joint_get_param", 1134789658u32);
    pub type Sig_cone_twist_joint_get_param = (Rid, crate::classes::physics_server_3d::ConeTwistJointParam,);
    pub const joint_make_generic_6dof: (&str, u32) = ("_joint_make_generic_6dof", 1684107643u32);
    pub type Sig_joint_make_generic_6dof = (Rid, Rid, Transform3D, Rid, Transform3D,);
    pub const generic_6dof_joint_set_param: (&str, u32) = ("_generic_6dof_joint_set_param", 2600081391u32);
    pub type Sig_generic_6dof_joint_set_param = (Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisParam, f32,);
    pub const generic_6dof_joint_get_param: (&str, u32) = ("_generic_6dof_joint_get_param", 467122058u32);
    pub type Sig_generic_6dof_joint_get_param = (Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisParam,);
    pub const generic_6dof_joint_set_flag: (&str, u32) = ("_generic_6dof_joint_set_flag", 3570926903u32);
    pub type Sig_generic_6dof_joint_set_flag = (Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisFlag, bool,);
    pub const generic_6dof_joint_get_flag: (&str, u32) = ("_generic_6dof_joint_get_flag", 4158090196u32);
    pub type Sig_generic_6dof_joint_get_flag = (Rid, Vector3Axis, crate::classes::physics_server_3d::G6dofJointAxisFlag,);
    pub const joint_get_type: (&str, u32) = ("_joint_get_type", 4290791900u32);
    pub type Sig_joint_get_type = (Rid,);
    pub const joint_set_solver_priority: (&str, u32) = ("_joint_set_solver_priority", 3411492887u32);
    pub type Sig_joint_set_solver_priority = (Rid, i32,);
    pub const joint_get_solver_priority: (&str, u32) = ("_joint_get_solver_priority", 2198884583u32);
    pub type Sig_joint_get_solver_priority = (Rid,);
    pub const joint_disable_collisions_between_bodies: (&str, u32) = ("_joint_disable_collisions_between_bodies", 1265174801u32);
    pub type Sig_joint_disable_collisions_between_bodies = (Rid, bool,);
    pub const joint_is_disabled_collisions_between_bodies: (&str, u32) = ("_joint_is_disabled_collisions_between_bodies", 4155700596u32);
    pub type Sig_joint_is_disabled_collisions_between_bodies = (Rid,);
    pub const free_rid: (&str, u32) = ("_free_rid", 2722037293u32);
    pub type Sig_free_rid = (Rid,);
    pub const set_active: (&str, u32) = ("_set_active", 2586408642u32);
    pub type Sig_set_active = (bool,);
    pub const init_ext: (&str, u32) = ("_init", 3218959716u32);
    pub type Sig_init_ext = ();
    pub const step: (&str, u32) = ("_step", 373806689u32);
    pub type Sig_step = (f32,);
    pub const sync: (&str, u32) = ("_sync", 3218959716u32);
    pub type Sig_sync = ();
    pub const flush_queries: (&str, u32) = ("_flush_queries", 3218959716u32);
    pub type Sig_flush_queries = ();
    pub const end_sync: (&str, u32) = ("_end_sync", 3218959716u32);
    pub type Sig_end_sync = ();
    pub const finish: (&str, u32) = ("_finish", 3218959716u32);
    pub type Sig_finish = ();
    pub const is_flushing_queries: (&str, u32) = ("_is_flushing_queries", 36873697u32);
    pub type Sig_is_flushing_queries = ();
    pub const get_process_info: (&str, u32) = ("_get_process_info", 1332958745u32);
    pub type Sig_get_process_info = (crate::classes::physics_server_3d::ProcessInfo,);
    
}
pub mod PhysicsServer3DManager {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsServer3DRenderingServerHandler {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const set_vertex: (&str, u32) = ("_set_vertex", 1530502735u32);
    pub type Sig_set_vertex = (i32, Vector3,);
    pub const set_normal: (&str, u32) = ("_set_normal", 1530502735u32);
    pub type Sig_set_normal = (i32, Vector3,);
    pub const set_aabb: (&str, u32) = ("_set_aabb", 259215842u32);
    pub type Sig_set_aabb = (Aabb,);
    
}
pub mod PhysicsShapeQueryParameters2D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsShapeQueryParameters3D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsTestMotionParameters2D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsTestMotionParameters3D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsTestMotionResult2D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PhysicsTestMotionResult3D {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PinJoint2D {
    pub use super::Joint2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PinJoint3D {
    pub use super::Joint3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaceholderCubemap {
    pub use super::PlaceholderTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaceholderCubemapArray {
    pub use super::PlaceholderTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaceholderMaterial {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaceholderMesh {
    pub use super::Mesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaceholderTexture2D {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaceholderTexture2DArray {
    pub use super::PlaceholderTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaceholderTexture3D {
    pub use super::Texture3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaceholderTextureLayered {
    pub use super::TextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PlaneMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PointLight2D {
    pub use super::Light2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PointMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Polygon2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PolygonOccluder3D {
    pub use super::Occluder3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PolygonPathFinder {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Popup {
    pub use super::Window::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PopupMenu {
    pub use super::Popup::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PopupPanel {
    pub use super::Popup::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PortableCompressedTexture2D {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PrimitiveMesh {
    pub use super::Mesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const create_mesh_array: (&str, u32) = ("_create_mesh_array", 3995934104u32);
    pub type Sig_create_mesh_array = ();
    
}
pub mod PrismMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ProceduralSkyMaterial {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ProgressBar {
    pub use super::Range::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ProjectSettings {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod PropertyTweener {
    pub use super::Tweener::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod QuadMesh {
    pub use super::PlaneMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod QuadOccluder3D {
    pub use super::Occluder3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdAttachmentFormat {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdFramebufferPass {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdPipelineColorBlendState {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdPipelineColorBlendStateAttachment {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdPipelineDepthStencilState {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdPipelineMultisampleState {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdPipelineRasterizationState {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdPipelineSpecializationConstant {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdSamplerState {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdShaderFile {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdShaderSpirv {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdShaderSource {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdTextureFormat {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdTextureView {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdUniform {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RdVertexAttribute {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RandomNumberGenerator {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Range {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const value_changed: (&str, u32) = ("_value_changed", 373806689u32);
    pub type Sig_value_changed = (f64,);
    
}
pub mod RayCast2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RayCast3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RectangleShape2D {
    pub use super::Shape2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RefCounted {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ReferenceRect {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ReflectionProbe {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RegEx {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RegExMatch {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RemoteTransform2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RemoteTransform3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderData {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderDataExtension {
    pub use super::RenderData::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_render_scene_buffers: (&str, u32) = ("_get_render_scene_buffers", 2793216201u32);
    pub type Sig_get_render_scene_buffers = ();
    pub const get_render_scene_data: (&str, u32) = ("_get_render_scene_data", 1288715698u32);
    pub type Sig_get_render_scene_data = ();
    pub const get_environment: (&str, u32) = ("_get_environment", 2944877500u32);
    pub type Sig_get_environment = ();
    pub const get_camera_attributes: (&str, u32) = ("_get_camera_attributes", 2944877500u32);
    pub type Sig_get_camera_attributes = ();
    
}
pub mod RenderDataRd {
    pub use super::RenderData::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderSceneBuffers {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderSceneBuffersConfiguration {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderSceneBuffersExtension {
    pub use super::RenderSceneBuffers::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const configure: (&str, u32) = ("_configure", 3072623270u32);
    pub type Sig_configure = (Option < Gd < crate::classes::RenderSceneBuffersConfiguration > >,);
    pub const set_fsr_sharpness: (&str, u32) = ("_set_fsr_sharpness", 373806689u32);
    pub type Sig_set_fsr_sharpness = (f32,);
    pub const set_texture_mipmap_bias: (&str, u32) = ("_set_texture_mipmap_bias", 373806689u32);
    pub type Sig_set_texture_mipmap_bias = (f32,);
    pub const set_anisotropic_filtering_level: (&str, u32) = ("_set_anisotropic_filtering_level", 1286410249u32);
    pub type Sig_set_anisotropic_filtering_level = (i32,);
    pub const set_use_debanding: (&str, u32) = ("_set_use_debanding", 2586408642u32);
    pub type Sig_set_use_debanding = (bool,);
    
}
pub mod RenderSceneBuffersRd {
    pub use super::RenderSceneBuffers::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderSceneData {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderSceneDataExtension {
    pub use super::RenderSceneData::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_cam_transform: (&str, u32) = ("_get_cam_transform", 3229777777u32);
    pub type Sig_get_cam_transform = ();
    pub const get_cam_projection: (&str, u32) = ("_get_cam_projection", 2910717950u32);
    pub type Sig_get_cam_projection = ();
    pub const get_view_count: (&str, u32) = ("_get_view_count", 3905245786u32);
    pub type Sig_get_view_count = ();
    pub const get_view_eye_offset: (&str, u32) = ("_get_view_eye_offset", 711720468u32);
    pub type Sig_get_view_eye_offset = (u32,);
    pub const get_view_projection: (&str, u32) = ("_get_view_projection", 3179846605u32);
    pub type Sig_get_view_projection = (u32,);
    pub const get_uniform_buffer: (&str, u32) = ("_get_uniform_buffer", 2944877500u32);
    pub type Sig_get_uniform_buffer = ();
    
}
pub mod RenderSceneDataRd {
    pub use super::RenderSceneData::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderingDevice {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RenderingServer {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Resource {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const setup_local_to_scene: (&str, u32) = ("_setup_local_to_scene", 3218959716u32);
    pub type Sig_setup_local_to_scene = ();
    pub const get_rid: (&str, u32) = ("_get_rid", 2944877500u32);
    pub type Sig_get_rid = ();
    pub const reset_state: (&str, u32) = ("_reset_state", 3218959716u32);
    pub type Sig_reset_state = ();
    pub const set_path_cache: (&str, u32) = ("_set_path_cache", 3089850668u32);
    pub type Sig_set_path_cache = (GString,);
    
}
pub mod ResourceFormatLoader {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_recognized_extensions: (&str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub type Sig_get_recognized_extensions = ();
    pub const recognize_path: (&str, u32) = ("_recognize_path", 2594487047u32);
    pub type Sig_recognize_path = (GString, StringName,);
    pub const handles_type: (&str, u32) = ("_handles_type", 2619796661u32);
    pub type Sig_handles_type = (StringName,);
    pub const get_resource_type: (&str, u32) = ("_get_resource_type", 3135753539u32);
    pub type Sig_get_resource_type = (GString,);
    pub const get_resource_script_class: (&str, u32) = ("_get_resource_script_class", 3135753539u32);
    pub type Sig_get_resource_script_class = (GString,);
    pub const get_resource_uid: (&str, u32) = ("_get_resource_uid", 1321353865u32);
    pub type Sig_get_resource_uid = (GString,);
    pub const get_dependencies: (&str, u32) = ("_get_dependencies", 6257701u32);
    pub type Sig_get_dependencies = (GString, bool,);
    pub const rename_dependencies: (&str, u32) = ("_rename_dependencies", 223715120u32);
    pub type Sig_rename_dependencies = (GString, VarDictionary,);
    pub const exists: (&str, u32) = ("_exists", 3927539163u32);
    pub type Sig_exists = (GString,);
    pub const get_classes_used: (&str, u32) = ("_get_classes_used", 4291131558u32);
    pub type Sig_get_classes_used = (GString,);
    pub const load: (&str, u32) = ("_load", 2885906527u32);
    pub type Sig_load = (GString, GString, bool, i32,);
    
}
pub mod ResourceFormatSaver {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const save: (&str, u32) = ("_save", 2794699034u32);
    pub type Sig_save = (Option < Gd < crate::classes::Resource > >, GString, u32,);
    pub const set_uid: (&str, u32) = ("_set_uid", 993915709u32);
    pub type Sig_set_uid = (GString, i64,);
    pub const recognize: (&str, u32) = ("_recognize", 3190994482u32);
    pub type Sig_recognize = (Option < Gd < crate::classes::Resource > >,);
    pub const get_recognized_extensions: (&str, u32) = ("_get_recognized_extensions", 1567505034u32);
    pub type Sig_get_recognized_extensions = (Option < Gd < crate::classes::Resource > >,);
    pub const recognize_path: (&str, u32) = ("_recognize_path", 710996192u32);
    pub type Sig_recognize_path = (Option < Gd < crate::classes::Resource > >, GString,);
    
}
pub mod ResourceImporter {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_build_dependencies: (&str, u32) = ("_get_build_dependencies", 4291131558u32);
    pub type Sig_get_build_dependencies = (GString,);
    
}
pub mod ResourceImporterBmFont {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterBitMap {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterCsvTranslation {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterDynamicFont {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterImage {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterImageFont {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterLayeredTexture {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterMp3 {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterObj {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterOggVorbis {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterSvg {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterScene {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterShaderFile {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterTexture {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterTextureAtlas {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceImporterWav {
    pub use super::ResourceImporter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceLoader {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourcePreloader {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceSaver {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ResourceUid {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RetargetModifier3D {
    pub use super::SkeletonModifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RibbonTrailMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RichTextEffect {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const process_custom_fx: (&str, u32) = ("_process_custom_fx", 31984339u32);
    pub type Sig_process_custom_fx = (Option < Gd < crate::classes::CharFxTransform > >,);
    
}
pub mod RichTextLabel {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod RigidBody2D {
    pub use super::PhysicsBody2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const integrate_forces: (&str, u32) = ("_integrate_forces", 370287496u32);
    pub type Sig_integrate_forces = (Option < Gd < crate::classes::PhysicsDirectBodyState2D > >,);
    
}
pub mod RigidBody3D {
    pub use super::PhysicsBody3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const integrate_forces: (&str, u32) = ("_integrate_forces", 420958145u32);
    pub type Sig_integrate_forces = (Option < Gd < crate::classes::PhysicsDirectBodyState3D > >,);
    
}
pub mod RootMotionView {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SceneMultiplayer {
    pub use super::MultiplayerApi::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SceneReplicationConfig {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SceneState {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SceneTree {
    pub use super::MainLoop::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SceneTreeTimer {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Script {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ScriptBacktrace {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ScriptCreateDialog {
    pub use super::ConfirmationDialog::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ScriptEditor {
    pub use super::PanelContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ScriptEditorBase {
    pub use super::VBoxContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ScriptExtension {
    pub use super::Script::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const editor_can_reload_from_file: (&str, u32) = ("_editor_can_reload_from_file", 2240911060u32);
    pub type Sig_editor_can_reload_from_file = ();
    pub const placeholder_erased_rawptr: (&str, u32) = ("_placeholder_erased", 1286410249u32);
    pub type Sig_placeholder_erased_rawptr = (* mut c_void,);
    pub const can_instantiate: (&str, u32) = ("_can_instantiate", 36873697u32);
    pub type Sig_can_instantiate = ();
    pub const get_base_script: (&str, u32) = ("_get_base_script", 278624046u32);
    pub type Sig_get_base_script = ();
    pub const get_global_name: (&str, u32) = ("_get_global_name", 2002593661u32);
    pub type Sig_get_global_name = ();
    pub const inherits_script: (&str, u32) = ("_inherits_script", 3669307804u32);
    pub type Sig_inherits_script = (Gd < crate::classes::Script >,);
    pub const get_instance_base_type: (&str, u32) = ("_get_instance_base_type", 2002593661u32);
    pub type Sig_get_instance_base_type = ();
    pub const instance_create_rawptr: (&str, u32) = ("_instance_create", 1107568780u32);
    pub type Sig_instance_create_rawptr = (Gd < crate::classes::Object >,);
    pub const placeholder_instance_create_rawptr: (&str, u32) = ("_placeholder_instance_create", 1107568780u32);
    pub type Sig_placeholder_instance_create_rawptr = (Gd < crate::classes::Object >,);
    pub const instance_has: (&str, u32) = ("_instance_has", 397768994u32);
    pub type Sig_instance_has = (Gd < crate::classes::Object >,);
    pub const has_source_code: (&str, u32) = ("_has_source_code", 36873697u32);
    pub type Sig_has_source_code = ();
    pub const get_source_code: (&str, u32) = ("_get_source_code", 201670096u32);
    pub type Sig_get_source_code = ();
    pub const set_source_code: (&str, u32) = ("_set_source_code", 83702148u32);
    pub type Sig_set_source_code = (GString,);
    pub const reload: (&str, u32) = ("_reload", 1413768114u32);
    pub type Sig_reload = (bool,);
    pub const get_doc_class_name: (&str, u32) = ("_get_doc_class_name", 2002593661u32);
    pub type Sig_get_doc_class_name = ();
    pub const get_documentation: (&str, u32) = ("_get_documentation", 3995934104u32);
    pub type Sig_get_documentation = ();
    pub const get_class_icon_path: (&str, u32) = ("_get_class_icon_path", 201670096u32);
    pub type Sig_get_class_icon_path = ();
    pub const has_method: (&str, u32) = ("_has_method", 2619796661u32);
    pub type Sig_has_method = (StringName,);
    pub const has_static_method: (&str, u32) = ("_has_static_method", 2619796661u32);
    pub type Sig_has_static_method = (StringName,);
    pub const get_script_method_argument_count: (&str, u32) = ("_get_script_method_argument_count", 2760726917u32);
    pub type Sig_get_script_method_argument_count = (StringName,);
    pub const get_method_info: (&str, u32) = ("_get_method_info", 4028089122u32);
    pub type Sig_get_method_info = (StringName,);
    pub const is_tool: (&str, u32) = ("_is_tool", 36873697u32);
    pub type Sig_is_tool = ();
    pub const is_valid: (&str, u32) = ("_is_valid", 36873697u32);
    pub type Sig_is_valid = ();
    pub const is_abstract: (&str, u32) = ("_is_abstract", 36873697u32);
    pub type Sig_is_abstract = ();
    pub const get_language: (&str, u32) = ("_get_language", 3096237657u32);
    pub type Sig_get_language = ();
    pub const has_script_signal: (&str, u32) = ("_has_script_signal", 2619796661u32);
    pub type Sig_has_script_signal = (StringName,);
    pub const get_script_signal_list: (&str, u32) = ("_get_script_signal_list", 3995934104u32);
    pub type Sig_get_script_signal_list = ();
    pub const has_property_default_value: (&str, u32) = ("_has_property_default_value", 2619796661u32);
    pub type Sig_has_property_default_value = (StringName,);
    pub const get_property_default_value: (&str, u32) = ("_get_property_default_value", 2760726917u32);
    pub type Sig_get_property_default_value = (StringName,);
    pub const update_exports: (&str, u32) = ("_update_exports", 3218959716u32);
    pub type Sig_update_exports = ();
    pub const get_script_method_list: (&str, u32) = ("_get_script_method_list", 3995934104u32);
    pub type Sig_get_script_method_list = ();
    pub const get_script_property_list: (&str, u32) = ("_get_script_property_list", 3995934104u32);
    pub type Sig_get_script_property_list = ();
    pub const get_member_line: (&str, u32) = ("_get_member_line", 2458036349u32);
    pub type Sig_get_member_line = (StringName,);
    pub const get_constants: (&str, u32) = ("_get_constants", 3102165223u32);
    pub type Sig_get_constants = ();
    pub const get_members: (&str, u32) = ("_get_members", 3995934104u32);
    pub type Sig_get_members = ();
    pub const is_placeholder_fallback_enabled: (&str, u32) = ("_is_placeholder_fallback_enabled", 36873697u32);
    pub type Sig_is_placeholder_fallback_enabled = ();
    pub const get_rpc_config: (&str, u32) = ("_get_rpc_config", 1214101251u32);
    pub type Sig_get_rpc_config = ();
    
}
pub mod ScriptLanguage {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ScriptLanguageExtension {
    pub use super::ScriptLanguage::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_name: (&str, u32) = ("_get_name", 201670096u32);
    pub type Sig_get_name = ();
    pub const init_ext: (&str, u32) = ("_init", 3218959716u32);
    pub type Sig_init_ext = ();
    pub const get_type: (&str, u32) = ("_get_type", 201670096u32);
    pub type Sig_get_type = ();
    pub const get_extension: (&str, u32) = ("_get_extension", 201670096u32);
    pub type Sig_get_extension = ();
    pub const finish: (&str, u32) = ("_finish", 3218959716u32);
    pub type Sig_finish = ();
    pub const get_reserved_words: (&str, u32) = ("_get_reserved_words", 1139954409u32);
    pub type Sig_get_reserved_words = ();
    pub const is_control_flow_keyword: (&str, u32) = ("_is_control_flow_keyword", 3927539163u32);
    pub type Sig_is_control_flow_keyword = (GString,);
    pub const get_comment_delimiters: (&str, u32) = ("_get_comment_delimiters", 1139954409u32);
    pub type Sig_get_comment_delimiters = ();
    pub const get_doc_comment_delimiters: (&str, u32) = ("_get_doc_comment_delimiters", 1139954409u32);
    pub type Sig_get_doc_comment_delimiters = ();
    pub const get_string_delimiters: (&str, u32) = ("_get_string_delimiters", 1139954409u32);
    pub type Sig_get_string_delimiters = ();
    pub const make_template: (&str, u32) = ("_make_template", 3583744548u32);
    pub type Sig_make_template = (GString, GString, GString,);
    pub const get_built_in_templates: (&str, u32) = ("_get_built_in_templates", 3147814860u32);
    pub type Sig_get_built_in_templates = (StringName,);
    pub const is_using_templates: (&str, u32) = ("_is_using_templates", 2240911060u32);
    pub type Sig_is_using_templates = ();
    pub const validate: (&str, u32) = ("_validate", 1697887509u32);
    pub type Sig_validate = (GString, GString, bool, bool, bool, bool,);
    pub const validate_path: (&str, u32) = ("_validate_path", 3135753539u32);
    pub type Sig_validate_path = (GString,);
    pub const create_script: (&str, u32) = ("_create_script", 1981248198u32);
    pub type Sig_create_script = ();
    pub const has_named_classes: (&str, u32) = ("_has_named_classes", 36873697u32);
    pub type Sig_has_named_classes = ();
    pub const supports_builtin_mode: (&str, u32) = ("_supports_builtin_mode", 36873697u32);
    pub type Sig_supports_builtin_mode = ();
    pub const supports_documentation: (&str, u32) = ("_supports_documentation", 36873697u32);
    pub type Sig_supports_documentation = ();
    pub const can_inherit_from_file: (&str, u32) = ("_can_inherit_from_file", 36873697u32);
    pub type Sig_can_inherit_from_file = ();
    pub const find_function: (&str, u32) = ("_find_function", 2878152881u32);
    pub type Sig_find_function = (GString, GString,);
    pub const make_function: (&str, u32) = ("_make_function", 1243061914u32);
    pub type Sig_make_function = (GString, GString, PackedStringArray,);
    pub const can_make_function: (&str, u32) = ("_can_make_function", 36873697u32);
    pub type Sig_can_make_function = ();
    pub const open_in_external_editor: (&str, u32) = ("_open_in_external_editor", 552845695u32);
    pub type Sig_open_in_external_editor = (Option < Gd < crate::classes::Script > >, i32, i32,);
    pub const overrides_external_editor: (&str, u32) = ("_overrides_external_editor", 2240911060u32);
    pub type Sig_overrides_external_editor = ();
    pub const preferred_file_name_casing: (&str, u32) = ("_preferred_file_name_casing", 2969522789u32);
    pub type Sig_preferred_file_name_casing = ();
    pub const complete_code: (&str, u32) = ("_complete_code", 950756616u32);
    pub type Sig_complete_code = (GString, GString, Option < Gd < crate::classes::Object > >,);
    pub const lookup_code: (&str, u32) = ("_lookup_code", 3143837309u32);
    pub type Sig_lookup_code = (GString, GString, GString, Option < Gd < crate::classes::Object > >,);
    pub const auto_indent_code: (&str, u32) = ("_auto_indent_code", 2531480354u32);
    pub type Sig_auto_indent_code = (GString, i32, i32,);
    pub const add_global_constant: (&str, u32) = ("_add_global_constant", 3776071444u32);
    pub type Sig_add_global_constant = (StringName, Variant,);
    pub const add_named_global_constant: (&str, u32) = ("_add_named_global_constant", 3776071444u32);
    pub type Sig_add_named_global_constant = (StringName, Variant,);
    pub const remove_named_global_constant: (&str, u32) = ("_remove_named_global_constant", 3304788590u32);
    pub type Sig_remove_named_global_constant = (StringName,);
    pub const thread_enter: (&str, u32) = ("_thread_enter", 3218959716u32);
    pub type Sig_thread_enter = ();
    pub const thread_exit: (&str, u32) = ("_thread_exit", 3218959716u32);
    pub type Sig_thread_exit = ();
    pub const debug_get_error: (&str, u32) = ("_debug_get_error", 201670096u32);
    pub type Sig_debug_get_error = ();
    pub const debug_get_stack_level_count: (&str, u32) = ("_debug_get_stack_level_count", 3905245786u32);
    pub type Sig_debug_get_stack_level_count = ();
    pub const debug_get_stack_level_line: (&str, u32) = ("_debug_get_stack_level_line", 923996154u32);
    pub type Sig_debug_get_stack_level_line = (i32,);
    pub const debug_get_stack_level_function: (&str, u32) = ("_debug_get_stack_level_function", 844755477u32);
    pub type Sig_debug_get_stack_level_function = (i32,);
    pub const debug_get_stack_level_source: (&str, u32) = ("_debug_get_stack_level_source", 844755477u32);
    pub type Sig_debug_get_stack_level_source = (i32,);
    pub const debug_get_stack_level_locals: (&str, u32) = ("_debug_get_stack_level_locals", 335235777u32);
    pub type Sig_debug_get_stack_level_locals = (i32, i32, i32,);
    pub const debug_get_stack_level_members: (&str, u32) = ("_debug_get_stack_level_members", 335235777u32);
    pub type Sig_debug_get_stack_level_members = (i32, i32, i32,);
    pub const debug_get_stack_level_instance_rawptr: (&str, u32) = ("_debug_get_stack_level_instance", 3744713108u32);
    pub type Sig_debug_get_stack_level_instance_rawptr = (i32,);
    pub const debug_get_globals: (&str, u32) = ("_debug_get_globals", 4123630098u32);
    pub type Sig_debug_get_globals = (i32, i32,);
    pub const debug_parse_stack_level_expression: (&str, u32) = ("_debug_parse_stack_level_expression", 1135811067u32);
    pub type Sig_debug_parse_stack_level_expression = (i32, GString, i32, i32,);
    pub const debug_get_current_stack_info: (&str, u32) = ("_debug_get_current_stack_info", 2915620761u32);
    pub type Sig_debug_get_current_stack_info = ();
    pub const reload_all_scripts: (&str, u32) = ("_reload_all_scripts", 3218959716u32);
    pub type Sig_reload_all_scripts = ();
    pub const reload_scripts: (&str, u32) = ("_reload_scripts", 3156113851u32);
    pub type Sig_reload_scripts = (VarArray, bool,);
    pub const reload_tool_script: (&str, u32) = ("_reload_tool_script", 1957307671u32);
    pub type Sig_reload_tool_script = (Option < Gd < crate::classes::Script > >, bool,);
    pub const get_recognized_extensions: (&str, u32) = ("_get_recognized_extensions", 1139954409u32);
    pub type Sig_get_recognized_extensions = ();
    pub const get_public_functions: (&str, u32) = ("_get_public_functions", 3995934104u32);
    pub type Sig_get_public_functions = ();
    pub const get_public_constants: (&str, u32) = ("_get_public_constants", 3102165223u32);
    pub type Sig_get_public_constants = ();
    pub const get_public_annotations: (&str, u32) = ("_get_public_annotations", 3995934104u32);
    pub type Sig_get_public_annotations = ();
    pub const profiling_start: (&str, u32) = ("_profiling_start", 3218959716u32);
    pub type Sig_profiling_start = ();
    pub const profiling_stop: (&str, u32) = ("_profiling_stop", 3218959716u32);
    pub type Sig_profiling_stop = ();
    pub const profiling_set_save_native_calls: (&str, u32) = ("_profiling_set_save_native_calls", 2586408642u32);
    pub type Sig_profiling_set_save_native_calls = (bool,);
    pub const profiling_get_accumulated_data_rawptr: (&str, u32) = ("_profiling_get_accumulated_data", 50157827u32);
    pub type Sig_profiling_get_accumulated_data_rawptr = (* mut ScriptLanguageExtensionProfilingInfo, i32,);
    pub const profiling_get_frame_data_rawptr: (&str, u32) = ("_profiling_get_frame_data", 50157827u32);
    pub type Sig_profiling_get_frame_data_rawptr = (* mut ScriptLanguageExtensionProfilingInfo, i32,);
    pub const frame: (&str, u32) = ("_frame", 3218959716u32);
    pub type Sig_frame = ();
    pub const handles_global_class_type: (&str, u32) = ("_handles_global_class_type", 3927539163u32);
    pub type Sig_handles_global_class_type = (GString,);
    pub const get_global_class_name: (&str, u32) = ("_get_global_class_name", 2248993622u32);
    pub type Sig_get_global_class_name = (GString,);
    
}
pub mod ScrollBar {
    pub use super::Range::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ScrollContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SegmentShape2D {
    pub use super::Shape2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SeparationRayShape2D {
    pub use super::Shape2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SeparationRayShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Separator {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Shader {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ShaderGlobalsOverride {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ShaderInclude {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ShaderIncludeDb {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ShaderMaterial {
    pub use super::Material::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Shape2D {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Shape3D {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ShapeCast2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ShapeCast3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Shortcut {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Skeleton2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Skeleton3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SkeletonIk3d {
    pub use super::SkeletonModifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SkeletonModifier3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const process_modification_with_delta: (&str, u32) = ("_process_modification_with_delta", 373806689u32);
    pub type Sig_process_modification_with_delta = (f64,);
    pub const process_modification: (&str, u32) = ("_process_modification", 3218959716u32);
    pub type Sig_process_modification = ();
    pub const skeleton_changed: (&str, u32) = ("_skeleton_changed", 2926744397u32);
    pub type Sig_skeleton_changed = (Option < Gd < crate::classes::Skeleton3D > >, Option < Gd < crate::classes::Skeleton3D > >,);
    pub const validate_bone_names: (&str, u32) = ("_validate_bone_names", 3218959716u32);
    pub type Sig_validate_bone_names = ();
    
}
pub mod SkeletonProfile {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SkeletonProfileHumanoid {
    pub use super::SkeletonProfile::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Skin {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SkinReference {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Sky {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Slider {
    pub use super::Range::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SliderJoint3D {
    pub use super::Joint3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SoftBody3D {
    pub use super::MeshInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SphereMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SphereOccluder3D {
    pub use super::Occluder3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SphereShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpinBox {
    pub use super::Range::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SplitContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpotLight3D {
    pub use super::Light3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpringArm3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpringBoneCollision3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpringBoneCollisionCapsule3D {
    pub use super::SpringBoneCollision3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpringBoneCollisionPlane3D {
    pub use super::SpringBoneCollision3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpringBoneCollisionSphere3D {
    pub use super::SpringBoneCollision3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpringBoneSimulator3D {
    pub use super::SkeletonModifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Sprite2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Sprite3D {
    pub use super::SpriteBase3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpriteBase3D {
    pub use super::GeometryInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SpriteFrames {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StandardMaterial3D {
    pub use super::BaseMaterial3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StaticBody2D {
    pub use super::PhysicsBody2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StaticBody3D {
    pub use super::PhysicsBody3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StatusIndicator {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StreamPeer {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StreamPeerBuffer {
    pub use super::StreamPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StreamPeerExtension {
    pub use super::StreamPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_data_rawptr: (&str, u32) = ("_get_data", 298948178u32);
    pub type Sig_get_data_rawptr = (* mut u8, i32, * mut i32,);
    pub const get_partial_data_rawptr: (&str, u32) = ("_get_partial_data", 298948178u32);
    pub type Sig_get_partial_data_rawptr = (* mut u8, i32, * mut i32,);
    pub const put_data_rawptr: (&str, u32) = ("_put_data", 298948178u32);
    pub type Sig_put_data_rawptr = (* const u8, i32, * mut i32,);
    pub const put_partial_data_rawptr: (&str, u32) = ("_put_partial_data", 298948178u32);
    pub type Sig_put_partial_data_rawptr = (* const u8, i32, * mut i32,);
    pub const get_available_bytes: (&str, u32) = ("_get_available_bytes", 3905245786u32);
    pub type Sig_get_available_bytes = ();
    
}
pub mod StreamPeerTcp {
    pub use super::StreamPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StreamPeerTls {
    pub use super::StreamPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StyleBox {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const draw: (&str, u32) = ("_draw", 2275962004u32);
    pub type Sig_draw = (Rid, Rect2,);
    pub const get_draw_rect: (&str, u32) = ("_get_draw_rect", 408950903u32);
    pub type Sig_get_draw_rect = (Rect2,);
    pub const get_minimum_size: (&str, u32) = ("_get_minimum_size", 3341600327u32);
    pub type Sig_get_minimum_size = ();
    pub const test_mask: (&str, u32) = ("_test_mask", 3735564539u32);
    pub type Sig_test_mask = (Vector2, Rect2,);
    
}
pub mod StyleBoxEmpty {
    pub use super::StyleBox::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StyleBoxFlat {
    pub use super::StyleBox::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StyleBoxLine {
    pub use super::StyleBox::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod StyleBoxTexture {
    pub use super::StyleBox::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SubViewport {
    pub use super::Viewport::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SubViewportContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const propagate_input_event: (&str, u32) = ("_propagate_input_event", 3738334489u32);
    pub type Sig_propagate_input_event = (Option < Gd < crate::classes::InputEvent > >,);
    
}
pub mod SubtweenTweener {
    pub use super::Tweener::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SurfaceTool {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod SyntaxHighlighter {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_line_syntax_highlighting: (&str, u32) = ("_get_line_syntax_highlighting", 3485342025u32);
    pub type Sig_get_line_syntax_highlighting = (i32,);
    pub const clear_highlighting_cache: (&str, u32) = ("_clear_highlighting_cache", 3218959716u32);
    pub type Sig_clear_highlighting_cache = ();
    pub const update_cache: (&str, u32) = ("_update_cache", 3218959716u32);
    pub type Sig_update_cache = ();
    
}
pub mod SystemFont {
    pub use super::Font::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TcpServer {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TlsOptions {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TabBar {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TabContainer {
    pub use super::Container::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextEdit {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const handle_unicode_input: (&str, u32) = ("_handle_unicode_input", 3937882851u32);
    pub type Sig_handle_unicode_input = (i32, i32,);
    pub const backspace: (&str, u32) = ("_backspace", 1286410249u32);
    pub type Sig_backspace = (i32,);
    pub const cut: (&str, u32) = ("_cut", 1286410249u32);
    pub type Sig_cut = (i32,);
    pub const copy: (&str, u32) = ("_copy", 1286410249u32);
    pub type Sig_copy = (i32,);
    pub const paste: (&str, u32) = ("_paste", 1286410249u32);
    pub type Sig_paste = (i32,);
    pub const paste_primary_clipboard: (&str, u32) = ("_paste_primary_clipboard", 1286410249u32);
    pub type Sig_paste_primary_clipboard = (i32,);
    
}
pub mod TextLine {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextParagraph {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextServer {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextServerAdvanced {
    pub use super::TextServerExtension::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextServerDummy {
    pub use super::TextServerExtension::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextServerExtension {
    pub use super::TextServer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const has_feature: (&str, u32) = ("_has_feature", 3967367083u32);
    pub type Sig_has_feature = (crate::classes::text_server::Feature,);
    pub const get_name: (&str, u32) = ("_get_name", 201670096u32);
    pub type Sig_get_name = ();
    pub const get_features: (&str, u32) = ("_get_features", 3905245786u32);
    pub type Sig_get_features = ();
    pub const free_rid: (&str, u32) = ("_free_rid", 2722037293u32);
    pub type Sig_free_rid = (Rid,);
    pub const has: (&str, u32) = ("_has", 3521089500u32);
    pub type Sig_has = (Rid,);
    pub const load_support_data: (&str, u32) = ("_load_support_data", 2323990056u32);
    pub type Sig_load_support_data = (GString,);
    pub const get_support_data_filename: (&str, u32) = ("_get_support_data_filename", 201670096u32);
    pub type Sig_get_support_data_filename = ();
    pub const get_support_data_info: (&str, u32) = ("_get_support_data_info", 201670096u32);
    pub type Sig_get_support_data_info = ();
    pub const save_support_data: (&str, u32) = ("_save_support_data", 3927539163u32);
    pub type Sig_save_support_data = (GString,);
    pub const get_support_data: (&str, u32) = ("_get_support_data", 2362200018u32);
    pub type Sig_get_support_data = ();
    pub const is_locale_right_to_left: (&str, u32) = ("_is_locale_right_to_left", 3927539163u32);
    pub type Sig_is_locale_right_to_left = (GString,);
    pub const name_to_tag: (&str, u32) = ("_name_to_tag", 1321353865u32);
    pub type Sig_name_to_tag = (GString,);
    pub const tag_to_name: (&str, u32) = ("_tag_to_name", 844755477u32);
    pub type Sig_tag_to_name = (i64,);
    pub const create_font: (&str, u32) = ("_create_font", 529393457u32);
    pub type Sig_create_font = ();
    pub const create_font_linked_variation: (&str, u32) = ("_create_font_linked_variation", 41030802u32);
    pub type Sig_create_font_linked_variation = (Rid,);
    pub const font_set_data: (&str, u32) = ("_font_set_data", 1355495400u32);
    pub type Sig_font_set_data = (Rid, PackedByteArray,);
    pub const font_set_data_ptr_rawptr: (&str, u32) = ("_font_set_data_ptr", 4288446313u32);
    pub type Sig_font_set_data_ptr_rawptr = (Rid, * const u8, i64,);
    pub const font_set_face_index: (&str, u32) = ("_font_set_face_index", 3411492887u32);
    pub type Sig_font_set_face_index = (Rid, i64,);
    pub const font_get_face_index: (&str, u32) = ("_font_get_face_index", 2198884583u32);
    pub type Sig_font_get_face_index = (Rid,);
    pub const font_get_face_count: (&str, u32) = ("_font_get_face_count", 2198884583u32);
    pub type Sig_font_get_face_count = (Rid,);
    pub const font_set_style: (&str, u32) = ("_font_set_style", 898466325u32);
    pub type Sig_font_set_style = (Rid, crate::classes::text_server::FontStyle,);
    pub const font_get_style: (&str, u32) = ("_font_get_style", 3082502592u32);
    pub type Sig_font_get_style = (Rid,);
    pub const font_set_name: (&str, u32) = ("_font_set_name", 2726140452u32);
    pub type Sig_font_set_name = (Rid, GString,);
    pub const font_get_name: (&str, u32) = ("_font_get_name", 642473191u32);
    pub type Sig_font_get_name = (Rid,);
    pub const font_get_ot_name_strings: (&str, u32) = ("_font_get_ot_name_strings", 1882737106u32);
    pub type Sig_font_get_ot_name_strings = (Rid,);
    pub const font_set_style_name: (&str, u32) = ("_font_set_style_name", 2726140452u32);
    pub type Sig_font_set_style_name = (Rid, GString,);
    pub const font_get_style_name: (&str, u32) = ("_font_get_style_name", 642473191u32);
    pub type Sig_font_get_style_name = (Rid,);
    pub const font_set_weight: (&str, u32) = ("_font_set_weight", 3411492887u32);
    pub type Sig_font_set_weight = (Rid, i64,);
    pub const font_get_weight: (&str, u32) = ("_font_get_weight", 2198884583u32);
    pub type Sig_font_get_weight = (Rid,);
    pub const font_set_stretch: (&str, u32) = ("_font_set_stretch", 3411492887u32);
    pub type Sig_font_set_stretch = (Rid, i64,);
    pub const font_get_stretch: (&str, u32) = ("_font_get_stretch", 2198884583u32);
    pub type Sig_font_get_stretch = (Rid,);
    pub const font_set_antialiasing: (&str, u32) = ("_font_set_antialiasing", 958337235u32);
    pub type Sig_font_set_antialiasing = (Rid, crate::classes::text_server::FontAntialiasing,);
    pub const font_get_antialiasing: (&str, u32) = ("_font_get_antialiasing", 3389420495u32);
    pub type Sig_font_get_antialiasing = (Rid,);
    pub const font_set_disable_embedded_bitmaps: (&str, u32) = ("_font_set_disable_embedded_bitmaps", 1265174801u32);
    pub type Sig_font_set_disable_embedded_bitmaps = (Rid, bool,);
    pub const font_get_disable_embedded_bitmaps: (&str, u32) = ("_font_get_disable_embedded_bitmaps", 4155700596u32);
    pub type Sig_font_get_disable_embedded_bitmaps = (Rid,);
    pub const font_set_generate_mipmaps: (&str, u32) = ("_font_set_generate_mipmaps", 1265174801u32);
    pub type Sig_font_set_generate_mipmaps = (Rid, bool,);
    pub const font_get_generate_mipmaps: (&str, u32) = ("_font_get_generate_mipmaps", 4155700596u32);
    pub type Sig_font_get_generate_mipmaps = (Rid,);
    pub const font_set_multichannel_signed_distance_field: (&str, u32) = ("_font_set_multichannel_signed_distance_field", 1265174801u32);
    pub type Sig_font_set_multichannel_signed_distance_field = (Rid, bool,);
    pub const font_is_multichannel_signed_distance_field: (&str, u32) = ("_font_is_multichannel_signed_distance_field", 4155700596u32);
    pub type Sig_font_is_multichannel_signed_distance_field = (Rid,);
    pub const font_set_msdf_pixel_range: (&str, u32) = ("_font_set_msdf_pixel_range", 3411492887u32);
    pub type Sig_font_set_msdf_pixel_range = (Rid, i64,);
    pub const font_get_msdf_pixel_range: (&str, u32) = ("_font_get_msdf_pixel_range", 2198884583u32);
    pub type Sig_font_get_msdf_pixel_range = (Rid,);
    pub const font_set_msdf_size: (&str, u32) = ("_font_set_msdf_size", 3411492887u32);
    pub type Sig_font_set_msdf_size = (Rid, i64,);
    pub const font_get_msdf_size: (&str, u32) = ("_font_get_msdf_size", 2198884583u32);
    pub type Sig_font_get_msdf_size = (Rid,);
    pub const font_set_fixed_size: (&str, u32) = ("_font_set_fixed_size", 3411492887u32);
    pub type Sig_font_set_fixed_size = (Rid, i64,);
    pub const font_get_fixed_size: (&str, u32) = ("_font_get_fixed_size", 2198884583u32);
    pub type Sig_font_get_fixed_size = (Rid,);
    pub const font_set_fixed_size_scale_mode: (&str, u32) = ("_font_set_fixed_size_scale_mode", 1029390307u32);
    pub type Sig_font_set_fixed_size_scale_mode = (Rid, crate::classes::text_server::FixedSizeScaleMode,);
    pub const font_get_fixed_size_scale_mode: (&str, u32) = ("_font_get_fixed_size_scale_mode", 4113120379u32);
    pub type Sig_font_get_fixed_size_scale_mode = (Rid,);
    pub const font_set_allow_system_fallback: (&str, u32) = ("_font_set_allow_system_fallback", 1265174801u32);
    pub type Sig_font_set_allow_system_fallback = (Rid, bool,);
    pub const font_is_allow_system_fallback: (&str, u32) = ("_font_is_allow_system_fallback", 4155700596u32);
    pub type Sig_font_is_allow_system_fallback = (Rid,);
    pub const font_clear_system_fallback_cache: (&str, u32) = ("_font_clear_system_fallback_cache", 3218959716u32);
    pub type Sig_font_clear_system_fallback_cache = ();
    pub const font_set_force_autohinter: (&str, u32) = ("_font_set_force_autohinter", 1265174801u32);
    pub type Sig_font_set_force_autohinter = (Rid, bool,);
    pub const font_is_force_autohinter: (&str, u32) = ("_font_is_force_autohinter", 4155700596u32);
    pub type Sig_font_is_force_autohinter = (Rid,);
    pub const font_set_modulate_color_glyphs: (&str, u32) = ("_font_set_modulate_color_glyphs", 1265174801u32);
    pub type Sig_font_set_modulate_color_glyphs = (Rid, bool,);
    pub const font_is_modulate_color_glyphs: (&str, u32) = ("_font_is_modulate_color_glyphs", 4155700596u32);
    pub type Sig_font_is_modulate_color_glyphs = (Rid,);
    pub const font_set_hinting: (&str, u32) = ("_font_set_hinting", 1520010864u32);
    pub type Sig_font_set_hinting = (Rid, crate::classes::text_server::Hinting,);
    pub const font_get_hinting: (&str, u32) = ("_font_get_hinting", 3971592737u32);
    pub type Sig_font_get_hinting = (Rid,);
    pub const font_set_subpixel_positioning: (&str, u32) = ("_font_set_subpixel_positioning", 3830459669u32);
    pub type Sig_font_set_subpixel_positioning = (Rid, crate::classes::text_server::SubpixelPositioning,);
    pub const font_get_subpixel_positioning: (&str, u32) = ("_font_get_subpixel_positioning", 2752233671u32);
    pub type Sig_font_get_subpixel_positioning = (Rid,);
    pub const font_set_keep_rounding_remainders: (&str, u32) = ("_font_set_keep_rounding_remainders", 1265174801u32);
    pub type Sig_font_set_keep_rounding_remainders = (Rid, bool,);
    pub const font_get_keep_rounding_remainders: (&str, u32) = ("_font_get_keep_rounding_remainders", 4155700596u32);
    pub type Sig_font_get_keep_rounding_remainders = (Rid,);
    pub const font_set_embolden: (&str, u32) = ("_font_set_embolden", 1794382983u32);
    pub type Sig_font_set_embolden = (Rid, f64,);
    pub const font_get_embolden: (&str, u32) = ("_font_get_embolden", 866169185u32);
    pub type Sig_font_get_embolden = (Rid,);
    pub const font_set_spacing: (&str, u32) = ("_font_set_spacing", 1307259930u32);
    pub type Sig_font_set_spacing = (Rid, crate::classes::text_server::SpacingType, i64,);
    pub const font_get_spacing: (&str, u32) = ("_font_get_spacing", 1213653558u32);
    pub type Sig_font_get_spacing = (Rid, crate::classes::text_server::SpacingType,);
    pub const font_set_baseline_offset: (&str, u32) = ("_font_set_baseline_offset", 1794382983u32);
    pub type Sig_font_set_baseline_offset = (Rid, f64,);
    pub const font_get_baseline_offset: (&str, u32) = ("_font_get_baseline_offset", 866169185u32);
    pub type Sig_font_get_baseline_offset = (Rid,);
    pub const font_set_transform: (&str, u32) = ("_font_set_transform", 1246044741u32);
    pub type Sig_font_set_transform = (Rid, Transform2D,);
    pub const font_get_transform: (&str, u32) = ("_font_get_transform", 213527486u32);
    pub type Sig_font_get_transform = (Rid,);
    pub const font_set_variation_coordinates: (&str, u32) = ("_font_set_variation_coordinates", 1217542888u32);
    pub type Sig_font_set_variation_coordinates = (Rid, VarDictionary,);
    pub const font_get_variation_coordinates: (&str, u32) = ("_font_get_variation_coordinates", 1882737106u32);
    pub type Sig_font_get_variation_coordinates = (Rid,);
    pub const font_set_oversampling: (&str, u32) = ("_font_set_oversampling", 1794382983u32);
    pub type Sig_font_set_oversampling = (Rid, f64,);
    pub const font_get_oversampling: (&str, u32) = ("_font_get_oversampling", 866169185u32);
    pub type Sig_font_get_oversampling = (Rid,);
    pub const font_get_size_cache_list: (&str, u32) = ("_font_get_size_cache_list", 2684255073u32);
    pub type Sig_font_get_size_cache_list = (Rid,);
    pub const font_clear_size_cache: (&str, u32) = ("_font_clear_size_cache", 2722037293u32);
    pub type Sig_font_clear_size_cache = (Rid,);
    pub const font_remove_size_cache: (&str, u32) = ("_font_remove_size_cache", 2450610377u32);
    pub type Sig_font_remove_size_cache = (Rid, Vector2i,);
    pub const font_get_size_cache_info: (&str, u32) = ("_font_get_size_cache_info", 2684255073u32);
    pub type Sig_font_get_size_cache_info = (Rid,);
    pub const font_set_ascent: (&str, u32) = ("_font_set_ascent", 1892459533u32);
    pub type Sig_font_set_ascent = (Rid, i64, f64,);
    pub const font_get_ascent: (&str, u32) = ("_font_get_ascent", 755457166u32);
    pub type Sig_font_get_ascent = (Rid, i64,);
    pub const font_set_descent: (&str, u32) = ("_font_set_descent", 1892459533u32);
    pub type Sig_font_set_descent = (Rid, i64, f64,);
    pub const font_get_descent: (&str, u32) = ("_font_get_descent", 755457166u32);
    pub type Sig_font_get_descent = (Rid, i64,);
    pub const font_set_underline_position: (&str, u32) = ("_font_set_underline_position", 1892459533u32);
    pub type Sig_font_set_underline_position = (Rid, i64, f64,);
    pub const font_get_underline_position: (&str, u32) = ("_font_get_underline_position", 755457166u32);
    pub type Sig_font_get_underline_position = (Rid, i64,);
    pub const font_set_underline_thickness: (&str, u32) = ("_font_set_underline_thickness", 1892459533u32);
    pub type Sig_font_set_underline_thickness = (Rid, i64, f64,);
    pub const font_get_underline_thickness: (&str, u32) = ("_font_get_underline_thickness", 755457166u32);
    pub type Sig_font_get_underline_thickness = (Rid, i64,);
    pub const font_set_scale: (&str, u32) = ("_font_set_scale", 1892459533u32);
    pub type Sig_font_set_scale = (Rid, i64, f64,);
    pub const font_get_scale: (&str, u32) = ("_font_get_scale", 755457166u32);
    pub type Sig_font_get_scale = (Rid, i64,);
    pub const font_get_texture_count: (&str, u32) = ("_font_get_texture_count", 1311001310u32);
    pub type Sig_font_get_texture_count = (Rid, Vector2i,);
    pub const font_clear_textures: (&str, u32) = ("_font_clear_textures", 2450610377u32);
    pub type Sig_font_clear_textures = (Rid, Vector2i,);
    pub const font_remove_texture: (&str, u32) = ("_font_remove_texture", 3810512262u32);
    pub type Sig_font_remove_texture = (Rid, Vector2i, i64,);
    pub const font_set_texture_image: (&str, u32) = ("_font_set_texture_image", 2354485091u32);
    pub type Sig_font_set_texture_image = (Rid, Vector2i, i64, Option < Gd < crate::classes::Image > >,);
    pub const font_get_texture_image: (&str, u32) = ("_font_get_texture_image", 2451761155u32);
    pub type Sig_font_get_texture_image = (Rid, Vector2i, i64,);
    pub const font_set_texture_offsets: (&str, u32) = ("_font_set_texture_offsets", 3005398047u32);
    pub type Sig_font_set_texture_offsets = (Rid, Vector2i, i64, PackedInt32Array,);
    pub const font_get_texture_offsets: (&str, u32) = ("_font_get_texture_offsets", 3420028887u32);
    pub type Sig_font_get_texture_offsets = (Rid, Vector2i, i64,);
    pub const font_get_glyph_list: (&str, u32) = ("_font_get_glyph_list", 46086620u32);
    pub type Sig_font_get_glyph_list = (Rid, Vector2i,);
    pub const font_clear_glyphs: (&str, u32) = ("_font_clear_glyphs", 2450610377u32);
    pub type Sig_font_clear_glyphs = (Rid, Vector2i,);
    pub const font_remove_glyph: (&str, u32) = ("_font_remove_glyph", 3810512262u32);
    pub type Sig_font_remove_glyph = (Rid, Vector2i, i64,);
    pub const font_get_glyph_advance: (&str, u32) = ("_font_get_glyph_advance", 2555689501u32);
    pub type Sig_font_get_glyph_advance = (Rid, i64, i64,);
    pub const font_set_glyph_advance: (&str, u32) = ("_font_set_glyph_advance", 3219397315u32);
    pub type Sig_font_set_glyph_advance = (Rid, i64, i64, Vector2,);
    pub const font_get_glyph_offset: (&str, u32) = ("_font_get_glyph_offset", 513728628u32);
    pub type Sig_font_get_glyph_offset = (Rid, Vector2i, i64,);
    pub const font_set_glyph_offset: (&str, u32) = ("_font_set_glyph_offset", 1812632090u32);
    pub type Sig_font_set_glyph_offset = (Rid, Vector2i, i64, Vector2,);
    pub const font_get_glyph_size: (&str, u32) = ("_font_get_glyph_size", 513728628u32);
    pub type Sig_font_get_glyph_size = (Rid, Vector2i, i64,);
    pub const font_set_glyph_size: (&str, u32) = ("_font_set_glyph_size", 1812632090u32);
    pub type Sig_font_set_glyph_size = (Rid, Vector2i, i64, Vector2,);
    pub const font_get_glyph_uv_rect: (&str, u32) = ("_font_get_glyph_uv_rect", 2274268786u32);
    pub type Sig_font_get_glyph_uv_rect = (Rid, Vector2i, i64,);
    pub const font_set_glyph_uv_rect: (&str, u32) = ("_font_set_glyph_uv_rect", 1973324081u32);
    pub type Sig_font_set_glyph_uv_rect = (Rid, Vector2i, i64, Rect2,);
    pub const font_get_glyph_texture_idx: (&str, u32) = ("_font_get_glyph_texture_idx", 4292800474u32);
    pub type Sig_font_get_glyph_texture_idx = (Rid, Vector2i, i64,);
    pub const font_set_glyph_texture_idx: (&str, u32) = ("_font_set_glyph_texture_idx", 4254580980u32);
    pub type Sig_font_set_glyph_texture_idx = (Rid, Vector2i, i64, i64,);
    pub const font_get_glyph_texture_rid: (&str, u32) = ("_font_get_glyph_texture_rid", 1451696141u32);
    pub type Sig_font_get_glyph_texture_rid = (Rid, Vector2i, i64,);
    pub const font_get_glyph_texture_size: (&str, u32) = ("_font_get_glyph_texture_size", 513728628u32);
    pub type Sig_font_get_glyph_texture_size = (Rid, Vector2i, i64,);
    pub const font_get_glyph_contours: (&str, u32) = ("_font_get_glyph_contours", 2903964473u32);
    pub type Sig_font_get_glyph_contours = (Rid, i64, i64,);
    pub const font_get_kerning_list: (&str, u32) = ("_font_get_kerning_list", 1778388067u32);
    pub type Sig_font_get_kerning_list = (Rid, i64,);
    pub const font_clear_kerning_map: (&str, u32) = ("_font_clear_kerning_map", 3411492887u32);
    pub type Sig_font_clear_kerning_map = (Rid, i64,);
    pub const font_remove_kerning: (&str, u32) = ("_font_remove_kerning", 2141860016u32);
    pub type Sig_font_remove_kerning = (Rid, i64, Vector2i,);
    pub const font_set_kerning: (&str, u32) = ("_font_set_kerning", 3630965883u32);
    pub type Sig_font_set_kerning = (Rid, i64, Vector2i, Vector2,);
    pub const font_get_kerning: (&str, u32) = ("_font_get_kerning", 1019980169u32);
    pub type Sig_font_get_kerning = (Rid, i64, Vector2i,);
    pub const font_get_glyph_index: (&str, u32) = ("_font_get_glyph_index", 1765635060u32);
    pub type Sig_font_get_glyph_index = (Rid, i64, i64, i64,);
    pub const font_get_char_from_glyph_index: (&str, u32) = ("_font_get_char_from_glyph_index", 2156738276u32);
    pub type Sig_font_get_char_from_glyph_index = (Rid, i64, i64,);
    pub const font_has_char: (&str, u32) = ("_font_has_char", 3120086654u32);
    pub type Sig_font_has_char = (Rid, i64,);
    pub const font_get_supported_chars: (&str, u32) = ("_font_get_supported_chars", 642473191u32);
    pub type Sig_font_get_supported_chars = (Rid,);
    pub const font_get_supported_glyphs: (&str, u32) = ("_font_get_supported_glyphs", 788230395u32);
    pub type Sig_font_get_supported_glyphs = (Rid,);
    pub const font_render_range: (&str, u32) = ("_font_render_range", 4254580980u32);
    pub type Sig_font_render_range = (Rid, Vector2i, i64, i64,);
    pub const font_render_glyph: (&str, u32) = ("_font_render_glyph", 3810512262u32);
    pub type Sig_font_render_glyph = (Rid, Vector2i, i64,);
    pub const font_draw_glyph: (&str, u32) = ("_font_draw_glyph", 404525066u32);
    pub type Sig_font_draw_glyph = (Rid, Rid, i64, Vector2, i64, Color, f32,);
    pub const font_draw_glyph_outline: (&str, u32) = ("_font_draw_glyph_outline", 940535541u32);
    pub type Sig_font_draw_glyph_outline = (Rid, Rid, i64, i64, Vector2, i64, Color, f32,);
    pub const font_is_language_supported: (&str, u32) = ("_font_is_language_supported", 3199320846u32);
    pub type Sig_font_is_language_supported = (Rid, GString,);
    pub const font_set_language_support_override: (&str, u32) = ("_font_set_language_support_override", 2313957094u32);
    pub type Sig_font_set_language_support_override = (Rid, GString, bool,);
    pub const font_get_language_support_override: (&str, u32) = ("_font_get_language_support_override", 2829184646u32);
    pub type Sig_font_get_language_support_override = (Rid, GString,);
    pub const font_remove_language_support_override: (&str, u32) = ("_font_remove_language_support_override", 2726140452u32);
    pub type Sig_font_remove_language_support_override = (Rid, GString,);
    pub const font_get_language_support_overrides: (&str, u32) = ("_font_get_language_support_overrides", 2801473409u32);
    pub type Sig_font_get_language_support_overrides = (Rid,);
    pub const font_is_script_supported: (&str, u32) = ("_font_is_script_supported", 3199320846u32);
    pub type Sig_font_is_script_supported = (Rid, GString,);
    pub const font_set_script_support_override: (&str, u32) = ("_font_set_script_support_override", 2313957094u32);
    pub type Sig_font_set_script_support_override = (Rid, GString, bool,);
    pub const font_get_script_support_override: (&str, u32) = ("_font_get_script_support_override", 2829184646u32);
    pub type Sig_font_get_script_support_override = (Rid, GString,);
    pub const font_remove_script_support_override: (&str, u32) = ("_font_remove_script_support_override", 2726140452u32);
    pub type Sig_font_remove_script_support_override = (Rid, GString,);
    pub const font_get_script_support_overrides: (&str, u32) = ("_font_get_script_support_overrides", 2801473409u32);
    pub type Sig_font_get_script_support_overrides = (Rid,);
    pub const font_set_opentype_feature_overrides: (&str, u32) = ("_font_set_opentype_feature_overrides", 1217542888u32);
    pub type Sig_font_set_opentype_feature_overrides = (Rid, VarDictionary,);
    pub const font_get_opentype_feature_overrides: (&str, u32) = ("_font_get_opentype_feature_overrides", 1882737106u32);
    pub type Sig_font_get_opentype_feature_overrides = (Rid,);
    pub const font_supported_feature_list: (&str, u32) = ("_font_supported_feature_list", 1882737106u32);
    pub type Sig_font_supported_feature_list = (Rid,);
    pub const font_supported_variation_list: (&str, u32) = ("_font_supported_variation_list", 1882737106u32);
    pub type Sig_font_supported_variation_list = (Rid,);
    pub const font_get_global_oversampling: (&str, u32) = ("_font_get_global_oversampling", 1740695150u32);
    pub type Sig_font_get_global_oversampling = ();
    pub const font_set_global_oversampling: (&str, u32) = ("_font_set_global_oversampling", 373806689u32);
    pub type Sig_font_set_global_oversampling = (f64,);
    pub const reference_oversampling_level: (&str, u32) = ("_reference_oversampling_level", 373806689u32);
    pub type Sig_reference_oversampling_level = (f64,);
    pub const unreference_oversampling_level: (&str, u32) = ("_unreference_oversampling_level", 373806689u32);
    pub type Sig_unreference_oversampling_level = (f64,);
    pub const get_hex_code_box_size: (&str, u32) = ("_get_hex_code_box_size", 3016396712u32);
    pub type Sig_get_hex_code_box_size = (i64, i64,);
    pub const draw_hex_code_box: (&str, u32) = ("_draw_hex_code_box", 1602046441u32);
    pub type Sig_draw_hex_code_box = (Rid, i64, Vector2, i64, Color,);
    pub const create_shaped_text: (&str, u32) = ("_create_shaped_text", 1431128392u32);
    pub type Sig_create_shaped_text = (crate::classes::text_server::Direction, crate::classes::text_server::Orientation,);
    pub const shaped_text_clear: (&str, u32) = ("_shaped_text_clear", 2722037293u32);
    pub type Sig_shaped_text_clear = (Rid,);
    pub const shaped_text_set_direction: (&str, u32) = ("_shaped_text_set_direction", 4276135416u32);
    pub type Sig_shaped_text_set_direction = (Rid, crate::classes::text_server::Direction,);
    pub const shaped_text_get_direction: (&str, u32) = ("_shaped_text_get_direction", 3065904362u32);
    pub type Sig_shaped_text_get_direction = (Rid,);
    pub const shaped_text_get_inferred_direction: (&str, u32) = ("_shaped_text_get_inferred_direction", 3065904362u32);
    pub type Sig_shaped_text_get_inferred_direction = (Rid,);
    pub const shaped_text_set_bidi_override: (&str, u32) = ("_shaped_text_set_bidi_override", 684822712u32);
    pub type Sig_shaped_text_set_bidi_override = (Rid, VarArray,);
    pub const shaped_text_set_custom_punctuation: (&str, u32) = ("_shaped_text_set_custom_punctuation", 2726140452u32);
    pub type Sig_shaped_text_set_custom_punctuation = (Rid, GString,);
    pub const shaped_text_get_custom_punctuation: (&str, u32) = ("_shaped_text_get_custom_punctuation", 642473191u32);
    pub type Sig_shaped_text_get_custom_punctuation = (Rid,);
    pub const shaped_text_set_custom_ellipsis: (&str, u32) = ("_shaped_text_set_custom_ellipsis", 3411492887u32);
    pub type Sig_shaped_text_set_custom_ellipsis = (Rid, i64,);
    pub const shaped_text_get_custom_ellipsis: (&str, u32) = ("_shaped_text_get_custom_ellipsis", 2198884583u32);
    pub type Sig_shaped_text_get_custom_ellipsis = (Rid,);
    pub const shaped_text_set_orientation: (&str, u32) = ("_shaped_text_set_orientation", 2306444742u32);
    pub type Sig_shaped_text_set_orientation = (Rid, crate::classes::text_server::Orientation,);
    pub const shaped_text_get_orientation: (&str, u32) = ("_shaped_text_get_orientation", 3142708106u32);
    pub type Sig_shaped_text_get_orientation = (Rid,);
    pub const shaped_text_set_preserve_invalid: (&str, u32) = ("_shaped_text_set_preserve_invalid", 1265174801u32);
    pub type Sig_shaped_text_set_preserve_invalid = (Rid, bool,);
    pub const shaped_text_get_preserve_invalid: (&str, u32) = ("_shaped_text_get_preserve_invalid", 4155700596u32);
    pub type Sig_shaped_text_get_preserve_invalid = (Rid,);
    pub const shaped_text_set_preserve_control: (&str, u32) = ("_shaped_text_set_preserve_control", 1265174801u32);
    pub type Sig_shaped_text_set_preserve_control = (Rid, bool,);
    pub const shaped_text_get_preserve_control: (&str, u32) = ("_shaped_text_get_preserve_control", 4155700596u32);
    pub type Sig_shaped_text_get_preserve_control = (Rid,);
    pub const shaped_text_set_spacing: (&str, u32) = ("_shaped_text_set_spacing", 1307259930u32);
    pub type Sig_shaped_text_set_spacing = (Rid, crate::classes::text_server::SpacingType, i64,);
    pub const shaped_text_get_spacing: (&str, u32) = ("_shaped_text_get_spacing", 1213653558u32);
    pub type Sig_shaped_text_get_spacing = (Rid, crate::classes::text_server::SpacingType,);
    pub const shaped_text_add_string: (&str, u32) = ("_shaped_text_add_string", 875249313u32);
    pub type Sig_shaped_text_add_string = (Rid, GString, Array < Rid >, i64, VarDictionary, GString, Variant,);
    pub const shaped_text_add_object: (&str, u32) = ("_shaped_text_add_object", 2452224230u32);
    pub type Sig_shaped_text_add_object = (Rid, Variant, Vector2, crate::global::InlineAlignment, i64, f64,);
    pub const shaped_text_resize_object: (&str, u32) = ("_shaped_text_resize_object", 2747466775u32);
    pub type Sig_shaped_text_resize_object = (Rid, Variant, Vector2, crate::global::InlineAlignment, f64,);
    pub const shaped_get_text: (&str, u32) = ("_shaped_get_text", 642473191u32);
    pub type Sig_shaped_get_text = (Rid,);
    pub const shaped_get_span_count: (&str, u32) = ("_shaped_get_span_count", 2198884583u32);
    pub type Sig_shaped_get_span_count = (Rid,);
    pub const shaped_get_span_meta: (&str, u32) = ("_shaped_get_span_meta", 4069510997u32);
    pub type Sig_shaped_get_span_meta = (Rid, i64,);
    pub const shaped_get_span_embedded_object: (&str, u32) = ("_shaped_get_span_embedded_object", 4069510997u32);
    pub type Sig_shaped_get_span_embedded_object = (Rid, i64,);
    pub const shaped_get_span_text: (&str, u32) = ("_shaped_get_span_text", 1464764419u32);
    pub type Sig_shaped_get_span_text = (Rid, i64,);
    pub const shaped_get_span_object: (&str, u32) = ("_shaped_get_span_object", 4069510997u32);
    pub type Sig_shaped_get_span_object = (Rid, i64,);
    pub const shaped_set_span_update_font: (&str, u32) = ("_shaped_set_span_update_font", 2569459151u32);
    pub type Sig_shaped_set_span_update_font = (Rid, i64, Array < Rid >, i64, VarDictionary,);
    pub const shaped_get_run_count: (&str, u32) = ("_shaped_get_run_count", 2198884583u32);
    pub type Sig_shaped_get_run_count = (Rid,);
    pub const shaped_get_run_text: (&str, u32) = ("_shaped_get_run_text", 1464764419u32);
    pub type Sig_shaped_get_run_text = (Rid, i64,);
    pub const shaped_get_run_range: (&str, u32) = ("_shaped_get_run_range", 4069534484u32);
    pub type Sig_shaped_get_run_range = (Rid, i64,);
    pub const shaped_get_run_font_rid: (&str, u32) = ("_shaped_get_run_font_rid", 1066463050u32);
    pub type Sig_shaped_get_run_font_rid = (Rid, i64,);
    pub const shaped_get_run_font_size: (&str, u32) = ("_shaped_get_run_font_size", 1120910005u32);
    pub type Sig_shaped_get_run_font_size = (Rid, i64,);
    pub const shaped_get_run_language: (&str, u32) = ("_shaped_get_run_language", 1464764419u32);
    pub type Sig_shaped_get_run_language = (Rid, i64,);
    pub const shaped_get_run_direction: (&str, u32) = ("_shaped_get_run_direction", 2413896864u32);
    pub type Sig_shaped_get_run_direction = (Rid, i64,);
    pub const shaped_get_run_object: (&str, u32) = ("_shaped_get_run_object", 4069510997u32);
    pub type Sig_shaped_get_run_object = (Rid, i64,);
    pub const shaped_text_substr: (&str, u32) = ("_shaped_text_substr", 1937682086u32);
    pub type Sig_shaped_text_substr = (Rid, i64, i64,);
    pub const shaped_text_get_parent: (&str, u32) = ("_shaped_text_get_parent", 3814569979u32);
    pub type Sig_shaped_text_get_parent = (Rid,);
    pub const shaped_text_fit_to_width: (&str, u32) = ("_shaped_text_fit_to_width", 1426448222u32);
    pub type Sig_shaped_text_fit_to_width = (Rid, f64, crate::classes::text_server::JustificationFlag,);
    pub const shaped_text_tab_align: (&str, u32) = ("_shaped_text_tab_align", 1283669550u32);
    pub type Sig_shaped_text_tab_align = (Rid, PackedFloat32Array,);
    pub const shaped_text_shape: (&str, u32) = ("_shaped_text_shape", 3521089500u32);
    pub type Sig_shaped_text_shape = (Rid,);
    pub const shaped_text_update_breaks: (&str, u32) = ("_shaped_text_update_breaks", 3521089500u32);
    pub type Sig_shaped_text_update_breaks = (Rid,);
    pub const shaped_text_update_justification_ops: (&str, u32) = ("_shaped_text_update_justification_ops", 3521089500u32);
    pub type Sig_shaped_text_update_justification_ops = (Rid,);
    pub const shaped_text_is_ready: (&str, u32) = ("_shaped_text_is_ready", 4155700596u32);
    pub type Sig_shaped_text_is_ready = (Rid,);
    pub const shaped_text_get_glyphs_rawptr: (&str, u32) = ("_shaped_text_get_glyphs", 2198884583u32);
    pub type Sig_shaped_text_get_glyphs_rawptr = (Rid,);
    pub const shaped_text_sort_logical_rawptr: (&str, u32) = ("_shaped_text_sort_logical", 3917799429u32);
    pub type Sig_shaped_text_sort_logical_rawptr = (Rid,);
    pub const shaped_text_get_glyph_count: (&str, u32) = ("_shaped_text_get_glyph_count", 2198884583u32);
    pub type Sig_shaped_text_get_glyph_count = (Rid,);
    pub const shaped_text_get_range: (&str, u32) = ("_shaped_text_get_range", 733700038u32);
    pub type Sig_shaped_text_get_range = (Rid,);
    pub const shaped_text_get_line_breaks_adv: (&str, u32) = ("_shaped_text_get_line_breaks_adv", 1488467363u32);
    pub type Sig_shaped_text_get_line_breaks_adv = (Rid, PackedFloat32Array, i64, bool, crate::classes::text_server::LineBreakFlag,);
    pub const shaped_text_get_line_breaks: (&str, u32) = ("_shaped_text_get_line_breaks", 3131311977u32);
    pub type Sig_shaped_text_get_line_breaks = (Rid, f64, i64, crate::classes::text_server::LineBreakFlag,);
    pub const shaped_text_get_word_breaks: (&str, u32) = ("_shaped_text_get_word_breaks", 2423529412u32);
    pub type Sig_shaped_text_get_word_breaks = (Rid, crate::classes::text_server::GraphemeFlag, crate::classes::text_server::GraphemeFlag,);
    pub const shaped_text_get_trim_pos: (&str, u32) = ("_shaped_text_get_trim_pos", 2198884583u32);
    pub type Sig_shaped_text_get_trim_pos = (Rid,);
    pub const shaped_text_get_ellipsis_pos: (&str, u32) = ("_shaped_text_get_ellipsis_pos", 2198884583u32);
    pub type Sig_shaped_text_get_ellipsis_pos = (Rid,);
    pub const shaped_text_get_ellipsis_glyph_count: (&str, u32) = ("_shaped_text_get_ellipsis_glyph_count", 2198884583u32);
    pub type Sig_shaped_text_get_ellipsis_glyph_count = (Rid,);
    pub const shaped_text_get_ellipsis_glyphs_rawptr: (&str, u32) = ("_shaped_text_get_ellipsis_glyphs", 2198884583u32);
    pub type Sig_shaped_text_get_ellipsis_glyphs_rawptr = (Rid,);
    pub const shaped_text_overrun_trim_to_width: (&str, u32) = ("_shaped_text_overrun_trim_to_width", 3364950921u32);
    pub type Sig_shaped_text_overrun_trim_to_width = (Rid, f64, crate::classes::text_server::TextOverrunFlag,);
    pub const shaped_text_get_objects: (&str, u32) = ("_shaped_text_get_objects", 2684255073u32);
    pub type Sig_shaped_text_get_objects = (Rid,);
    pub const shaped_text_get_object_rect: (&str, u32) = ("_shaped_text_get_object_rect", 447978354u32);
    pub type Sig_shaped_text_get_object_rect = (Rid, Variant,);
    pub const shaped_text_get_object_range: (&str, u32) = ("_shaped_text_get_object_range", 2524675647u32);
    pub type Sig_shaped_text_get_object_range = (Rid, Variant,);
    pub const shaped_text_get_object_glyph: (&str, u32) = ("_shaped_text_get_object_glyph", 1260085030u32);
    pub type Sig_shaped_text_get_object_glyph = (Rid, Variant,);
    pub const shaped_text_get_size: (&str, u32) = ("_shaped_text_get_size", 2440833711u32);
    pub type Sig_shaped_text_get_size = (Rid,);
    pub const shaped_text_get_ascent: (&str, u32) = ("_shaped_text_get_ascent", 866169185u32);
    pub type Sig_shaped_text_get_ascent = (Rid,);
    pub const shaped_text_get_descent: (&str, u32) = ("_shaped_text_get_descent", 866169185u32);
    pub type Sig_shaped_text_get_descent = (Rid,);
    pub const shaped_text_get_width: (&str, u32) = ("_shaped_text_get_width", 866169185u32);
    pub type Sig_shaped_text_get_width = (Rid,);
    pub const shaped_text_get_underline_position: (&str, u32) = ("_shaped_text_get_underline_position", 866169185u32);
    pub type Sig_shaped_text_get_underline_position = (Rid,);
    pub const shaped_text_get_underline_thickness: (&str, u32) = ("_shaped_text_get_underline_thickness", 866169185u32);
    pub type Sig_shaped_text_get_underline_thickness = (Rid,);
    pub const shaped_text_get_dominant_direction_in_range: (&str, u32) = ("_shaped_text_get_dominant_direction_in_range", 2156738276u32);
    pub type Sig_shaped_text_get_dominant_direction_in_range = (Rid, i64, i64,);
    pub const shaped_text_get_carets_rawptr: (&str, u32) = ("_shaped_text_get_carets", 1191777527u32);
    pub type Sig_shaped_text_get_carets_rawptr = (Rid, i64, * mut CaretInfo,);
    pub const shaped_text_get_selection: (&str, u32) = ("_shaped_text_get_selection", 3714187733u32);
    pub type Sig_shaped_text_get_selection = (Rid, i64, i64,);
    pub const shaped_text_hit_test_grapheme: (&str, u32) = ("_shaped_text_hit_test_grapheme", 3149310417u32);
    pub type Sig_shaped_text_hit_test_grapheme = (Rid, f64,);
    pub const shaped_text_hit_test_position: (&str, u32) = ("_shaped_text_hit_test_position", 3149310417u32);
    pub type Sig_shaped_text_hit_test_position = (Rid, f64,);
    pub const shaped_text_draw: (&str, u32) = ("_shaped_text_draw", 2079930245u32);
    pub type Sig_shaped_text_draw = (Rid, Rid, Vector2, f64, f64, Color, f32,);
    pub const shaped_text_draw_outline: (&str, u32) = ("_shaped_text_draw_outline", 601976754u32);
    pub type Sig_shaped_text_draw_outline = (Rid, Rid, Vector2, f64, f64, i64, Color, f32,);
    pub const shaped_text_get_grapheme_bounds: (&str, u32) = ("_shaped_text_get_grapheme_bounds", 2546185844u32);
    pub type Sig_shaped_text_get_grapheme_bounds = (Rid, i64,);
    pub const shaped_text_next_grapheme_pos: (&str, u32) = ("_shaped_text_next_grapheme_pos", 1120910005u32);
    pub type Sig_shaped_text_next_grapheme_pos = (Rid, i64,);
    pub const shaped_text_prev_grapheme_pos: (&str, u32) = ("_shaped_text_prev_grapheme_pos", 1120910005u32);
    pub type Sig_shaped_text_prev_grapheme_pos = (Rid, i64,);
    pub const shaped_text_get_character_breaks: (&str, u32) = ("_shaped_text_get_character_breaks", 788230395u32);
    pub type Sig_shaped_text_get_character_breaks = (Rid,);
    pub const shaped_text_next_character_pos: (&str, u32) = ("_shaped_text_next_character_pos", 1120910005u32);
    pub type Sig_shaped_text_next_character_pos = (Rid, i64,);
    pub const shaped_text_prev_character_pos: (&str, u32) = ("_shaped_text_prev_character_pos", 1120910005u32);
    pub type Sig_shaped_text_prev_character_pos = (Rid, i64,);
    pub const shaped_text_closest_character_pos: (&str, u32) = ("_shaped_text_closest_character_pos", 1120910005u32);
    pub type Sig_shaped_text_closest_character_pos = (Rid, i64,);
    pub const format_number: (&str, u32) = ("_format_number", 315676799u32);
    pub type Sig_format_number = (GString, GString,);
    pub const parse_number: (&str, u32) = ("_parse_number", 315676799u32);
    pub type Sig_parse_number = (GString, GString,);
    pub const percent_sign: (&str, u32) = ("_percent_sign", 3135753539u32);
    pub type Sig_percent_sign = (GString,);
    pub const strip_diacritics: (&str, u32) = ("_strip_diacritics", 3135753539u32);
    pub type Sig_strip_diacritics = (GString,);
    pub const is_valid_identifier: (&str, u32) = ("_is_valid_identifier", 3927539163u32);
    pub type Sig_is_valid_identifier = (GString,);
    pub const is_valid_letter: (&str, u32) = ("_is_valid_letter", 1116898809u32);
    pub type Sig_is_valid_letter = (u64,);
    pub const string_get_word_breaks: (&str, u32) = ("_string_get_word_breaks", 3658450588u32);
    pub type Sig_string_get_word_breaks = (GString, GString, i64,);
    pub const string_get_character_breaks: (&str, u32) = ("_string_get_character_breaks", 2509056759u32);
    pub type Sig_string_get_character_breaks = (GString, GString,);
    pub const is_confusable: (&str, u32) = ("_is_confusable", 1433197768u32);
    pub type Sig_is_confusable = (GString, PackedStringArray,);
    pub const spoof_check: (&str, u32) = ("_spoof_check", 3927539163u32);
    pub type Sig_spoof_check = (GString,);
    pub const string_to_upper: (&str, u32) = ("_string_to_upper", 315676799u32);
    pub type Sig_string_to_upper = (GString, GString,);
    pub const string_to_lower: (&str, u32) = ("_string_to_lower", 315676799u32);
    pub type Sig_string_to_lower = (GString, GString,);
    pub const string_to_title: (&str, u32) = ("_string_to_title", 315676799u32);
    pub type Sig_string_to_title = (GString, GString,);
    pub const parse_structured_text: (&str, u32) = ("_parse_structured_text", 3310685015u32);
    pub type Sig_parse_structured_text = (crate::classes::text_server::StructuredTextParser, VarArray, GString,);
    pub const cleanup: (&str, u32) = ("_cleanup", 3218959716u32);
    pub type Sig_cleanup = ();
    
}
pub mod TextServerManager {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Texture {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Texture2D {
    pub use super::Texture::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_width: (&str, u32) = ("_get_width", 3905245786u32);
    pub type Sig_get_width = ();
    pub const get_height: (&str, u32) = ("_get_height", 3905245786u32);
    pub type Sig_get_height = ();
    pub const is_pixel_opaque: (&str, u32) = ("_is_pixel_opaque", 2522259332u32);
    pub type Sig_is_pixel_opaque = (i32, i32,);
    pub const has_alpha: (&str, u32) = ("_has_alpha", 36873697u32);
    pub type Sig_has_alpha = ();
    pub const draw: (&str, u32) = ("_draw", 1384643611u32);
    pub type Sig_draw = (Rid, Vector2, Color, bool,);
    pub const draw_rect: (&str, u32) = ("_draw_rect", 3819628907u32);
    pub type Sig_draw_rect = (Rid, Rect2, bool, Color, bool,);
    pub const draw_rect_region: (&str, u32) = ("_draw_rect_region", 4094143664u32);
    pub type Sig_draw_rect_region = (Rid, Rect2, Rect2, Color, bool, bool,);
    
}
pub mod Texture2DArray {
    pub use super::ImageTextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Texture2DArrayRd {
    pub use super::TextureLayeredRd::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Texture2Drd {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Texture3D {
    pub use super::Texture::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_format: (&str, u32) = ("_get_format", 3847873762u32);
    pub type Sig_get_format = ();
    pub const get_width: (&str, u32) = ("_get_width", 3905245786u32);
    pub type Sig_get_width = ();
    pub const get_height: (&str, u32) = ("_get_height", 3905245786u32);
    pub type Sig_get_height = ();
    pub const get_depth: (&str, u32) = ("_get_depth", 3905245786u32);
    pub type Sig_get_depth = ();
    pub const has_mipmaps: (&str, u32) = ("_has_mipmaps", 36873697u32);
    pub type Sig_has_mipmaps = ();
    pub const get_data: (&str, u32) = ("_get_data", 3995934104u32);
    pub type Sig_get_data = ();
    
}
pub mod Texture3Drd {
    pub use super::Texture3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextureButton {
    pub use super::BaseButton::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextureCubemapArrayRd {
    pub use super::TextureLayeredRd::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextureCubemapRd {
    pub use super::TextureLayeredRd::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextureLayered {
    pub use super::Texture::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_format: (&str, u32) = ("_get_format", 3847873762u32);
    pub type Sig_get_format = ();
    pub const get_layered_type: (&str, u32) = ("_get_layered_type", 3905245786u32);
    pub type Sig_get_layered_type = ();
    pub const get_width: (&str, u32) = ("_get_width", 3905245786u32);
    pub type Sig_get_width = ();
    pub const get_height: (&str, u32) = ("_get_height", 3905245786u32);
    pub type Sig_get_height = ();
    pub const get_layers: (&str, u32) = ("_get_layers", 3905245786u32);
    pub type Sig_get_layers = ();
    pub const has_mipmaps: (&str, u32) = ("_has_mipmaps", 36873697u32);
    pub type Sig_has_mipmaps = ();
    pub const get_layer_data: (&str, u32) = ("_get_layer_data", 3655284255u32);
    pub type Sig_get_layer_data = (i32,);
    
}
pub mod TextureLayeredRd {
    pub use super::TextureLayered::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextureProgressBar {
    pub use super::Range::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TextureRect {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Theme {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ThemeDb {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TileData {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TileMap {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const use_tile_data_runtime_update: (&str, u32) = ("_use_tile_data_runtime_update", 3957903770u32);
    pub type Sig_use_tile_data_runtime_update = (i32, Vector2i,);
    pub const tile_data_runtime_update: (&str, u32) = ("_tile_data_runtime_update", 4223434291u32);
    pub type Sig_tile_data_runtime_update = (i32, Vector2i, Option < Gd < crate::classes::TileData > >,);
    
}
pub mod TileMapLayer {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const use_tile_data_runtime_update: (&str, u32) = ("_use_tile_data_runtime_update", 3715736492u32);
    pub type Sig_use_tile_data_runtime_update = (Vector2i,);
    pub const tile_data_runtime_update: (&str, u32) = ("_tile_data_runtime_update", 1627322126u32);
    pub type Sig_tile_data_runtime_update = (Vector2i, Option < Gd < crate::classes::TileData > >,);
    pub const update_cells: (&str, u32) = ("_update_cells", 3156113851u32);
    pub type Sig_update_cells = (Array < Vector2i >, bool,);
    
}
pub mod TileMapPattern {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TileSet {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TileSetAtlasSource {
    pub use super::TileSetSource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TileSetScenesCollectionSource {
    pub use super::TileSetSource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TileSetSource {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Time {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Timer {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TorusMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TouchScreenButton {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Translation {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_plural_message: (&str, u32) = ("_get_plural_message", 1970324172u32);
    pub type Sig_get_plural_message = (StringName, StringName, i32, StringName,);
    pub const get_message: (&str, u32) = ("_get_message", 3639719779u32);
    pub type Sig_get_message = (StringName, StringName,);
    
}
pub mod TranslationDomain {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TranslationServer {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Tree {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TreeItem {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TriangleMesh {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod TubeTrailMesh {
    pub use super::PrimitiveMesh::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Tween {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Tweener {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod UdpServer {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Upnp {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod UpnpDevice {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod UndoRedo {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod UniformSetCacheRd {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VBoxContainer {
    pub use super::BoxContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VFlowContainer {
    pub use super::FlowContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VScrollBar {
    pub use super::ScrollBar::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VSeparator {
    pub use super::Separator::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VSlider {
    pub use super::Slider::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VSplitContainer {
    pub use super::SplitContainer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VehicleBody3D {
    pub use super::RigidBody3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VehicleWheel3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VideoStream {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const instantiate_playback: (&str, u32) = ("_instantiate_playback", 294648086u32);
    pub type Sig_instantiate_playback = ();
    
}
pub mod VideoStreamPlayback {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const stop: (&str, u32) = ("_stop", 3218959716u32);
    pub type Sig_stop = ();
    pub const play: (&str, u32) = ("_play", 3218959716u32);
    pub type Sig_play = ();
    pub const is_playing: (&str, u32) = ("_is_playing", 36873697u32);
    pub type Sig_is_playing = ();
    pub const set_paused: (&str, u32) = ("_set_paused", 2586408642u32);
    pub type Sig_set_paused = (bool,);
    pub const is_paused: (&str, u32) = ("_is_paused", 36873697u32);
    pub type Sig_is_paused = ();
    pub const get_length: (&str, u32) = ("_get_length", 1740695150u32);
    pub type Sig_get_length = ();
    pub const get_playback_position: (&str, u32) = ("_get_playback_position", 1740695150u32);
    pub type Sig_get_playback_position = ();
    pub const seek: (&str, u32) = ("_seek", 373806689u32);
    pub type Sig_seek = (f64,);
    pub const set_audio_track: (&str, u32) = ("_set_audio_track", 1286410249u32);
    pub type Sig_set_audio_track = (i32,);
    pub const get_texture: (&str, u32) = ("_get_texture", 3635182373u32);
    pub type Sig_get_texture = ();
    pub const update: (&str, u32) = ("_update", 373806689u32);
    pub type Sig_update = (f64,);
    pub const get_channels: (&str, u32) = ("_get_channels", 3905245786u32);
    pub type Sig_get_channels = ();
    pub const get_mix_rate: (&str, u32) = ("_get_mix_rate", 3905245786u32);
    pub type Sig_get_mix_rate = ();
    
}
pub mod VideoStreamPlayer {
    pub use super::Control::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VideoStreamTheora {
    pub use super::VideoStream::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Viewport {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ViewportTexture {
    pub use super::Texture2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisibleOnScreenEnabler2D {
    pub use super::VisibleOnScreenNotifier2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisibleOnScreenEnabler3D {
    pub use super::VisibleOnScreenNotifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisibleOnScreenNotifier2D {
    pub use super::Node2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisibleOnScreenNotifier3D {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualInstance3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_aabb: (&str, u32) = ("_get_aabb", 1068685055u32);
    pub type Sig_get_aabb = ();
    
}
pub mod VisualShader {
    pub use super::Shader::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNode {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeBillboard {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeBooleanConstant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeBooleanParameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeClamp {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeColorConstant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeColorFunc {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeColorOp {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeColorParameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeComment {
    pub use super::VisualShaderNodeFrame::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeCompare {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeConstant {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeCubemap {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeCubemapParameter {
    pub use super::VisualShaderNodeTextureParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeCurveTexture {
    pub use super::VisualShaderNodeResizableBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeCurveXyzTexture {
    pub use super::VisualShaderNodeResizableBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeCustom {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_name: (&str, u32) = ("_get_name", 201670096u32);
    pub type Sig_get_name = ();
    pub const get_description: (&str, u32) = ("_get_description", 201670096u32);
    pub type Sig_get_description = ();
    pub const get_category: (&str, u32) = ("_get_category", 201670096u32);
    pub type Sig_get_category = ();
    pub const get_return_icon_type: (&str, u32) = ("_get_return_icon_type", 1287173294u32);
    pub type Sig_get_return_icon_type = ();
    pub const get_input_port_count: (&str, u32) = ("_get_input_port_count", 3905245786u32);
    pub type Sig_get_input_port_count = ();
    pub const get_input_port_type: (&str, u32) = ("_get_input_port_type", 4102573379u32);
    pub type Sig_get_input_port_type = (i32,);
    pub const get_input_port_name: (&str, u32) = ("_get_input_port_name", 844755477u32);
    pub type Sig_get_input_port_name = (i32,);
    pub const get_input_port_default_value: (&str, u32) = ("_get_input_port_default_value", 4227898402u32);
    pub type Sig_get_input_port_default_value = (i32,);
    pub const get_default_input_port: (&str, u32) = ("_get_default_input_port", 1894493699u32);
    pub type Sig_get_default_input_port = (crate::classes::visual_shader_node::PortType,);
    pub const get_output_port_count: (&str, u32) = ("_get_output_port_count", 3905245786u32);
    pub type Sig_get_output_port_count = ();
    pub const get_output_port_type: (&str, u32) = ("_get_output_port_type", 4102573379u32);
    pub type Sig_get_output_port_type = (i32,);
    pub const get_output_port_name: (&str, u32) = ("_get_output_port_name", 844755477u32);
    pub type Sig_get_output_port_name = (i32,);
    pub const get_property_count: (&str, u32) = ("_get_property_count", 3905245786u32);
    pub type Sig_get_property_count = ();
    pub const get_property_name: (&str, u32) = ("_get_property_name", 844755477u32);
    pub type Sig_get_property_name = (i32,);
    pub const get_property_default_index: (&str, u32) = ("_get_property_default_index", 923996154u32);
    pub type Sig_get_property_default_index = (i32,);
    pub const get_property_options: (&str, u32) = ("_get_property_options", 647634434u32);
    pub type Sig_get_property_options = (i32,);
    pub const get_code: (&str, u32) = ("_get_code", 4287175357u32);
    pub type Sig_get_code = (Array < GString >, Array < GString >, crate::classes::shader::Mode, crate::classes::visual_shader::Type,);
    pub const get_func_code: (&str, u32) = ("_get_func_code", 1924221678u32);
    pub type Sig_get_func_code = (crate::classes::shader::Mode, crate::classes::visual_shader::Type,);
    pub const get_global_code: (&str, u32) = ("_get_global_code", 3956542358u32);
    pub type Sig_get_global_code = (crate::classes::shader::Mode,);
    pub const is_highend: (&str, u32) = ("_is_highend", 36873697u32);
    pub type Sig_is_highend = ();
    pub const is_available: (&str, u32) = ("_is_available", 1932120545u32);
    pub type Sig_is_available = (crate::classes::shader::Mode, crate::classes::visual_shader::Type,);
    
}
pub mod VisualShaderNodeDerivativeFunc {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeDeterminant {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeDistanceFade {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeDotProduct {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeExpression {
    pub use super::VisualShaderNodeGroupBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeFaceForward {
    pub use super::VisualShaderNodeVectorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeFloatConstant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeFloatFunc {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeFloatOp {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeFloatParameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeFrame {
    pub use super::VisualShaderNodeResizableBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeFresnel {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeGlobalExpression {
    pub use super::VisualShaderNodeExpression::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeGroupBase {
    pub use super::VisualShaderNodeResizableBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeIf {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeInput {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeIntConstant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeIntFunc {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeIntOp {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeIntParameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeIs {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeLinearSceneDepth {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeMix {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeMultiplyAdd {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeOuterProduct {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeOutput {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParameter {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParameterRef {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleAccelerator {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleBoxEmitter {
    pub use super::VisualShaderNodeParticleEmitter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleConeVelocity {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleEmit {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleEmitter {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleMeshEmitter {
    pub use super::VisualShaderNodeParticleEmitter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleMultiplyByAxisAngle {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleOutput {
    pub use super::VisualShaderNodeOutput::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleRandomness {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleRingEmitter {
    pub use super::VisualShaderNodeParticleEmitter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeParticleSphereEmitter {
    pub use super::VisualShaderNodeParticleEmitter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeProximityFade {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeRandomRange {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeRemap {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeReroute {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeResizableBase {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeRotationByAxis {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeSdfRaymarch {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeSdfToScreenUv {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeSample3D {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeScreenNormalWorldSpace {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeScreenUvToSdf {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeSmoothStep {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeStep {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeSwitch {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTexture {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTexture2DArray {
    pub use super::VisualShaderNodeSample3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTexture2DArrayParameter {
    pub use super::VisualShaderNodeTextureParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTexture2DParameter {
    pub use super::VisualShaderNodeTextureParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTexture3D {
    pub use super::VisualShaderNodeSample3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTexture3DParameter {
    pub use super::VisualShaderNodeTextureParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTextureParameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTextureParameterTriplanar {
    pub use super::VisualShaderNodeTextureParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTextureSdf {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTextureSdfNormal {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTransformCompose {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTransformConstant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTransformDecompose {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTransformFunc {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTransformOp {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTransformParameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeTransformVecMult {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeUIntConstant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeUIntFunc {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeUIntOp {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeUIntParameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeUvFunc {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeUvPolarCoord {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVarying {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVaryingGetter {
    pub use super::VisualShaderNodeVarying::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVaryingSetter {
    pub use super::VisualShaderNodeVarying::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVec2Constant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVec2Parameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVec3Constant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVec3Parameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVec4Constant {
    pub use super::VisualShaderNodeConstant::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVec4Parameter {
    pub use super::VisualShaderNodeParameter::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVectorBase {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVectorCompose {
    pub use super::VisualShaderNodeVectorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVectorDecompose {
    pub use super::VisualShaderNodeVectorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVectorDistance {
    pub use super::VisualShaderNodeVectorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVectorFunc {
    pub use super::VisualShaderNodeVectorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVectorLen {
    pub use super::VisualShaderNodeVectorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVectorOp {
    pub use super::VisualShaderNodeVectorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeVectorRefract {
    pub use super::VisualShaderNodeVectorBase::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VisualShaderNodeWorldPositionFromDepth {
    pub use super::VisualShaderNode::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VoxelGi {
    pub use super::VisualInstance3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod VoxelGiData {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WeakRef {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WebRtcDataChannel {
    pub use super::PacketPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WebRtcDataChannelExtension {
    pub use super::WebRtcDataChannel::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_packet_rawptr: (&str, u32) = ("_get_packet", 3099858825u32);
    pub type Sig_get_packet_rawptr = (* mut * const u8, * mut i32,);
    pub const put_packet_rawptr: (&str, u32) = ("_put_packet", 3099858825u32);
    pub type Sig_put_packet_rawptr = (* const u8, i32,);
    pub const get_available_packet_count: (&str, u32) = ("_get_available_packet_count", 3905245786u32);
    pub type Sig_get_available_packet_count = ();
    pub const get_max_packet_size: (&str, u32) = ("_get_max_packet_size", 3905245786u32);
    pub type Sig_get_max_packet_size = ();
    pub const poll: (&str, u32) = ("_poll", 166280745u32);
    pub type Sig_poll = ();
    pub const close: (&str, u32) = ("_close", 3218959716u32);
    pub type Sig_close = ();
    pub const set_write_mode: (&str, u32) = ("_set_write_mode", 1999768052u32);
    pub type Sig_set_write_mode = (crate::classes::web_rtc_data_channel::WriteMode,);
    pub const get_write_mode: (&str, u32) = ("_get_write_mode", 2848495172u32);
    pub type Sig_get_write_mode = ();
    pub const was_string_packet: (&str, u32) = ("_was_string_packet", 36873697u32);
    pub type Sig_was_string_packet = ();
    pub const get_ready_state: (&str, u32) = ("_get_ready_state", 3501143017u32);
    pub type Sig_get_ready_state = ();
    pub const get_label: (&str, u32) = ("_get_label", 201670096u32);
    pub type Sig_get_label = ();
    pub const is_ordered: (&str, u32) = ("_is_ordered", 36873697u32);
    pub type Sig_is_ordered = ();
    pub const get_id: (&str, u32) = ("_get_id", 3905245786u32);
    pub type Sig_get_id = ();
    pub const get_max_packet_life_time: (&str, u32) = ("_get_max_packet_life_time", 3905245786u32);
    pub type Sig_get_max_packet_life_time = ();
    pub const get_max_retransmits: (&str, u32) = ("_get_max_retransmits", 3905245786u32);
    pub type Sig_get_max_retransmits = ();
    pub const get_protocol: (&str, u32) = ("_get_protocol", 201670096u32);
    pub type Sig_get_protocol = ();
    pub const is_negotiated: (&str, u32) = ("_is_negotiated", 36873697u32);
    pub type Sig_is_negotiated = ();
    pub const get_buffered_amount: (&str, u32) = ("_get_buffered_amount", 3905245786u32);
    pub type Sig_get_buffered_amount = ();
    
}
pub mod WebRtcMultiplayerPeer {
    pub use super::MultiplayerPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WebRtcPeerConnection {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WebRtcPeerConnectionExtension {
    pub use super::WebRtcPeerConnection::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_connection_state: (&str, u32) = ("_get_connection_state", 2275710506u32);
    pub type Sig_get_connection_state = ();
    pub const get_gathering_state: (&str, u32) = ("_get_gathering_state", 4262591401u32);
    pub type Sig_get_gathering_state = ();
    pub const get_signaling_state: (&str, u32) = ("_get_signaling_state", 3342956226u32);
    pub type Sig_get_signaling_state = ();
    pub const initialize: (&str, u32) = ("_initialize", 1494659981u32);
    pub type Sig_initialize = (VarDictionary,);
    pub const create_data_channel: (&str, u32) = ("_create_data_channel", 4111292546u32);
    pub type Sig_create_data_channel = (GString, VarDictionary,);
    pub const create_offer: (&str, u32) = ("_create_offer", 166280745u32);
    pub type Sig_create_offer = ();
    pub const set_remote_description: (&str, u32) = ("_set_remote_description", 852856452u32);
    pub type Sig_set_remote_description = (GString, GString,);
    pub const set_local_description: (&str, u32) = ("_set_local_description", 852856452u32);
    pub type Sig_set_local_description = (GString, GString,);
    pub const add_ice_candidate: (&str, u32) = ("_add_ice_candidate", 3958950400u32);
    pub type Sig_add_ice_candidate = (GString, i32, GString,);
    pub const poll: (&str, u32) = ("_poll", 166280745u32);
    pub type Sig_poll = ();
    pub const close: (&str, u32) = ("_close", 3218959716u32);
    pub type Sig_close = ();
    
}
pub mod WebSocketMultiplayerPeer {
    pub use super::MultiplayerPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WebSocketPeer {
    pub use super::PacketPeer::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WebXrInterface {
    pub use super::XrInterface::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Window {
    pub use super::Viewport::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_contents_minimum_size: (&str, u32) = ("_get_contents_minimum_size", 3341600327u32);
    pub type Sig_get_contents_minimum_size = ();
    
}
pub mod WorkerThreadPool {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod World2D {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod World3D {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WorldBoundaryShape2D {
    pub use super::Shape2D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WorldBoundaryShape3D {
    pub use super::Shape3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod WorldEnvironment {
    pub use super::Node::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod X509Certificate {
    pub use super::Resource::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XmlParser {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrAnchor3D {
    pub use super::XrNode3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrCamera3D {
    pub use super::Camera3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrController3D {
    pub use super::XrNode3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrControllerTracker {
    pub use super::XrPositionalTracker::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrHandModifier3D {
    pub use super::SkeletonModifier3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrHandTracker {
    pub use super::XrPositionalTracker::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrInterface {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrInterfaceExtension {
    pub use super::XrInterface::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    pub const get_name: (&str, u32) = ("_get_name", 2002593661u32);
    pub type Sig_get_name = ();
    pub const get_capabilities: (&str, u32) = ("_get_capabilities", 3905245786u32);
    pub type Sig_get_capabilities = ();
    pub const is_initialized: (&str, u32) = ("_is_initialized", 36873697u32);
    pub type Sig_is_initialized = ();
    pub const initialize: (&str, u32) = ("_initialize", 2240911060u32);
    pub type Sig_initialize = ();
    pub const uninitialize: (&str, u32) = ("_uninitialize", 3218959716u32);
    pub type Sig_uninitialize = ();
    pub const get_system_info: (&str, u32) = ("_get_system_info", 3102165223u32);
    pub type Sig_get_system_info = ();
    pub const supports_play_area_mode: (&str, u32) = ("_supports_play_area_mode", 2693703033u32);
    pub type Sig_supports_play_area_mode = (crate::classes::xr_interface::PlayAreaMode,);
    pub const get_play_area_mode: (&str, u32) = ("_get_play_area_mode", 1615132885u32);
    pub type Sig_get_play_area_mode = ();
    pub const set_play_area_mode: (&str, u32) = ("_set_play_area_mode", 2693703033u32);
    pub type Sig_set_play_area_mode = (crate::classes::xr_interface::PlayAreaMode,);
    pub const get_play_area: (&str, u32) = ("_get_play_area", 497664490u32);
    pub type Sig_get_play_area = ();
    pub const get_render_target_size: (&str, u32) = ("_get_render_target_size", 1497962370u32);
    pub type Sig_get_render_target_size = ();
    pub const get_view_count: (&str, u32) = ("_get_view_count", 2455072627u32);
    pub type Sig_get_view_count = ();
    pub const get_camera_transform: (&str, u32) = ("_get_camera_transform", 4183770049u32);
    pub type Sig_get_camera_transform = ();
    pub const get_transform_for_view: (&str, u32) = ("_get_transform_for_view", 518934792u32);
    pub type Sig_get_transform_for_view = (u32, Transform3D,);
    pub const get_projection_for_view: (&str, u32) = ("_get_projection_for_view", 4067457445u32);
    pub type Sig_get_projection_for_view = (u32, f64, f64, f64,);
    pub const get_vrs_texture: (&str, u32) = ("_get_vrs_texture", 529393457u32);
    pub type Sig_get_vrs_texture = ();
    pub const get_vrs_texture_format: (&str, u32) = ("_get_vrs_texture_format", 1500923256u32);
    pub type Sig_get_vrs_texture_format = ();
    pub const process: (&str, u32) = ("_process", 3218959716u32);
    pub type Sig_process = ();
    pub const pre_render: (&str, u32) = ("_pre_render", 3218959716u32);
    pub type Sig_pre_render = ();
    pub const pre_draw_viewport: (&str, u32) = ("_pre_draw_viewport", 3521089500u32);
    pub type Sig_pre_draw_viewport = (Rid,);
    pub const post_draw_viewport: (&str, u32) = ("_post_draw_viewport", 1378122625u32);
    pub type Sig_post_draw_viewport = (Rid, Rect2,);
    pub const end_frame: (&str, u32) = ("_end_frame", 3218959716u32);
    pub type Sig_end_frame = ();
    pub const get_suggested_tracker_names: (&str, u32) = ("_get_suggested_tracker_names", 1139954409u32);
    pub type Sig_get_suggested_tracker_names = ();
    pub const get_suggested_pose_names: (&str, u32) = ("_get_suggested_pose_names", 1761182771u32);
    pub type Sig_get_suggested_pose_names = (StringName,);
    pub const get_tracking_status: (&str, u32) = ("_get_tracking_status", 167423259u32);
    pub type Sig_get_tracking_status = ();
    pub const trigger_haptic_pulse: (&str, u32) = ("_trigger_haptic_pulse", 3752640163u32);
    pub type Sig_trigger_haptic_pulse = (GString, StringName, f64, f64, f64, f64,);
    pub const get_anchor_detection_is_enabled: (&str, u32) = ("_get_anchor_detection_is_enabled", 36873697u32);
    pub type Sig_get_anchor_detection_is_enabled = ();
    pub const set_anchor_detection_is_enabled: (&str, u32) = ("_set_anchor_detection_is_enabled", 2586408642u32);
    pub type Sig_set_anchor_detection_is_enabled = (bool,);
    pub const get_camera_feed_id: (&str, u32) = ("_get_camera_feed_id", 3905245786u32);
    pub type Sig_get_camera_feed_id = ();
    pub const get_color_texture: (&str, u32) = ("_get_color_texture", 529393457u32);
    pub type Sig_get_color_texture = ();
    pub const get_depth_texture: (&str, u32) = ("_get_depth_texture", 529393457u32);
    pub type Sig_get_depth_texture = ();
    pub const get_velocity_texture: (&str, u32) = ("_get_velocity_texture", 529393457u32);
    pub type Sig_get_velocity_texture = ();
    
}
pub mod XrNode3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrOrigin3D {
    pub use super::Node3D::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrPose {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrPositionalTracker {
    pub use super::XrTracker::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrServer {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod XrTracker {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod Xrvrs {
    pub use super::Object::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ZipPacker {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}
pub mod ZipReader {
    pub use super::RefCounted::*;
    pub use crate::builtin::*;
    pub use crate::classes::native::*;
    pub use crate::obj::Gd;
    pub use std::ffi::c_void;
    
}