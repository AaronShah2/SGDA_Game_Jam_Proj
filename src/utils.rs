use amethyst::{core::transform::ParentHierarchy, ecs::Entity, prelude::*};

pub fn delete_hierarchy(world: &mut World, root: Entity) {
    let mut to_delete: Vec<Entity> = world.read_resource::<ParentHierarchy>().all_children_iter(root).collect();
    to_delete.push(root);
    world.delete_entities(&to_delete).expect("Failed to remove menu elements");
}
