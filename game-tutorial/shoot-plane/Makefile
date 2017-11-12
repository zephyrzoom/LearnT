#OBJS specifies which files to compile as part of the project
OBJS = Temp/main.o                  \
	   Temp/game.o 					\
	   Temp/texture_manager.o 		\
	   Temp/main_menu_state.o 		\
	   Temp/sdl_game_object.o 		\
	   Temp/input_handler.o 		\
	   Temp/state_parser.o 			\
	   Temp/game_state_machine.o 	\
	   Temp/play_state.o 			\
	   Temp/menu_button.o			\
	   Temp/pause_state.o 			\
	   Temp/plane.o 				\
	   Temp/animated_graphic.o 		\
	   Temp/game_over_state.o 		\
	   Temp/game_object_factory.o	\
	   Temp/level_manager.o

#CC specifies which compiler we're using
CC = clang++

#COMPILER_FLAGS specifies the additional compilation options we're using
# -w suppresses all warnings
COMPILER_FLAGS = -w -fno-limit-debug-info

#LINKER_FLAGS specifies the libraries we're linking against
LINKER_FLAGS = -lSDL2 -lSDL2_image

#OBJ_NAME specifies the name of our exectuable
OBJ_NAME = Game/shoot-plane Temp/*.o

TARGET_NAME = Game/shoot-plane

DEBUG_FLAGS = -g

#This is the target that compiles our executable
all : $(OBJS)
	$(CC) $(OBJS) $(COMPILER_FLAGS) $(LINKER_FLAGS) -o $(TARGET_NAME) $(DEBUG_FLAGS) -std=c++11

Temp/main.o : Source/main.cpp Source/game.h
	$(CC) -c Source/main.cpp -o Temp/main.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/game.o : Source/game.cpp Source/game.h
	$(CC) -c Source/game.cpp -o Temp/game.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/texture_manager.o : Engine/texture_manager.cpp Engine/texture_manager.h
	$(CC) -c Engine/texture_manager.cpp -o Temp/texture_manager.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/sdl_game_object.o : Engine/sdl_game_object.cpp Engine/sdl_game_object.h
	$(CC) -c Engine/sdl_game_object.cpp -o Temp/sdl_game_object.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/input_handler.o : Engine/input_handler.cpp Engine/input_handler.h
	$(CC) -c Engine/input_handler.cpp -o Temp/input_handler.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/game_state_machine.o : Engine/game_state_machine.cpp Engine/game_state_machine.h
	$(CC) -c Engine/game_state_machine.cpp -o Temp/game_state_machine.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/play_state.o : Source/play_state.cpp Source/play_state.h
	$(CC) -c Source/play_state.cpp -o Temp/play_state.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/menu_button.o : Source/menu_button.cpp Source/menu_button.h
	$(CC) -c Source/menu_button.cpp -o Temp/menu_button.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/pause_state.o : Source/pause_state.cpp Source/pause_state.h
	$(CC) -c Source/pause_state.cpp -o Temp/pause_state.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/plane.o : Source/plane.cpp Source/plane.h
	$(CC) -c Source/plane.cpp -o Temp/plane.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/animated_graphic.o : Source/animated_graphic.cpp Source/animated_graphic.h
	$(CC) -c Source/animated_graphic.cpp -o Temp/animated_graphic.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/game_over_state.o : Source/game_over_state.cpp Source/game_over_state.h
	$(CC) -c Source/game_over_state.cpp -o Temp/game_over_state.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/main_menu_state.o : Source/main_menu_state.cpp Source/main_menu_state.h
	$(CC) -c Source/main_menu_state.cpp -o Temp/main_menu_state.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/state_parser.o : Engine/state_parser.cpp Engine/state_parser.h
	$(CC) -c Engine/state_parser.cpp -o Temp/state_parser.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/game_object_factory.o : Engine/game_object_factory.cpp Engine/game_object_factory.h
	$(CC) -c Engine/game_object_factory.cpp -o Temp/game_object_factory.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

Temp/level_manager.o : Source/level_manager.cpp Source/level_manager.h
	$(CC) -c Source/level_manager.cpp -o Temp/level_manager.o -std=c++11 $(DEBUG_FLAGS) $(COMPILER_FLAGS)

run :
	./$(TARGET_NAME)

#clean executable
clean:
	rm $(OBJ_NAME)

release : $(OBJS)
	rm -r Game/Assets;
	rm Game/Source/settings.json;
	$(CC) $(OBJS) $(COMPILER_FLAGS) $(LINKER_FLAGS) -o $(TARGET_NAME) -std=c++11;
	cp -r Assets Game/Assets;
	cp Source/settings.json Game/Source/settings.json;

init :
	mkdir Game Temp Game/Assets;
	sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev libsdl2-mixer-dev;
