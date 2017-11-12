#ifndef __GAME_H__
#define __GAME_H__

#include <SDL2/SDL.h>
#include <vector>

#include "../Engine/texture_manager.h"
#include "../Engine/game_state_machine.h"
#include "../Engine/game_object.h"

class Game
{
public:
    Game() {}
    ~Game() {}

    bool init(const char* title, int xpos,int ypos, int width, int height, int flags);
    void render();
    void update();
    void handleEvents();
    void clean();
    void draw();
    void quit();

    bool running() { return m_bRunning; }
    SDL_Renderer* getRenderer() const { return m_pRenderer; }
    GameStateMachine* getStateMachine(){ return m_pGameStateMachine; }


    static int s_screenWidth;
    static int s_screenHeight;
    static Game* instance();

private:
    bool m_bRunning = true;

    SDL_Window* m_pWindow;
    SDL_Renderer* m_pRenderer;

    std::vector<GameObject*> m_gameObjects;

    static Game* s_pInstance;
    GameStateMachine* m_pGameStateMachine;
};

#endif
