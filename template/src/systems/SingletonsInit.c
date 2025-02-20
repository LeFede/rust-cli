#include "systems.h"

void SingletonsInit(ecs_iter_t *it) {
  ecs_singleton_set(it->world, SceneManager, {.current_scene = 0});
  ecs_singleton_set(
      it->world, Cam,
      {.main.rotation = 0.0f,
       .main.zoom = WINDOW_ZOOM,
       .main.offset = (Vector2){WINDOW_WIDTH / 2.0f, WINDOW_HEIGHT / 2.0f},
       .main.target = (Vector2){0.0f, 0.0f},
       .zoom_speed = 4.0f,
       .rotate_speed = 40.0f});
}
