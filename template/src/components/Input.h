#ifndef _INPUT_H
#define _INPUT_H
#include "../constants.h"
#include <flecs.h>

typedef struct Input {
  u32 a : 1, s : 1, d : 1, w : 1, space : 1, left_ctrl : 1, enter : 1, alt : 1,
      q : 1, e : 1, up : 1, down : 1, left : 1, right : 1;
} Input;

extern ECS_COMPONENT_DECLARE(Input);
#endif
