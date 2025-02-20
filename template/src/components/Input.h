#ifndef _INPUT_H
#define _INPUT_H
#include "../constants.h"
#include <flecs.h>

typedef struct Input {
  u8 a : 1;
  u8 s : 1;
  u8 d : 1;
  u8 w : 1;
  u8 space : 1;
  u8 left_ctrl : 1;
  u8 enter : 1;
  u8 alt : 1;
  u8 q : 1;
  u8 e : 1;
  u8 up : 1;
  u8 down : 1;
  u8 left : 1;
  u8 right : 1;
  u8 down_1 : 1;
  u8 down_2 : 1;
  u8 down_3 : 1;
} Input;

extern ECS_COMPONENT_DECLARE(Input);
#endif
