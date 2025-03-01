#ifndef _POSITION_H
#define _POSITION_H
#include "../constants.h"
#include <flecs.h>

typedef struct Position {
  i32 x, y;
} Position;

extern ECS_COMPONENT_DECLARE(Position);
#endif
