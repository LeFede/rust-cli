#ifndef _SYSTEMS_H
#define _SYSTEMS_H

#include "../components/components.h"
#include <flecs.h>
#include <raylib.h>

void CameraControl(ecs_iter_t *it);
void SingletonsInit(ecs_iter_t *it);
void DrawUI(ecs_iter_t *it);
void HandleInput(ecs_iter_t *it);
void DrawDebug(ecs_iter_t *it);

extern void define_systems(ecs_world_t *world);
#endif
