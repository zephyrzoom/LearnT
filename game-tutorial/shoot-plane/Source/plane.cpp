#include "plane.h"
#include "game.h"
#include "../Engine/input_handler.h"
#include "level_manager.h"

#include <iostream>

Plane::Plane() : SDLGameObject() {}

void Plane::load(const LoaderParams* pParams)
{
    SDLGameObject::load(pParams);
    setVelocity(LevelManager::instance()->getVelocity());
}

void Plane::draw()
{
    SDLGameObject::draw();
}

void Plane::update()
{
    m_currentFrame = int(((SDL_GetTicks() / 100) % m_numFrames));
    SDLGameObject::update();
}

void Plane::clean() {}
