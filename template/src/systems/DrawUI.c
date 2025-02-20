#include "systems.h"
#include <stdio.h>

void DrawUI(ecs_iter_t *it) {
  const Input *input = ecs_singleton_get(it->world, Input);
  const Cam *c = ecs_singleton_get(it->world, Cam);
  const int fps_number = GetFPS();
  const SceneManager *scene_manager =
      ecs_singleton_get(it->world, SceneManager);

  DrawRectangle(0, 0, 250, 600, (Color){20, 20, 20, 50});
  DrawText(TextFormat("current_scene: %d\n", scene_manager->current_scene), 20,
           600, 25, (Color){255, 255, 255, 255});
  DrawText(TextFormat("Delta: %f\n", it->delta_time), 20, 20, 25,
           (Color){255, 255, 255, 255});
  DrawText(TextFormat("Fps: %d\n", fps_number), 20, 45, 25,
           (Color){255, 255, 255, 255});
  DrawText(TextFormat("Zoom: %.2f\n", c->main.zoom), 20, 80, 25,
           (Color){255, 255, 255, 255});
  DrawText(TextFormat("Rotation: %.2f\n", c->main.rotation), 20, 105, 25,
           (Color){255, 255, 255, 255});
  DrawText(
      TextFormat("Q:%d\nE:%d\nW:%d\nA:%d\nS:%d\nD:%d\nCTRL:%d\nSPACE:%"
                 "d\nENTER:%d\nALT:%d\nUP:%d\nLEFT:%d\nDOWN:%d\nRIGHT:%d\n",
                 input->q, input->e, input->w, input->a, input->s, input->d,
                 input->left_ctrl, input->space, input->enter, input->alt,
                 input->up, input->left, input->down, input->right),
      20, 160, 25, (Color){255, 255, 255, 255});
  DrawLine(0, WINDOW_HEIGHT / 2, WINDOW_WIDTH, WINDOW_HEIGHT / 2,
           (Color){0, 255, 0, 100});
  DrawLine(WINDOW_WIDTH / 2, 0, WINDOW_WIDTH / 2, WINDOW_HEIGHT,
           (Color){0, 255, 0, 100});
}
