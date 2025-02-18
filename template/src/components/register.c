#include "components.h"

ECS_COMPONENT_DECLARE(Position);

void define_components(ecs_world_t *world) {
  ECS_COMPONENT_DEFINE(world, Position);
}
