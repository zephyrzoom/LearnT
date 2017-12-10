LINK_DIRECTORIES(/home/zeek/Tmp/lib)
ADD_EXECUTABLE(main main.c)
INCLUDE_DIRECTORIES(/home/zeek/Tmp/include/hello/)
TARGET_LINK_LIBRARIES(main hello)
