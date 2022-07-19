use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{components::{Velocity, Laser, WinSize, DespawnedList, SpriteSize, BlockSize, Block, ExplosionToSpawn, BlockToDecimate}, BlockHeat, Unsupported};

pub fn laser_move_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Laser>>,
    win_size: Res<WinSize>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * 5.0;

        if translation.x >= win_size.w / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}



pub fn laser_hit_system(
    mut commands: Commands,
    mut despawned_list: ResMut<DespawnedList>,
    mut laser_query: Query<(Entity, &Transform, &SpriteSize), With<Laser>>,
    mut block_query: Query<(Entity, &Transform, &SpriteSize, &BlockSize, &mut BlockHeat), With<Block>>,
) {
    let despawned = &mut despawned_list.0;
    for (laser_entity, laser_transform, laser_sprite_size) in laser_query.iter_mut() {
        if despawned.contains(&laser_entity) {
            continue;
        }
            for (block_entity, block_transform, block_sprite_size, block_size,mut block_heat) in block_query.iter_mut() {
                if despawned.contains(&block_entity) || despawned.contains(&laser_entity) {
                    continue;
                }

                let collision = collide(
                    laser_transform.translation,
                    laser_sprite_size.0,
                    block_transform.translation,
                    block_sprite_size.0,
                );
                
                const LASER_HEAT:u32 = 1000;
                const BLOCK_MAX_HEAT: u32 = 3000;
                // perform collision
                if let Some(_) = collision {
                    block_heat.0 += LASER_HEAT;
                    if block_heat.0 >= BLOCK_MAX_HEAT {
                        // remove the block
                        despawned.insert(block_entity);
                        commands.entity(block_entity).despawn();

                        // spawn the explosionToSpawn
                        let mut location = block_transform.translation.clone();
                        // move up the Z
                        location[2] = 500.0;

                        match block_size {
                            BlockSize::Large(_) => {commands
                                .spawn()
                                .insert(ExplosionToSpawn(location))
                                .insert(BlockToDecimate(block_transform.translation.clone()))
                                .insert(block_size.clone());}
                            BlockSize::Medium(_) => {commands
                                .spawn()
                                .insert(ExplosionToSpawn(location))
                                .insert(Unsupported(location));}
                            _ => panic!("Unsupported Block size")
                        }
                    }

                    // spawn the explosionToSpawn
                    let mut explosion_location = block_transform.translation.clone();
                    // move up the Z
                    explosion_location[2] = 500.0;

                    commands
                        .spawn()
                        .insert(ExplosionToSpawn(explosion_location))
                        .insert(block_size.clone());

                    // remove the laser
                    despawned.insert(laser_entity);
                    commands.entity(laser_entity).despawn();
                }
            }
    }
}
