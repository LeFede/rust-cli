#include "systems.h"

void define_systems(ecs_world_t *world) {
  ECS_SYSTEM(world, Draw, EcsOnUpdate);
}
