use legion::*;
use legion::world::SubWorld;
use nox_components::*;
use super::messaging;

#[system]
#[read_component(MyTurn)]
#[read_component(Position)]
#[read_component(IdentityTag)]
#[read_component(Settler)]
pub fn hauling(ecs: &SubWorld) {
}