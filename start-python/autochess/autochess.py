from itertools import combinations
from multiprocessing import Process

WEIGHT1 = 1
WEIGHT2 = 1
WEIGHT3 = 1
WEIGHT4 = 1
WEIGHT6 = 1
WEIGHT9 = 1

class Hero:
    def __init__(self, name, spec, cls):
        self.name = name
        self.spec = spec
        self.cls = cls

    def __str__(self):
        return self.name
    

def main():
    axe = Hero('axe', [1], 1)
    juggernaut = Hero('juggernaut', [1], 1)
    tusk = Hero('tusk', [2], 1)
    lycan = Hero('lycan', [2, 3], 1)
    kunkka = Hero('kunkka', [3], 1)
    troll_warlod = Hero('troll_warlod', [9], 1)
    tiny = Hero('tiny', [10], 1)
    mars = Hero('mars', [11], 1)
    doom = Hero('doom', [12], 1)
    slardar = Hero('slardar', [13], 1)
    enchantress = Hero('enchantress', [2], 2)
    lone_druid = Hero('lone_druid', [2], 2)
    furion = Hero('furion', [5], 2)
    treant_protector = Hero('treant_protector', [5], 2)
    crystal_maiden = Hero('crystal_maiden', [3], 3)
    lina = Hero('lina', [3], 3)
    keeper_of_the_light = Hero('keeper_of_the_light', [3], 3)
    puck = Hero('puck', [4, 5], 3)
    ogre_magi = Hero('ogre_magi', [6], 3)
    lich = Hero('lich', [7], 3)
    razor = Hero('razor', [10], 3)
    zeus = Hero('zeus', [11], 3)
    beastmaster = Hero('beastmaster', [1], 4)
    mirana = Hero('mirana', [5], 4)
    windranger = Hero('windranger', [5], 4)
    drow_ranger = Hero('drow_ranger', [7], 4)
    medusa = Hero('medusa', [13], 4)
    tidehunter = Hero('tidehunter', [13], 4)
    sniper = Hero('sniper', [14], 4)
    viper = Hero('viper', [4], 5)
    phantom_assassin = Hero('phantom_assassin', [5], 5)
    templar_assassin = Hero('templar_assassin', [5], 5)
    bounty_hunter = Hero('bounty_hunter', [8], 5)
    morphling = Hero('morphling', [10], 5)
    queen_of_pain = Hero('queen_of_pain', [12], 5)
    clockwerk = Hero('clockwerk', [8], 6)
    tinker = Hero('tinker', [8], 6)
    timbersaw = Hero('timbersaw', [8], 6)
    techies = Hero('techies', [8], 6)
    gyrocopter = Hero('gyrocopter', [14], 6)
    disruptor = Hero('disruptor', [1], 7)
    shadow_shaman = Hero('shadow_shaman', [9], 7)
    omniknight = Hero('omniknight', [3], 8)
    dragon_knight = Hero('dragon_knight', [3, 4], 8)
    luna = Hero('luna', [5], 8)
    abaddon = Hero('abaddon', [7], 8)
    batrider = Hero('batrider', [9], 8)
    chaos_knight = Hero('chaos_knight', [12], 8)
    anti_mage = Hero('anti_mage', [5], 9)
    terrorblade = Hero('terrorblade', [12], 9)
    venomancer = Hero('venomancer', [2], 10)
    necrophos = Hero('necrophos', [7], 10)
    death_prophet = Hero('death_prophet', [7], 10)
    alchemist = Hero('alchemist', [8], 10)
    witch_doctor = Hero('witch_doctor', [9], 10)
    enigma = Hero('enigma', [10], 10)
    shadow_fiend = Hero('shadow_fiend', [12], 10)

    all_heroes = [
        axe,
        juggernaut,
        tusk,
        lycan,
        kunkka,
        troll_warlod,
        tiny,
        mars,
        doom,
        slardar,
        enchantress,
        lone_druid,
        furion,
        treant_protector,
        crystal_maiden,
        lina,
        keeper_of_the_light,
        puck,
        ogre_magi,
        lich,
        razor,
        zeus,
        beastmaster,
        mirana,
        windranger,
        drow_ranger,
        medusa,
        tidehunter,
        sniper,
        viper,
        phantom_assassin,
        templar_assassin,
        bounty_hunter,
        morphling,
        queen_of_pain,
        clockwerk,
        tinker,
        timbersaw,
        techies,
        gyrocopter,
        disruptor,
        shadow_shaman,
        omniknight,
        dragon_knight,
        luna,
        abaddon,
        batrider,
        chaos_knight,
        anti_mage,
        terrorblade,
        venomancer,
        necrophos,
        death_prophet,
        alchemist,
        witch_doctor,
        enigma,
        shadow_fiend
    ]

    heroes = combinations(all_heroes, 10)
    # one_four = len(heroes) // 2 // 2
    # two_four = len(heroes) // 2
    # three_four = len(heroes) - one_four

    # jobs = []
    # p = Process(target=seperate_calc, args=(heroes[:one_four],))
    # jobs.append(p)
    # p = Process(target=seperate_calc, args=(heroes[one_four:two_four],))
    # jobs.append(p)
    # p = Process(target=seperate_calc, args=(heroes[two_four:three_four],))
    # jobs.append(p)
    # p = Process(target=seperate_calc, args=(heroes[three_four:],))
    # jobs.append(p)

    # for job in jobs:
    #     job.start()
    
    # for job in jobs:
    #     job.join()

    seperate_calc(heroes)

def seperate_calc(heroes):
    print('*********start*********')
    max = 0
    max_heroes = None
    i = 0

    for picked in heroes:
        result = score(picked)
        if result > max:
            max = result
            max_heroes = picked
        i += 1
        if i % 10240000 == 0:
            print(max)
            print(i)
            for hero in max_heroes:
                print(hero)
            print('------------split-------------')
    
    print('===========final============')
    for hero in max_heroes:
        print(max)
        print(hero)

def score(heroes):
    result = 0
    result += fit1(heroes)
    result += fit2(heroes)
    result += fit3(heroes)
    result += fit4(heroes)
    result += fit6(heroes)
    result += fit9(heroes)
    return result

def fit1(heroes):
    result = 0
    demon = 0
    ogre = 0
    god = 0
    dwarf = 0
    demon_hunter = 0
    for hero in heroes:
        for spec in hero.spec:
            if spec == 6:
                ogre += 1
            elif spec == 11:
                god += 1
            elif spec == 14:
                dwarf += 1
            elif spec == 12:
                demon += 1
        if hero.cls == 9:
            demon_hunter += 1
            
    if demon == 1:
        result += WEIGHT1
    if ogre >= 1:
        result += WEIGHT1
    if god >= 1:
        result += WEIGHT1
    if dwarf >= 1:
        result += WEIGHT1
    if demon_hunter >= 1:
        result += WEIGHT1
    return result

def fit2(heroes):
    result = 0
    orc = 0
    beast = 0
    human = 0
    undead = 0
    troll = 0
    elemental = 0
    god = 0
    naga = 0
    druid = 0
    mech = 0
    shaman = 0
    knight = 0
    demon_hunter = 0
    for hero in heroes:
        for spec in hero.spec:
            if spec == 1:
                orc += 1
            elif spec == 2:
                beast += 1
            elif spec == 3:
                human += 1
            elif spec == 7:
                undead += 1
            elif spec == 9:
                troll += 1
            elif spec == 10:
                elemental += 1
            elif spec == 11:
                god += 1
            elif spec == 13:
                naga += 1
        if hero.cls == 2:
            druid += 1
        elif hero.cls == 6:
            mech += 1
        elif hero.cls == 7:
            shaman += 1
        elif hero.cls == 8:
            knight += 1
        elif hero.cls == 9:
            demon_hunter += 1
    if orc >= 2:
        result += WEIGHT2
    if beast >= 2:
        result += WEIGHT2
    if human >= 2:
        result += WEIGHT2
    if undead >= 2:
        result += WEIGHT2
    if troll >= 2:
        result += WEIGHT2
    if elemental >= 2:
        result += WEIGHT2
    if god >= 2:
        result += WEIGHT2
    if naga >= 2:
        result += WEIGHT2
    if druid >= 2:
        result += WEIGHT2
    if mech >= 2:
        result += WEIGHT2
    if shaman >= 2:
        result += WEIGHT2
    if knight >= 2:
        result += WEIGHT2
    if demon_hunter >= 2:
        result += WEIGHT2
    return result

def fit3(heroes):
    result = 0
    dragon = 0
    elf = 0
    goblin = 0
    warrior = 0
    mage = 0
    hunter = 0
    assassin = 0
    warlock = 0
    for hero in heroes:
        for spec in hero.spec:
            if spec == 4:
                dragon += 1
            elif spec == 5:
                elf += 1
            elif spec == 8:
                goblin += 1
        if hero.cls == 1:
            warrior += 1
        elif hero.cls == 3:
            mage += 1
        elif hero.cls == 4:
            hunter += 1
        elif hero.cls == 5:
            assassin += 1
        elif hero.cls == 10:
            warlock += 1
    if dragon >= 3:
        result += WEIGHT3
    if elf >= 3:
        result += WEIGHT3
    if goblin >= 3:
        result += WEIGHT3
    if warrior >= 3:
        result += WEIGHT3
    if mage >= 3:
        result += WEIGHT3
    if hunter >= 3:
        result += WEIGHT3
    if assassin >= 3:
        result += WEIGHT3
    if warlock >= 3:
        result += WEIGHT3
    return result

def fit4(heroes):
    result = 0
    orc = 0
    beast = 0
    human = 0
    undead = 0
    elemental = 0

    druid = 0
    mech = 0
    knight = 0

    for hero in heroes:
        for spec in hero.spec:
            if spec == 1:
                orc += 1
            elif spec == 2:
                beast += 1
            elif spec == 3:
                human += 1
            elif spec == 7:
                undead += 1
            elif spec == 10:
                elemental += 1
        if hero.cls == 2:
            druid += 1
        elif hero.cls == 6:
            mech += 1
        elif hero.cls == 8:
            knight += 1
    if orc >= 4:
        result += WEIGHT4
    if beast >= 4:
        result += WEIGHT4
    if human >= 4:
        result += WEIGHT4
    if undead >= 4:
        result += WEIGHT4
    if elemental >= 4:
        result += WEIGHT4
    if druid >= 4:
        result += WEIGHT4
    if mech >= 4:
        result += WEIGHT4
    if knight >= 4:
        result += WEIGHT4
    return result

def fit6(heroes):
    result = 0

    human = 0
    goblin = 0

    warrior = 0
    mage = 0
    hunter = 0
    assassin = 0
    knight = 0
    warlock = 0

    for hero in heroes:
        for spec in hero.spec:
            if spec == 3:
                human += 1
            elif spec == 8:
                goblin += 1
        if hero.cls == 1:
            warrior += 1
        elif hero.cls == 3:
            mage += 1
        elif hero.cls == 4:
            hunter += 1
        elif hero.cls == 5:
            assassin += 1
        elif hero.cls == 8:
            knight += 1
        elif hero.cls == 10:
            warlock += 1
    if human >= 6:
        result += WEIGHT6
    if goblin >= 6:
        result += WEIGHT6
    if warrior >= 6:
        result += WEIGHT6
    if mage >= 6:
        result += WEIGHT6
    if hunter >= 6:
        result += WEIGHT6
    if assassin >= 6:
        result += WEIGHT6
    if knight >= 6:
        result += WEIGHT6
    if warlock >= 6:
        result += WEIGHT6
    return result

def fit9(heroes):
    result = 0

    elf = 0

    warrior = 0

    for hero in heroes:
        for spec in hero.spec:
            if spec == 5:
                elf += 1
        if hero.cls == 1:
            warrior += 1
    if elf >= 9:
        result += WEIGHT9
    if warrior >= 9:
        result += WEIGHT9
    return result


if __name__ == '__main__':
    main()