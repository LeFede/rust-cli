#include "components.h"

ECS_COMPONENT_DECLARE(Position);
ECS_COMPONENT_DECLARE(Input);

void define_components(ecs_world_t *world) {
  ECS_COMPONENT_DEFINE(world, Position);
  ECS_COMPONENT_DEFINE(world, Input);
}
