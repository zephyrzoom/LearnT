#include "level_manager.h"

#include <iostream>

LevelManager* LevelManager::s_pInstance = nullptr;

LevelManager* LevelManager::instance()
{
    if (s_pInstance == nullptr) {
        s_pInstance = new LevelManager();
    }
    return s_pInstance;
}

bool LevelManager::levelUp()
{
    m_perfects++;
    // std::cout << m_perfects << std::endl;
    if (m_perfects % 10 == 0) {
        m_velocity = m_velocity + Vector2D(0, -1);
        m_delay -= k;
        k /= 2;
        // std::cout << m_delay << ", " << m_velocity.getY() << std::endl;
        return true;
    }
    return false;
}

void LevelManager::levelDown()
{
    m_perfects = 0;
    m_delay += 10;
    std::cout << m_delay << std::endl;
}

void LevelManager::resetLevel()
{
    m_perfects = 0;
    m_delay = 1000;
    m_velocity = Vector2D(0, -1);
}
