#include "systems.h"

void DrawDebug(ecs_iter_t *it) {
  Position *p = ecs_field(it, Position, 0);
  const Cam *c = ecs_singleton_get(it->world, Cam);

  for (int i = 0; i < it->count; i++) {
    BeginMode2D(c->main);
    DrawRectangle(p[i].x, p[i].y, 1, 1, RED);
    EndMode2D();
  }
}
