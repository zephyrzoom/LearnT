#OBJS specifies which files to compile as part of the project
OBJS = 06_extension_libraries_and_loading_other_image_formats.cpp

#CC specifies which compiler we're using
CC = g++

#COMPILER_FLAGS specifies the additional compilation options we're using
# -w suppresses all warnings
COMPILER_FLAGS = -w

#LINKER_FLAGS specifies the libraries we're linking against
LINKER_FLAGS = -lSDL2 -lSDL2_image

#OBJ_NAME specifies the name of our exectuable
OBJ_NAME = 06_extension_libraries_and_loading_other_image_formats

#This is the target that compiles our executable
all : $(OBJS)
	$(CC) $(OBJS) $(COMPILER_FLAGS) $(LINKER_FLAGS) -o $(OBJ_NAME) -std=c++11
	./$(OBJ_NAME)

#clean executable
clean:
	rm $(OBJ_NAME)
