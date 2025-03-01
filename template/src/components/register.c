#include "components.h"

ECS_COMPONENT_DECLARE(Position);
ECS_COMPONENT_DECLARE(Input);
ECS_COMPONENT_DECLARE(Cam);
ECS_COMPONENT_DECLARE(SceneManager);

ECS_COMPONENT_DECLARE(Uli);
void define_components(ecs_world_t *world) {
  ECS_COMPONENT_DEFINE(world, Position);
  ECS_COMPONENT_DEFINE(world, Input);
  ECS_COMPONENT_DEFINE(world, Cam);
  ECS_COMPONENT_DEFINE(world, SceneManager);

  ecs_singleton_add(world, Input);
  ecs_singleton_add(world, Cam);
  ecs_singleton_add(world, SceneManager);
}
