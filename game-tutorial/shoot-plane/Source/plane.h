#ifndef __PLANE_H__
#define __PLANE_H__

#include "../Engine/sdl_game_object.h"
#include "../Engine/game_object_factory.h"

class Plane : public SDLGameObject
{
public:

    void load(const LoaderParams* pParams);
    void draw();
    void update();
    void clean();

    Plane();
};

class PlaneCreator : public BaseCreator {
    GameObject* createGameObject() const {
        return new Plane();
    }
};

#endif
