#include "systems.h"

void Draw(ecs_iter_t *it) {
  // const Singleton *s = ecs_singleton_get(it->world, Singleton);
  // Component *c = ecs_field(it, Component, 0);
  DrawFPS(20, 20);
}
