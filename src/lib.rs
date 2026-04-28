pub use arcadia_engine_derive::ComponentTrait;

pub mod core;
pub mod util;


pub trait ComponentTrait {
    fn update(&mut self, delta_time: f32);
    fn get_component_name(&self) -> &'static str;
    fn process_input(&mut self, input: u8);
    fn on_update_world_tranform(&mut self);


}