#ifndef __PLAY_STATE_H__
#define __PLAY_STATE_H__

#include "../Engine/game_state.h"
#include "../Engine/game_object.h"
#include "../Engine/sdl_game_object.h"

#include <string>
#include <vector>
class PlayState : public GameState
{
public:
    virtual void update();
    virtual void render();
    virtual bool onEnter();
    virtual bool onExit();
    virtual std::string getStateID() const { return s_playID; }

    bool flyOut();
    void checkKeyDown();
private:
    static const std::string s_playID;
    std::vector<GameObject*> m_gameObjects;

    int m_lastMove = 0;
    int m_clickRepeat = 0;


};
#endif
