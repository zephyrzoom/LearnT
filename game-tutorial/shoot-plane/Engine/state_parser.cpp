#include "state_parser.h"
#include "../Source/game.h"
#include "texture_manager.h"
#include "game_object_factory.h"

#include <string>
#include <vector>
#include <fstream>

bool StateParser::parseState(const char *stateFile, std::string stateID,
    std::vector<GameObject *> *pObjects, std::vector<std::string> *pTextureIDs)
{

    json root;
    // 载入json
    std::ifstream iStateFile(stateFile);

    if(!iStateFile)
    {
        std::cerr << "file error" << std::endl;
        return false;
    }

    // 获取根节点
    iStateFile >> root;
    root = root["STATES"];
    // 不同状态的根节点
    json stateRoot;
    // 获取某个状态的根节点
    if(root[stateID] != nullptr)
    {
        stateRoot = root[stateID];
    }
    // 纹理根节点
    json textureRoot;
    // 获取纹理的根节点
    if (stateRoot["TEXTURES"] != nullptr) {
        textureRoot = stateRoot["TEXTURES"];
    }

    // 解析纹理
    parseTextures(textureRoot, pTextureIDs);
    // 游戏对象的根节点
    json objectRoot;
    // 获取游戏对象的根节点
    if (stateRoot["OBJECTS"] != nullptr) {
        objectRoot = stateRoot["OBJECTS"];
    }

    // 解析游戏对象
    parseObjects(objectRoot, pObjects);
    return true;
}

void StateParser::parseTextures(json textureRoot,
    std::vector<std::string> *pTextureIDs)
{
    for (auto i : textureRoot) {
        for (auto j : i) {
            std::string filename = j["filename"];
            std::string id = j["ID"];
            pTextureIDs->push_back(id);
            TextureManager::instance()->load(filename, id, Game::instance()->getRenderer());
        }
    }
}

void StateParser::parseObjects(json objectRoot,
    std::vector<GameObject *> *pObjects)
{
    for (auto i: objectRoot) {
        for (auto j : i) {
            int x, y, width, height, numFrames, callbackID, animSpeed;
            std::string textureID;
            x = j["x"];
            y = j["y"];
            width = j["width"];
            height = j["height"];
            numFrames = j["numFrames"];
            callbackID = j["callbackID"];
            animSpeed = j["animSpeed"];
            textureID = j["textureID"];
            GameObject* pGameObject = GameObjectFactory::instance()->create(j["type"]);
            pGameObject->load(new LoaderParams(x, y, width, height, textureID, numFrames, callbackID, animSpeed));
            pObjects->push_back(pGameObject);
        }
    }
}
