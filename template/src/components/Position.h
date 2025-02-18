#ifndef _POSITION_H
#define _POSITION_H
#include <flecs.h>

typedef struct Position {
  int x, y;
} Position;

extern ECS_COMPONENT_DECLARE(Position);
#endif
