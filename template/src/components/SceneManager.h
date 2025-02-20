#ifndef _SCENEMANAGER_H
#define _SCENEMANAGER_H
#include <flecs.h>

typedef struct SceneManager {
  int current_scene;
} SceneManager;

extern ECS_COMPONENT_DECLARE(SceneManager);
#endif
