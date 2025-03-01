#include "components/components.h"
#include "constants.h"
#include "systems/systems.h"

int main(int argc, char *argv[]) {
  /* ══════════════════════════ Setup ═══════════════════════════════════════ */
  InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE);
  SetTargetFPS(WINDOW_FPS);
  ecs_world_t *world = ecs_init();

  /* ══════════════════════════ Define ══════════════════════════════════════ */
  define_components(world);
  define_systems(world);

  ecs_entity_t e = ecs_new(world);
  ecs_set(world, e, Position, {.x = 0, .y = 0});

  /* ══════════════════════════ Main Loop
     ═══════════════════════════════════ */
  float delta = 0.0f;
  while (!WindowShouldClose()) {
    delta = GetFrameTime();
    BeginDrawing();
    ClearBackground(WINDOW_BACKGROUND);
    ecs_progress(world, delta);
    EndDrawing();
  }

  /* ══════════════════════════ CleanUp ═════════════════════════════════════ */
  ecs_fini(world);
  CloseWindow();
  return 0;
}
