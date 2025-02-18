#ifndef _SYSTEMS_H
#define _SYSTEMS_H

#include "../components/components.h"
#include <flecs.h>
#include <raylib.h>

void Draw(ecs_iter_t *it);
void HandleInput(ecs_iter_t *it);

extern void define_systems(ecs_world_t *world);
#endif
