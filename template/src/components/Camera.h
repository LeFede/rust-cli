#ifndef _CAMERA_H
#define _CAMERA_H
#include <flecs.h>
#include <raylib.h>

typedef struct Cam {
  Camera2D main;
  float zoom;
  float zoom_speed;
  float rotate_speed;
} Cam;

extern ECS_COMPONENT_DECLARE(Cam);
#endif
