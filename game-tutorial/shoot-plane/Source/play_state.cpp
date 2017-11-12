#include "play_state.h"
#include "../Engine/texture_manager.h"
#include "game.h"
#include "../Engine/input_handler.h"
#include "pause_state.h"
#include "game_over_state.h"
#include "../Engine/state_parser.h"
#include "../Engine/game_object_factory.h"
#include "level_manager.h"

#include <iostream>
#include <cstdlib>

const std::string PlayState::s_playID = "PLAY";
const int CLICK_DELAY = 200;

void PlayState::update()
{
    if(InputHandler::instance()->isKeyDown(SDL_SCANCODE_ESCAPE))
    {
        Game::instance()->getStateMachine()->pushState(new PauseState());
    }

    // 删掉飞出屏幕的飞机
    if (flyOut()) {
        // 成对删除
        m_gameObjects.erase(m_gameObjects.begin(), m_gameObjects.begin()+2);
        // LevelManager::instance()->resetLevel();
        Game::instance()->getStateMachine()->pushState(new GameOverState());
    }

    if (m_clickRepeat <= SDL_GetTicks()) {
        checkKeyDown();
    }


    // 更新之前的对象
    for (auto i : m_gameObjects) {
		i->update();
        dynamic_cast<SDLGameObject*>(i)->setVelocity(LevelManager::instance()->getVelocity());
	}



    int currentTime = SDL_GetTicks();
    if (currentTime > m_lastMove + LevelManager::instance()->getLevel()) {

        int x = rand() % (Game::s_screenWidth - 150);

        // 一个以上要防重叠
        if (m_gameObjects.size() >= 2) {
            auto e = m_gameObjects.end();
            int lastX = dynamic_cast<SDLGameObject *>(*(e-1))->getPosition().getX();
            if (x > lastX + 150 || x + 150 < lastX) {
                // 飞机
                GameObject* pGameObject = GameObjectFactory::instance()->create("Plane");
                pGameObject->load(new LoaderParams(x, Game::s_screenHeight, 150, 134, "plane", 4, 0, 1));
                m_gameObjects.push_back(pGameObject);

                int row = rand() % 26 + 1;
                // 字母
                pGameObject = GameObjectFactory::instance()->create("Plane");
                pGameObject->load(new LoaderParams(x+25, Game::s_screenHeight+20, 100, 75, "abc", 1, 0, 1));
                dynamic_cast<SDLGameObject*>(pGameObject)->setCurrentRow(row);
                m_gameObjects.push_back(pGameObject);

            } else {
                x = rand() % (Game::s_screenWidth - 150);
            }
        } else {
            // 飞机
            GameObject* pGameObject = GameObjectFactory::instance()->create("Plane");
            pGameObject->load(new LoaderParams(x, Game::s_screenHeight, 150, 134, "plane", 4, 0, 1));
            m_gameObjects.push_back(pGameObject);

            int row = rand() % 26 + 1;
            // 字母
            pGameObject = GameObjectFactory::instance()->create("Plane");
            pGameObject->load(new LoaderParams(x+25, Game::s_screenHeight+20, 100, 75, "abc", 1, 0, 1));
            dynamic_cast<SDLGameObject*>(pGameObject)->setCurrentRow(row);
            m_gameObjects.push_back(pGameObject);


        }
        m_lastMove = currentTime;
    }

}

void PlayState::render()
{
    for (auto i : m_gameObjects) {
		i->draw();
	}
}

bool PlayState::onEnter()
{

    StateParser stateParser;
    stateParser.parseState("Source/settings.json", s_playID, &m_gameObjects,
        &m_textureIDList);
    LevelManager::instance()->resetLevel();

    std::cout << "entering PlayState\n";
    return true;
}

bool PlayState::onExit()
{
    for (auto i : m_gameObjects) {
        i->clean();
    }
    m_gameObjects.clear();
    TextureManager::instance()->clearFromTextureMap("plane");
    std::cout << "exiting PlayState\n";
    return true;
}

bool PlayState::flyOut()
{
    if (m_gameObjects.empty()) {
        return false;
    }

    if (dynamic_cast<SDLGameObject*>(m_gameObjects.front())->getPosition().getY() + 100 < 0) {
        return true;
    }
    return false;
}

void PlayState::checkKeyDown()
{

    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_A)) {

        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 1) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_B)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 2) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_C)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 3) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_D)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 4) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_E)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 5) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_F)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 6) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_G)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 7) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_H)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 8) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_I)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 9) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_J)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 10) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_K)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 11) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_L)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 12) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_M)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 13) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_N)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 14) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_O)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 15) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_P)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 16) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_Q)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 17) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);

                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_R)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 18) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_S)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 19) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_T)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 20) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_U)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 21) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_V)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 22) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_W)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 23) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_X)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 24) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_Y)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 25) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
    if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_Z)) {
        m_clickRepeat = SDL_GetTicks() + CLICK_DELAY;
        for (auto b = m_gameObjects.begin() + 1, e = m_gameObjects.end(); b < e; b+=2) {
            if (dynamic_cast<SDLGameObject*>(*b)->getCurrentRow() == 26) {
                m_gameObjects.erase(b);
                m_gameObjects.erase(b-1);
                LevelManager::instance()->levelUp();
                return;
            }
        }
    }
}
