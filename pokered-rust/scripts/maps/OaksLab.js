const SCRIPT = {
  DEFAULT: 0, OAK_ENTERS_LAB: 1, TOGGLE_OAKS: 2, PLAYER_ENTERS_LAB: 3,
  FOLLOWED_OAK: 4, OAK_CHOOSE_MON_SPEECH: 5, PLAYER_DONT_GO_AWAY: 6,
  PLAYER_FORCED_WALK_BACK: 7, CHOSE_STARTER: 8, RIVAL_CHOOSES_STARTER: 9,
  RIVAL_CHALLENGES_PLAYER: 10, RIVAL_START_BATTLE: 11, RIVAL_END_BATTLE: 12,
  RIVAL_STARTS_EXIT: 13, PLAYER_WATCH_RIVAL_EXIT: 14,
  RIVAL_ARRIVES_AT_OAKS_REQUEST: 15, OAK_GIVES_POKEDEX: 16,
  RIVAL_LEAVES_WITH_POKEDEX: 17, NOOP: 18,
};

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


// ── Script State Functions ───────────────────────────────────────────

// State 0: Default — check if Oak appeared in Pallet, show Oak2, transition
async function scriptDefault() {
  if (!game.getFlag(EVENT.OAK_APPEARED_IN_PALLET)) {
    return;
  }
  await game.showObject(TOGGLE.OAK2);
  await game.setMapScript(SCRIPT.OAK_ENTERS_LAB);
}

// State 1: Oak enters the lab — move Oak2 up 3 tiles
async function scriptOakEntersLab() {
  await game.moveNpc(NPC.OAK2, ["up", "up", "up"]);
  await game.setMapScript(SCRIPT.TOGGLE_OAKS);
}

// State 2: Toggle Oak sprites — hide Oak2, show Oak1
async function scriptToggleOaks() {
  await game.hideObject(TOGGLE.OAK2);
  await game.showObject(TOGGLE.OAK1);
  await game.setMapScript(SCRIPT.PLAYER_ENTERS_LAB);
}

// State 3: Player enters lab — walk player up 8 tiles, face rival+oak down
async function scriptPlayerEntersLab() {
  await game.delay(3);
  // Simulate player walking up 8 tiles
  await game.moveNpc("PLAYER", ["up", "up", "up", "up", "up", "up", "up", "up"]);
  // Face rival and oak down
  await game.faceNpc(NPC.RIVAL, "down");
  await game.faceNpc(NPC.OAK1, "down");
  await game.setMapScript(SCRIPT.FOLLOWED_OAK);
}

// State 4: Followed Oak — set event flags, face rival up, play default music
async function scriptFollowedOak() {
  game.setFlag(EVENT.FOLLOWED_OAK_INTO_LAB);
  game.setFlag(EVENT.FOLLOWED_OAK_INTO_LAB_2);
  await game.faceNpc(NPC.RIVAL, "up");
  await game.playMusic("MUSIC_PALLET_TOWN");
  await game.setMapScript(SCRIPT.OAK_CHOOSE_MON_SPEECH);
}

// State 5: Oak's choose-mon speech — 4 dialogs, set event
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

  await game.setMapScript(SCRIPT.PLAYER_DONT_GO_AWAY);
}

// State 6: Player don't go away — if player at y==6, push them back
async function scriptPlayerDontGoAway() {
  // This checks every frame: if player reaches y==6 (near the exit)
  const pos = game.getPlayerPos ? game.getPlayerPos() : null;
  if (!pos || pos.y !== 6) {
    return;
  }
  // Oak and rival face down
  await game.faceNpc(NPC.OAK1, "down");
  await game.faceNpc(NPC.RIVAL, "down");
  await game.showText("OAK: Hey! Don't\ngo away yet!");
  // Push the player back up 1 tile
  await game.moveNpc("PLAYER", ["up"]);
  await game.facePlayer("up");
  await game.setMapScript(SCRIPT.PLAYER_FORCED_WALK_BACK);
}

// State 7: Player forced to walk back — wait, delay, loop back to state 6
async function scriptPlayerForcedWalkBack() {
  await game.delay(3);
  await game.setMapScript(SCRIPT.PLAYER_DONT_GO_AWAY);
}

// State 8: Chose starter — move rival to the ball based on player's starter
async function scriptChoseStarter() {
  const starter = game.getVar ? game.getVar("wPlayerStarter") : STARTER.CHARMANDER;
  const playerPos = game.getPlayerPos ? game.getPlayerPos() : { x: 0, y: 3 };

  if (starter === STARTER.CHARMANDER) {
    // Rival goes to the middle ball
    if (playerPos.y === 4) {
      // Player below the table
      await game.moveNpc(NPC.RIVAL, ["down", "down", "right", "right", "right", "up"]);
    } else {
      await game.moveNpc(NPC.RIVAL, ["down", "right", "right", "right"]);
    }
  } else if (starter === STARTER.SQUIRTLE) {
    // Rival goes to the right ball
    if (playerPos.y === 4) {
      await game.moveNpc(NPC.RIVAL, ["down", "down", "right", "right", "right", "right", "up"]);
    } else {
      await game.moveNpc(NPC.RIVAL, ["down", "right", "right", "right", "right"]);
    }
  } else {
    // Bulbasaur — rival goes to the left ball
    await game.moveNpc(NPC.RIVAL, ["down", "right", "right"]);
  }

  await game.setMapScript(SCRIPT.RIVAL_CHOOSES_STARTER);
}

// State 9: Rival chooses starter — "I'll take this one!", hide ball, received mon
async function scriptRivalChoosesStarter() {
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);
  await game.faceNpc(NPC.RIVAL, "up");
  await game.showText("<RIVAL>: I'll\ntake this one\nthen!");

  // Hide the rival's ball based on which starter was taken
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

  await game.setMapScript(SCRIPT.RIVAL_CHALLENGES_PLAYER);
}

// State 10: Rival challenges player — at y==6, play rival music, rival walks to player
async function scriptRivalChallengesPlayer() {
  const pos = game.getPlayerPos ? game.getPlayerPos() : null;
  if (!pos || pos.y !== 6) {
    return;
  }
  await game.faceNpc(NPC.RIVAL, "down");
  await game.facePlayer("up");
  await game.playMusic("MUSIC_MEET_RIVAL");
  await game.showText("<RIVAL>: Wait\n<PLAYER>!\nLet's check out\nour POKeMON!\nCome on, I'll\ntake you on!");
  // Rival walks to player — the engine handles pathfinding
  await game.moveNpc(NPC.RIVAL, ["find_player"]);
  await game.setMapScript(SCRIPT.RIVAL_START_BATTLE);
}

// State 11: Rival starts battle
async function scriptRivalStartBattle() {
  await game.facePlayer("up");
  const result = await game.startBattle("OPP_RIVAL1");
  await game.setMapScript(SCRIPT.RIVAL_END_BATTLE);
}

// State 12: Rival end battle — face up, rival faces down, heal, set event
async function scriptRivalEndBattle() {
  await game.setJoyIgnore(PAD.DPAD);
  await game.facePlayer("up");
  await game.faceNpc(NPC.RIVAL, "down");
  await game.heal();
  game.setFlag(EVENT.BATTLED_RIVAL_IN_OAKS_LAB);
  await game.setMapScript(SCRIPT.RIVAL_STARTS_EXIT);
}

// State 13: Rival starts exit — delay, "Smell you later!", rival walks out
async function scriptRivalStartsExit() {
  await game.delay(20);
  await game.showText("<RIVAL>: Smell\nyou later!");
  await game.playMusic("MUSIC_MEET_RIVAL");

  // Rival walks down to the door and exits
  const playerPos = game.getPlayerPos ? game.getPlayerPos() : { x: 5, y: 6 };
  const sideStep = playerPos.x <= 4 ? "right" : "left";
  await game.moveNpc(NPC.RIVAL, [sideStep, "down", "down", "down", "down", "down"]);

  await game.setMapScript(SCRIPT.PLAYER_WATCH_RIVAL_EXIT);
}

// State 14: Player watches rival exit — hide rival, play default music, noop
async function scriptPlayerWatchRivalExit() {
  await game.hideObject(TOGGLE.RIVAL);
  await game.clearJoyIgnore();
  await game.playMusic("MUSIC_PALLET_TOWN");
  await game.setMapScript(SCRIPT.NOOP);
}

// State 15: Rival arrives at Oak's request — stop music, play rival music, show rival
async function scriptRivalArrivesAtOaksRequest() {
  await game.playSound("SFX_STOP_ALL_MUSIC");
  await game.playMusic("MUSIC_MEET_RIVAL");
  await game.showText("<RIVAL>: Gramps!");

  // Show rival and move them up to Oak
  await game.showObject(TOGGLE.RIVAL);
  // Rival walks up from the door to near Oak
  const numSteps = game.getVar ? (game.getVar("wRivalMovementSteps") || 4) : 4;
  const path = [];
  for (let i = 0; i < numSteps; i++) {
    path.push("up");
  }
  await game.moveNpc(NPC.RIVAL, path);

  await game.setMapScript(SCRIPT.OAK_GIVES_POKEDEX);
}

// State 16: Oak gives Pokédex — multiple dialogs, give pokedex, rival exits
async function scriptOakGivesPokedex() {
  await game.playMusic("MUSIC_PALLET_TOWN");
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);

  // Rival face up, Oak face down pattern for dialog
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

  // Give Pokédex text
  await game.showText("<PLAYER> got a\nPOKeDEX from\nOAK!");

  await game.delay(3);
  // Hide the Pokédex objects on the desk
  await game.hideObject(TOGGLE.POKEDEX_1);
  await game.hideObject(TOGGLE.POKEDEX_2);

  // More dialog
  await game.faceNpc(NPC.RIVAL, "up");
  await game.faceNpc(NPC.OAK1, "down");
  await game.showText("To make a\ncomplete guide\non all the\nPOKeMON in the\nworld...\nThat was my\ndream!\nBut, I'm too old!\nI can't do it!\nSo, I want you\ntwo to fulfill my\ndream for me!\nGet moving, you\ntwo!\nThis is a great\nundertaking in\nPOKeMON history!");

  // Rival faces right, pauses, then speaks
  await game.faceNpc(NPC.RIVAL, "right");
  await game.delay(3);
  await game.showText("<RIVAL>: Alright\nGramps! Leave it\nall to me!\n\n<PLAYER>, I hate\nto say it, but I\ndon't need you!\n\nI know! I'll\nborrow a TOWN MAP\nfrom my sis!\n\nI'll tell her not\nto lend you one,\n<PLAYER>! Hahaha!");

  game.setFlag(EVENT.GOT_POKEDEX);
  game.setFlag(EVENT.OAK_GOT_PARCEL);

  // Toggle the old man objects (Viridian)
  await game.hideObject(TOGGLE.LYING_OLD_MAN);
  await game.showObject(TOGGLE.OLD_MAN);

  // Rival walks back down and exits with music
  await game.playSound("SFX_STOP_ALL_MUSIC");
  await game.playMusic("MUSIC_MEET_RIVAL");

  const stepsDown = game.getVar ? (game.getVar("wRivalMovementSteps") || 4) : 4;
  const exitPath = [];
  for (let i = 0; i < stepsDown; i++) {
    exitPath.push("down");
  }
  await game.moveNpc(NPC.RIVAL, exitPath);

  await game.setMapScript(SCRIPT.RIVAL_LEAVES_WITH_POKEDEX);
}

// State 17: Rival leaves with Pokédex — hide rival, set route 22 events
async function scriptRivalLeavesWithPokedex() {
  await game.playMusic("MUSIC_PALLET_TOWN");
  await game.hideObject(TOGGLE.RIVAL);

  // Set Route 22 rival battle events
  game.setFlag(EVENT.FIRST_ROUTE22_RIVAL_BATTLE);
  game.resetFlag(EVENT.SECOND_ROUTE22_RIVAL_BATTLE);
  game.setFlag(EVENT.ROUTE22_RIVAL_WANTS_BATTLE);

  // Show Route 22 rival object
  await game.showObject(TOGGLE.ROUTE_22_RIVAL);

  // Also set Pallet Town to Daisy script
  // (This is wPalletTownCurScript = SCRIPT_PALLETTOWN_DAISY)

  await game.clearJoyIgnore();
  await game.setMapScript(SCRIPT.NOOP);
}

// State 18: Noop
async function scriptNoop() {
  // Do nothing
}

// ── Map Script Dispatcher ────────────────────────────────────────────
// Called every frame by the engine. Dispatches to current script state.
async function onMapScript() {
  // Pre-check: if EVENT_PALLET_AFTER_GETTING_POKEBALLS_2, load text pointers 2
  // (In the original this swaps the text pointer table)

  const scriptIndex = game.getMapScriptIndex ? game.getMapScriptIndex() : 0;

  switch (scriptIndex) {
    case SCRIPT.DEFAULT:                      await scriptDefault(); break;
    case SCRIPT.OAK_ENTERS_LAB:               await scriptOakEntersLab(); break;
    case SCRIPT.TOGGLE_OAKS:                  await scriptToggleOaks(); break;
    case SCRIPT.PLAYER_ENTERS_LAB:            await scriptPlayerEntersLab(); break;
    case SCRIPT.FOLLOWED_OAK:                 await scriptFollowedOak(); break;
    case SCRIPT.OAK_CHOOSE_MON_SPEECH:        await scriptOakChooseMonSpeech(); break;
    case SCRIPT.PLAYER_DONT_GO_AWAY:          await scriptPlayerDontGoAway(); break;
    case SCRIPT.PLAYER_FORCED_WALK_BACK:      await scriptPlayerForcedWalkBack(); break;
    case SCRIPT.CHOSE_STARTER:                await scriptChoseStarter(); break;
    case SCRIPT.RIVAL_CHOOSES_STARTER:        await scriptRivalChoosesStarter(); break;
    case SCRIPT.RIVAL_CHALLENGES_PLAYER:      await scriptRivalChallengesPlayer(); break;
    case SCRIPT.RIVAL_START_BATTLE:           await scriptRivalStartBattle(); break;
    case SCRIPT.RIVAL_END_BATTLE:             await scriptRivalEndBattle(); break;
    case SCRIPT.RIVAL_STARTS_EXIT:            await scriptRivalStartsExit(); break;
    case SCRIPT.PLAYER_WATCH_RIVAL_EXIT:      await scriptPlayerWatchRivalExit(); break;
    case SCRIPT.RIVAL_ARRIVES_AT_OAKS_REQUEST: await scriptRivalArrivesAtOaksRequest(); break;
    case SCRIPT.OAK_GIVES_POKEDEX:            await scriptOakGivesPokedex(); break;
    case SCRIPT.RIVAL_LEAVES_WITH_POKEDEX:    await scriptRivalLeavesWithPokedex(); break;
    case SCRIPT.NOOP:
    default:
      break;
  }
}

// ── NPC Talk Handler ─────────────────────────────────────────────────
// npcTextId is 1-based matching OaksLab_TextPointers / TextPointers2.
async function onTalkNpc(npcTextId) {
  switch (npcTextId) {
    case 1:  await talkRival(); break;
    case 2:  await talkCharmanderBall(); break;
    case 3:  await talkSquirtleBall(); break;
    case 4:  await talkBulbasaurBall(); break;
    case 5:  await talkOak1(); break;
    case 6:  await talkPokedex(); break;
    case 7:  await talkPokedex(); break;
    case 8:  // Oak2 text
      await game.showText("OAK: Right!\nYoung man and\nyoung lady!\nFollow me!");
      break;
    case 9:  // Girl
      await game.showText("I want to go on\nan adventure too,\nbut Prof. OAK won't\nlet me...");
      break;
    case 10: // Scientist 1
    case 11: // Scientist 2
      await game.showText("I study POKeMON\nas PROF.OAK's\nAIDE.");
      break;
    default: break;
  }
}

// ── Rival NPC Text ───────────────────────────────────────────────────
async function talkRival() {
  if (!game.getFlag(EVENT.FOLLOWED_OAK_INTO_LAB_2)) {
    await game.showText("<RIVAL>: Heh,\nI don't need to\nbe Pokemon to\nbeat you!\n...Gramps isn't\naround.");
  } else if (!game.getFlag(EVENT.GOT_STARTER)) {
    await game.showText("<RIVAL>: Go\nahead and choose,\n<PLAYER>!");
  } else {
    await game.showText("<RIVAL>: My\nPOKeMON looks a\nlot tougher\nthan yours!");
  }
}

// ── Poké Ball NPC Texts (starters) ──────────────────────────────────
async function talkCharmanderBall() {
  await handlePokeBallInteraction(STARTER.CHARMANDER, "CHARMANDER_BALL", STARTER.SQUIRTLE, "SQUIRTLE_BALL");
}

async function talkSquirtleBall() {
  await handlePokeBallInteraction(STARTER.SQUIRTLE, "SQUIRTLE_BALL", STARTER.BULBASAUR, "BULBASAUR_BALL");
}

async function talkBulbasaurBall() {
  await handlePokeBallInteraction(STARTER.BULBASAUR, "BULBASAUR_BALL", STARTER.CHARMANDER, "CHARMANDER_BALL");
}

// Common handler for interacting with a starter Poké Ball on the table
async function handlePokeBallInteraction(starterSpecies, playerBallId, rivalStarter, rivalBallId) {
  if (game.getFlag(EVENT.GOT_STARTER)) {
    // Already got a starter — "That's the last POKeMON!"
    await game.faceNpc(NPC.OAK1, "down");
    await game.showText("OAK: If a wild\nPOKeMON appears,\nyour POKeMON\ncan fight\nagainst it!");
    return;
  }

  if (!game.getFlag(EVENT.OAK_ASKED_TO_CHOOSE_MON)) {
    // Oak hasn't given permission yet
    await game.showText("Those are POKe\nBALLs. They\ncontain POKeMON!");
    return;
  }

  // Oak and rival face the player's direction
  await game.faceNpc(NPC.OAK1, "down");
  await game.faceNpc(NPC.RIVAL, "right");

  // Show the starter info (Pokédex-style)
  await game.showText("So! You want the\n" + getStarterTypeName(starterSpecies) + "\nPOKeMON,\n" + starterSpecies + "?");

  // Yes/No choice
  const choice = await game.showChoice(["YES", "NO"]);
  if (choice !== 0) {
    // Player chose NO
    return;
  }

  // Player chose YES — give the starter
  await game.showText("This POKeMON is\nreally energetic!");
  await game.showText("<PLAYER> received\na " + starterSpecies + "!");
  await game.givePokemon(starterSpecies, 5);

  // Hide the ball object
  if (starterSpecies === STARTER.CHARMANDER) {
    await game.hideObject(TOGGLE.STARTER_BALL_1);
  } else if (starterSpecies === STARTER.SQUIRTLE) {
    await game.hideObject(TOGGLE.STARTER_BALL_2);
  } else {
    await game.hideObject(TOGGLE.STARTER_BALL_3);
  }

  // Set joy ignore and transition to the rival choosing their starter
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);
  await game.setMapScript(SCRIPT.CHOSE_STARTER);
}

function getStarterTypeName(species) {
  switch (species) {
    case STARTER.CHARMANDER: return "fire";
    case STARTER.SQUIRTLE:   return "water";
    case STARTER.BULBASAUR:  return "grass";
    default: return "unknown";
  }
}

// ── Oak1 NPC Text (complex branching) ────────────────────────────────
async function talkOak1() {
  // Check if we're in the late game (after getting Pokéballs)
  if (game.getFlag(EVENT.PALLET_AFTER_GETTING_POKEBALLS_2)) {
    await game.showText("OAK: How is your\nPOKeDEX coming?\nHere, let me see.");
    // TODO: DisplayDexRating
    return;
  }

  // Check if player has Poké Balls in bag
  // (simplified — engine handles inventory checks)
  if (game.getFlag(EVENT.GOT_POKEBALLS_FROM_OAK)) {
    await game.showText("OAK: Come see me\nsometimes.");
    return;
  }

  if (game.getFlag(EVENT.BEAT_ROUTE22_RIVAL_1ST)) {
    // Give Poké Balls
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
    // Check for Oak's Parcel
    if (game.getFlag("HAS_OAKS_PARCEL")) {
      // Deliver the parcel
      await game.showText("OAK: Oh, <PLAYER>!\nWhat's this?\nA parcel for me?\nThank you!");
      await game.takeItem("OAKS_PARCEL", 1);
      // Trigger rival arrives script
      await game.setMapScript(SCRIPT.RIVAL_ARRIVES_AT_OAKS_REQUEST);
      return;
    }
    await game.showText("OAK: Raise your\nyoung POKeMON by\nmaking it fight!");
    return;
  }

  // Before rival battle but after getting starter
  if (game.getFlag(EVENT.GOT_STARTER)) {
    await game.showText("OAK: Your very\nown POKeMON can\nfight for you!\nYou should go\ntry battling\nnearby trainers!");
    return;
  }

  // Default: haven't chosen yet
  await game.showText("OAK: Now,\n<PLAYER>, which\nPOKeMON do you\nwant?");
}

// ── Pokédex on Desk ──────────────────────────────────────────────────
async function talkPokedex() {
  await game.showText("It's a POKeDEX!\nA hi-tech\nencyclopedia!");
}

// ── Sign Talk Handler ────────────────────────────────────────────────
async function onTalkSign(signTextId) {
  // Oak's Lab has no signs (signs array is empty in the original)
}

// ── Coord Event Handler ──────────────────────────────────────────────
async function onCoordEvent(x, y) {
  // No coord events defined for Oak's Lab
}
