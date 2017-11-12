

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

#include <iostream>
#include <vector>

#include "game.h"
#include "../Engine/input_handler.h"
#include "main_menu_state.h"
#include "play_state.h"
#include "menu_button.h"
#include "plane.h"
#include "animated_graphic.h"

int Game::s_screenWidth = 800;
int Game::s_screenHeight = 600;
Game* Game::s_pInstance = nullptr;

bool Game::init(const char* title, int xpos,int ypos, int width, int height, int flags)
{
	if( SDL_Init( SDL_INIT_VIDEO ) == 0 )
	{
		std::cout << "SDL init success\n";
		m_pWindow = SDL_CreateWindow( title, SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, s_screenWidth, s_screenHeight, SDL_WINDOW_SHOWN );
		if (m_pWindow != nullptr) {
			std::cout << "window creation success\n";
			m_pRenderer = SDL_CreateRenderer( m_pWindow, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC );
			if (m_pRenderer != nullptr) {
				std::cout << "renderer creation success\n";

				InputHandler::instance()->initialiseMouse();

				GameObjectFactory::instance()->registerType("MenuButton", new MenuButtonCreator());

				GameObjectFactory::instance()->registerType("Plane", new PlaneCreator());
				// GameObjectFactory::instance()->registerType("Alpha", new AlphaCreator());

				GameObjectFactory::instance()->registerType("AnimatedGraphic", new AnimatedGraphicCreator());

				m_pGameStateMachine = new GameStateMachine();
				m_pGameStateMachine->changeState(new MainMenuState());

			} else {
				std::cout << "renderer init fail\n";
				return false;
			}
		} else {
			std::cout << "window init fail\n";
			return false;
		}
	} else {
		std::cout << "SDL init fail\n";
		return false;
	}
	return true;
}

void Game::render()
{
	SDL_RenderClear(m_pRenderer);
	m_pGameStateMachine->render();
	SDL_RenderPresent(m_pRenderer);
}

void Game::handleEvents()
{
	InputHandler::instance()->update();
	if (InputHandler::instance()->isKeyDown(SDL_SCANCODE_RETURN)) {
		m_pGameStateMachine->changeState(new PlayState());
	}
}

void Game::clean()
{
	std::cout << "cleaning game\n";
	InputHandler::instance()->clean();
	SDL_DestroyWindow(m_pWindow);
	SDL_DestroyRenderer(m_pRenderer);
	SDL_Quit();
}

void Game::update()
{
	m_pGameStateMachine->update();
}

void Game::draw()
{
	for(std::vector<GameObject*>::size_type i = 0; i != m_gameObjects.size(); i++)
	{
		m_gameObjects[i]->draw();
	}
}

Game* Game::instance()
{
   if(s_pInstance == 0)
   {
	   s_pInstance = new Game();
	   return s_pInstance;
   }
   return s_pInstance;
}

void Game::quit()
{
	m_bRunning = false;
}
