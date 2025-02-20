#include "systems.h"

void HandleInput(ecs_iter_t *it) {
  Input *input = ecs_field(it, Input, 0);

  input->a = IsKeyDown(KEY_A);
  input->s = IsKeyDown(KEY_S);
  input->d = IsKeyDown(KEY_D);
  input->w = IsKeyDown(KEY_W);
  input->space = IsKeyDown(KEY_SPACE);
  input->left_ctrl = IsKeyDown(KEY_LEFT_CONTROL);
  input->enter = IsKeyDown(KEY_ENTER);
  input->alt = IsKeyDown(KEY_LEFT_ALT);
  input->q = IsKeyDown(KEY_Q);
  input->e = IsKeyDown(KEY_E);
  input->up = IsKeyDown(KEY_UP);
  input->down = IsKeyDown(KEY_DOWN);
  input->right = IsKeyDown(KEY_RIGHT);
  input->left = IsKeyDown(KEY_LEFT);

  input->down_1 = IsKeyPressed(KEY_ONE);
  input->down_2 = IsKeyPressed(KEY_TWO);
  input->down_3 = IsKeyPressed(KEY_THREE);
}
