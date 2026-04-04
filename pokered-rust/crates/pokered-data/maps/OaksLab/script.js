// OaksLab.js — Oak's Lab map script (JSON-bound architecture)

const NPC = { OAK1: "OAKSLAB_OAK1", OAK2: "OAKSLAB_OAK2", RIVAL: "OAKSLAB_RIVAL" };

const EVENT = {
  OAK_APPEARED_IN_PALLET: "EVENT_OAK_APPEARED_IN_PALLET",
  FOLLOWED_OAK_INTO_LAB: "EVENT_FOLLOWED_OAK_INTO_LAB",
  FOLLOWED_OAK_INTO_LAB_2: "EVENT_FOLLOWED_OAK_INTO_LAB_2",
  OAK_ASKED_TO_CHOOSE_MON: "EVENT_OAK_ASKED_TO_CHOOSE_MON",
  GOT_STARTER: "EVENT_GOT_STARTER",
  BATTLED_RIVAL_IN_OAKS_LAB: "EVENT_BATTLED_RIVAL_IN_OAKS_LAB",
  GOT_POKEDEX: "EVENT_GOT_POKEDEX",
  OAK_GOT_PARCEL: "EVENT_OAK_GOT_PARCEL",
  PALLET_AFTER_GETTING_POKEBALLS_2: "EVENT_PALLET_AFTER_GETTING_POKEBALLS_2",
  GOT_POKEBALLS_FROM_OAK: "EVENT_GOT_POKEBALLS_FROM_OAK",
  BEAT_ROUTE22_RIVAL_1ST: "EVENT_BEAT_ROUTE22_RIVAL_1ST_BATTLE",
  FIRST_ROUTE22_RIVAL_BATTLE: "EVENT_1ST_ROUTE22_RIVAL_BATTLE",
  SECOND_ROUTE22_RIVAL_BATTLE: "EVENT_2ND_ROUTE22_RIVAL_BATTLE",
  ROUTE22_RIVAL_WANTS_BATTLE: "EVENT_ROUTE22_RIVAL_WANTS_BATTLE",
};

const TOGGLE = {
  OAK1: 0, OAK2: 1, RIVAL: 2,
  STARTER_BALL_1: 3, STARTER_BALL_2: 4, STARTER_BALL_3: 5,
  POKEDEX_1: 6, POKEDEX_2: 7,
  LYING_OLD_MAN: 8, OLD_MAN: 9, ROUTE_22_RIVAL: 10,
};

const PAD = { SELECT: 0x04, START: 0x08, DPAD: 0xF0, BUTTONS: 0x0F };

const STARTER = { CHARMANDER: "CHARMANDER", SQUIRTLE: "SQUIRTLE", BULBASAUR: "BULBASAUR" };


// ── Script Chain Functions ────────────────────────────────────────────

export async function oaksLabOnLoad() {
  if (!game.getFlag(EVENT.OAK_APPEARED_IN_PALLET)) {
    return;
  }
  await game.showObject(TOGGLE.OAK2);
  await scriptOakEntersLab();
}

async function scriptOakEntersLab() {
  await game.moveNpc(NPC.OAK2, ["up", "up", "up"]);
  await scriptToggleOaks();
}

async function scriptToggleOaks() {
  await game.hideObject(TOGGLE.OAK2);
  await game.showObject(TOGGLE.OAK1);
  await scriptPlayerEntersLab();
}

async function scriptPlayerEntersLab() {
  await game.delay(3);
  await game.moveNpc("PLAYER", ["up", "up", "up", "up", "up", "up", "up", "up"]);
  await game.faceNpc(NPC.RIVAL, "down");
  await game.faceNpc(NPC.OAK1, "down");
  await scriptFollowedOak();
}

async function scriptFollowedOak() {
  game.setFlag(EVENT.FOLLOWED_OAK_INTO_LAB);
  game.setFlag(EVENT.FOLLOWED_OAK_INTO_LAB_2);
  await game.faceNpc(NPC.RIVAL, "up");
  await game.playMusic("MUSIC_PALLET_TOWN");
  await scriptOakChooseMonSpeech();
}

async function scriptOakChooseMonSpeech() {
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);

  await game.showText("<RIVAL>: Gramps!\nI'm fed up with\nwaiting!");
  await game.delay(3);
  await game.showText("OAK: <RIVAL>?\nLet me think...\nOh, that's right,\nI told you to\ncome! Just wait!\nHere, <PLAYER>.\nThere are 3\nPOKeMON here!\nHaha!\nThey are inside\nthe POKe BALLS.\nWhen I was young,\nI was a serious\nPOKeMON trainer!\nIn my old age, I\nhave only 3 left,\nbut you can have\none! Choose!");
  await game.delay(3);
  await game.showText("<RIVAL>: Hey!\nGramps! What\nabout me?");
  await game.delay(3);
  await game.showText("OAK: Be patient!\n<RIVAL>, you can\nhave one too!");

  game.setFlag(EVENT.OAK_ASKED_TO_CHOOSE_MON);
  await game.clearJoyIgnore();
}

async function scriptPlayerDontGoAway() {
  await game.faceNpc(NPC.OAK1, "down");
  await game.faceNpc(NPC.RIVAL, "down");
  await game.showText("OAK: Hey! Don't\ngo away yet!");
  await game.moveNpc("PLAYER", ["up"]);
  await game.facePlayer("up");
}

async function scriptChoseStarter() {
  const starter = game.getVar ? game.getVar("wPlayerStarter") : STARTER.CHARMANDER;
  const playerPos = game.getPlayerPos ? game.getPlayerPos() : { x: 0, y: 3 };

  if (starter === STARTER.CHARMANDER) {
    if (playerPos.y === 4) {
      await game.moveNpc(NPC.RIVAL, ["down", "down", "right", "right", "right", "up"]);
    } else {
      await game.moveNpc(NPC.RIVAL, ["down", "right", "right", "right"]);
    }
  } else if (starter === STARTER.SQUIRTLE) {
    if (playerPos.y === 4) {
      await game.moveNpc(NPC.RIVAL, ["down", "down", "right", "right", "right", "right", "up"]);
    } else {
      await game.moveNpc(NPC.RIVAL, ["down", "right", "right", "right", "right"]);
    }
  } else {
    await game.moveNpc(NPC.RIVAL, ["down", "right", "right"]);
  }

  await scriptRivalChoosesStarter();
}

async function scriptRivalChoosesStarter() {
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);
  await game.faceNpc(NPC.RIVAL, "up");
  await game.showText("<RIVAL>: I'll\ntake this one\nthen!");

  const rivalBall = game.getVar ? game.getVar("wRivalStarterBallSpriteIndex") : null;
  if (rivalBall === "CHARMANDER") {
    await game.hideObject(TOGGLE.STARTER_BALL_1);
  } else if (rivalBall === "SQUIRTLE") {
    await game.hideObject(TOGGLE.STARTER_BALL_2);
  } else {
    await game.hideObject(TOGGLE.STARTER_BALL_3);
  }

  await game.delay(3);
  await game.faceNpc(NPC.RIVAL, "up");
  await game.showText("<RIVAL> received\na @!");
  game.setFlag(EVENT.GOT_STARTER);
  await game.clearJoyIgnore();

  await scriptRivalChallengesPlayer();
}

async function scriptRivalChallengesPlayer() {
  await game.faceNpc(NPC.RIVAL, "down");
  await game.facePlayer("up");
  await game.playMusic("MUSIC_MEET_RIVAL");
  await game.showText("<RIVAL>: Wait\n<PLAYER>!\nLet's check out\nour POKeMON!\nCome on, I'll\ntake you on!");
  await game.moveNpc(NPC.RIVAL, ["find_player"]);
  await scriptRivalStartBattle();
}

async function scriptRivalStartBattle() {
  await game.facePlayer("up");
  const result = await game.startBattle("OPP_RIVAL1");
  await scriptRivalEndBattle();
}

async function scriptRivalEndBattle() {
  await game.setJoyIgnore(PAD.DPAD);
  await game.facePlayer("up");
  await game.faceNpc(NPC.RIVAL, "down");
  await game.heal();
  game.setFlag(EVENT.BATTLED_RIVAL_IN_OAKS_LAB);
  await scriptRivalStartsExit();
}

async function scriptRivalStartsExit() {
  await game.delay(20);
  await game.showText("<RIVAL>: Smell\nyou later!");
  await game.playMusic("MUSIC_MEET_RIVAL");

  const playerPos = game.getPlayerPos ? game.getPlayerPos() : { x: 5, y: 6 };
  const sideStep = playerPos.x <= 4 ? "right" : "left";
  await game.moveNpc(NPC.RIVAL, [sideStep, "down", "down", "down", "down", "down"]);

  await scriptPlayerWatchRivalExit();
}

async function scriptPlayerWatchRivalExit() {
  await game.hideObject(TOGGLE.RIVAL);
  await game.clearJoyIgnore();
  await game.playMusic("MUSIC_PALLET_TOWN");
}

async function scriptRivalArrivesAtOaksRequest() {
  await game.playSound("SFX_STOP_ALL_MUSIC");
  await game.playMusic("MUSIC_MEET_RIVAL");
  await game.showText("<RIVAL>: Gramps!");

  await game.showObject(TOGGLE.RIVAL);
  const numSteps = game.getVar ? (game.getVar("wRivalMovementSteps") || 4) : 4;
  const path = [];
  for (let i = 0; i < numSteps; i++) path.push("up");
  await game.moveNpc(NPC.RIVAL, path);

  await scriptOakGivesPokedex();
}

async function scriptOakGivesPokedex() {
  await game.playMusic("MUSIC_PALLET_TOWN");
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);

  await game.faceNpc(NPC.RIVAL, "up");
  await game.faceNpc(NPC.OAK1, "down");
  await game.showText("<RIVAL>: What\ndid you call me\nfor?");

  await game.delay(1);
  await game.faceNpc(NPC.RIVAL, "up");
  await game.faceNpc(NPC.OAK1, "down");
  await game.showText("OAK: Oh right! I\nhave a request\nof you two.\n\nOn the desk there\nis my invention,\nPOKeDEX!\nIt automatically\nrecords data on\nPOKeMON you've\nseen or caught!\nIt's a hi-tech\nencyclopedia!");

  await game.delay(1);
  await game.faceNpc(NPC.RIVAL, "up");
  await game.faceNpc(NPC.OAK1, "down");
  await game.showText("OAK: <PLAYER>\nand <RIVAL>! Take\nthese with you!");

  await game.showText("<PLAYER> got a\nPOKeDEX from\nOAK!");

  await game.delay(3);
  await game.hideObject(TOGGLE.POKEDEX_1);
  await game.hideObject(TOGGLE.POKEDEX_2);

  await game.faceNpc(NPC.RIVAL, "up");
  await game.faceNpc(NPC.OAK1, "down");
  await game.showText("To make a\ncomplete guide\non all the\nPOKeMON in the\nworld...\nThat was my\ndream!\nBut, I'm too old!\nI can't do it!\nSo, I want you\ntwo to fulfill my\ndream for me!\nGet moving, you\ntwo!\nThis is a great\nundertaking in\nPOKeMON history!");

  await game.faceNpc(NPC.RIVAL, "right");
  await game.delay(3);
  await game.showText("<RIVAL>: Alright\nGramps! Leave it\nall to me!\n\n<PLAYER>, I hate\nto say it, but I\ndon't need you!\n\nI know! I'll\nborrow a TOWN MAP\nfrom my sis!\n\nI'll tell her not\nto lend you one,\n<PLAYER>! Hahaha!");

  game.setFlag(EVENT.GOT_POKEDEX);
  game.setFlag(EVENT.OAK_GOT_PARCEL);

  await game.hideObject(TOGGLE.LYING_OLD_MAN);
  await game.showObject(TOGGLE.OLD_MAN);

  await game.playSound("SFX_STOP_ALL_MUSIC");
  await game.playMusic("MUSIC_MEET_RIVAL");

  const stepsDown = game.getVar ? (game.getVar("wRivalMovementSteps") || 4) : 4;
  const exitPath = [];
  for (let i = 0; i < stepsDown; i++) exitPath.push("down");
  await game.moveNpc(NPC.RIVAL, exitPath);

  await scriptRivalLeavesWithPokedex();
}

async function scriptRivalLeavesWithPokedex() {
  await game.playMusic("MUSIC_PALLET_TOWN");
  await game.hideObject(TOGGLE.RIVAL);

  game.setFlag(EVENT.FIRST_ROUTE22_RIVAL_BATTLE);
  game.resetFlag(EVENT.SECOND_ROUTE22_RIVAL_BATTLE);
  game.setFlag(EVENT.ROUTE22_RIVAL_WANTS_BATTLE);

  await game.showObject(TOGGLE.ROUTE_22_RIVAL);

  await game.clearJoyIgnore();
}


// ── Coord event handlers (bound via coordEvents[] in OaksLab.json) ───

export async function coordExitRow() {
  if (game.getFlag(EVENT.OAK_ASKED_TO_CHOOSE_MON) && !game.getFlag(EVENT.GOT_STARTER)) {
    await scriptPlayerDontGoAway();
    return;
  }
  if (game.getFlag(EVENT.GOT_STARTER) && !game.getFlag(EVENT.BATTLED_RIVAL_IN_OAKS_LAB)) {
    await scriptRivalChallengesPlayer();
    return;
  }
}

// ── NPC Talk Handlers (named for JSON binding) ───────────────────────

export async function talkRival() {
  if (!game.getFlag(EVENT.FOLLOWED_OAK_INTO_LAB_2)) {
    await game.showText("<RIVAL>: Heh,\nI don't need to\nbe Pokemon to\nbeat you!\n...Gramps isn't\naround.");
  } else if (!game.getFlag(EVENT.GOT_STARTER)) {
    await game.showText("<RIVAL>: Go\nahead and choose,\n<PLAYER>!");
  } else {
    await game.showText("<RIVAL>: My\nPOKeMON looks a\nlot tougher\nthan yours!");
  }
}

export async function talkCharmanderBall() {
  await handlePokeBallInteraction(STARTER.CHARMANDER, "CHARMANDER_BALL", STARTER.SQUIRTLE, "SQUIRTLE_BALL");
}

export async function talkSquirtleBall() {
  await handlePokeBallInteraction(STARTER.SQUIRTLE, "SQUIRTLE_BALL", STARTER.BULBASAUR, "BULBASAUR_BALL");
}

export async function talkBulbasaurBall() {
  await handlePokeBallInteraction(STARTER.BULBASAUR, "BULBASAUR_BALL", STARTER.CHARMANDER, "CHARMANDER_BALL");
}

async function handlePokeBallInteraction(starterSpecies, playerBallId, rivalStarter, rivalBallId) {
  if (game.getFlag(EVENT.GOT_STARTER)) {
    await game.faceNpc(NPC.OAK1, "down");
    await game.showText("OAK: If a wild\nPOKeMON appears,\nyour POKeMON\ncan fight\nagainst it!");
    return;
  }

  if (!game.getFlag(EVENT.OAK_ASKED_TO_CHOOSE_MON)) {
    await game.showText("Those are POKe\nBALLs. They\ncontain POKeMON!");
    return;
  }

  await game.faceNpc(NPC.OAK1, "down");
  await game.faceNpc(NPC.RIVAL, "right");

  await game.showText("So! You want the\n" + getStarterTypeName(starterSpecies) + "\nPOKeMON,\n" + starterSpecies + "?");

  const choice = await game.showChoice(["YES", "NO"]);
  if (choice !== 0) {
    return;
  }

  await game.showText("This POKeMON is\nreally energetic!");
  await game.showText("<PLAYER> received\na " + starterSpecies + "!");
  await game.givePokemon(starterSpecies, 5);

  if (starterSpecies === STARTER.CHARMANDER) {
    await game.hideObject(TOGGLE.STARTER_BALL_1);
  } else if (starterSpecies === STARTER.SQUIRTLE) {
    await game.hideObject(TOGGLE.STARTER_BALL_2);
  } else {
    await game.hideObject(TOGGLE.STARTER_BALL_3);
  }

  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);
  await scriptChoseStarter();
}

function getStarterTypeName(species) {
  switch (species) {
    case STARTER.CHARMANDER: return "fire";
    case STARTER.SQUIRTLE:   return "water";
    case STARTER.BULBASAUR:  return "grass";
    default: return "unknown";
  }
}

export async function talkOak1() {
  if (game.getFlag(EVENT.PALLET_AFTER_GETTING_POKEBALLS_2)) {
    await game.showText("OAK: How is your\nPOKeDEX coming?\nHere, let me see.");
    return;
  }

  if (game.getFlag(EVENT.GOT_POKEBALLS_FROM_OAK)) {
    await game.showText("OAK: Come see me\nsometimes.");
    return;
  }

  if (game.getFlag(EVENT.BEAT_ROUTE22_RIVAL_1ST)) {
    if (!game.getFlag(EVENT.GOT_POKEBALLS_FROM_OAK)) {
      game.setFlag(EVENT.GOT_POKEBALLS_FROM_OAK);
      await game.giveItem("POKE_BALL", 5);
      await game.showText("<PLAYER> got 5\nPOKe BALLs from\nOAK!");
      await game.showText("OAK: You can't\nget detailed data\non POKeMON by\njust seeing them.\nYou have to\ncatch them!\nUse these to\ncatch wild\nPOKeMON.");
      return;
    }
    await game.showText("OAK: Come see me\nsometimes.");
    return;
  }

  if (game.getFlag(EVENT.GOT_POKEDEX)) {
    await game.showText("OAK: <PLAYER>,\nhave you seen\nPOKeMON on\nROUTE 1?\nYou can find wild\nPOKeMON in tall\ngrass!");
    return;
  }

  if (game.getFlag(EVENT.BATTLED_RIVAL_IN_OAKS_LAB)) {
    if (game.getFlag("HAS_OAKS_PARCEL")) {
      await game.showText("OAK: Oh, <PLAYER>!\nWhat's this?\nA parcel for me?\nThank you!");
      await game.takeItem("OAKS_PARCEL", 1);
      await scriptRivalArrivesAtOaksRequest();
      return;
    }
    await game.showText("OAK: Raise your\nyoung POKeMON by\nmaking it fight!");
    return;
  }

  if (game.getFlag(EVENT.GOT_STARTER)) {
    await game.showText("OAK: Your very\nown POKeMON can\nfight for you!\nYou should go\ntry battling\nnearby trainers!");
    return;
  }

  await game.showText("OAK: Now,\n<PLAYER>, which\nPOKeMON do you\nwant?");
}

export async function talkPokedex() {
  await game.showText("It's a POKeDEX!\nA hi-tech\nencyclopedia!");
}

// New small functions to replace inline dialog cases
export async function talkOak2() {
  await game.showText("OAK: Right!\nYoung man and\nyoung lady!\nFollow me!");
}

export async function talkLabGirl() {
  await game.showText("I want to go on\nan adventure too,\nbut Prof. OAK won't\nlet me...");
}

export async function talkScientist() {
  await game.showText("I study POKeMON\nas PROF.OAK's\nAIDE.");
}
