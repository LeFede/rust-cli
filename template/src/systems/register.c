#include "systems.h"

void define_systems(ecs_world_t *world) {
  ECS_SYSTEM(world, SingletonsInit, EcsOnStart);
  ECS_SYSTEM(world, HandleInput, EcsOnLoad, Input);
  ECS_SYSTEM(world, CameraControl, EcsPostLoad, Cam);
  ECS_SYSTEM(world, DrawDebug, EcsOnUpdate, Position);
  ECS_SYSTEM(world, DrawUI, EcsOnStore);
}
