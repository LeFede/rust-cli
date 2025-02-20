#include "systems.h"

void CameraControl(ecs_iter_t *it) {
  Cam *c = ecs_field(it, Cam, 0);
  const Input *input = ecs_singleton_get(it->world, Input);

  c->main.zoom += c->zoom_speed * GetFrameTime() * (input->w - input->s);
  c->main.rotation += c->rotate_speed * GetFrameTime() * (input->a - input->d);

  if (input->down_1)
    ecs_singleton_set(it->world, SceneManager, {.current_scene = 1});

  if (input->down_2)
    ecs_singleton_set(it->world, SceneManager, {.current_scene = 2});
}
