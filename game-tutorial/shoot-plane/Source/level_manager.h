#ifndef __LEVEL_MANAGER_H__
#define __LEVEL_MANAGER_H__

#include "../Engine/vector2d.h"

class LevelManager
{
    public:
        static LevelManager* instance();
        bool levelUp();
        void levelDown();
        int getLevel() { return m_delay; }
        void resetLevel();
        Vector2D getVelocity() { return m_velocity; }

    private:
        int m_perfects = 0;
        int m_delay = 1000;
        Vector2D m_velocity = Vector2D(0, -1);
        int k = 300;

        static LevelManager* s_pInstance;
        LevelManager() {};

};

#endif
