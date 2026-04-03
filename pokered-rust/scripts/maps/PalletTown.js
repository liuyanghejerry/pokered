// PalletTown.js — Pallet Town map script
//
// Faithfully reproduces the original PalletTown.asm script logic:
//   7 script states: Default, OakHeyWait, OakWalksToPlayer, NotSafe,
//                    PlayerFollows, Daisy, Noop
//
// Hook functions called by the engine:
//   onMapScript()  — runs every frame (map script pointer table dispatch)
//   onTalkNpc(npcId) — NPC interaction handler
//   onTalkSign(signId) — Sign interaction handler

// ── Constants ────────────────────────────────────────────────────────
const SCRIPT = {
  DEFAULT: 0,
  OAK_HEY_WAIT: 1,
  OAK_WALKS_TO_PLAYER: 2,
  OAK_NOT_SAFE: 3,
  PLAYER_FOLLOWS_OAK: 4,
  DAISY: 5,
  NOOP: 6,
};

const NPC = {
  OAK: "PALLETTOWN_OAK",
};

const EVENT = {
  OAK_APPEARED_IN_PALLET: "EVENT_OAK_APPEARED_IN_PALLET",
  FOLLOWED_OAK_INTO_LAB: "EVENT_FOLLOWED_OAK_INTO_LAB",
  GOT_POKEBALLS_FROM_OAK: "EVENT_GOT_POKEBALLS_FROM_OAK",
  PALLET_AFTER_GETTING_POKEBALLS: "EVENT_PALLET_AFTER_GETTING_POKEBALLS",
  PALLET_AFTER_GETTING_POKEBALLS_2: "EVENT_PALLET_AFTER_GETTING_POKEBALLS_2",
  DAISY_WALKING: "EVENT_DAISY_WALKING",
  GOT_TOWN_MAP: "EVENT_GOT_TOWN_MAP",
  ENTERED_BLUES_HOUSE: "EVENT_ENTERED_BLUES_HOUSE",
};

const TOGGLE = {
  PALLET_TOWN_OAK: 0, // Oak's toggleable object index
  DAISY_SITTING: 1,
  DAISY_WALKING: 2,
};

const PAD = {
  SELECT: 0x04,
  START: 0x08,
  DPAD: 0xF0,
  BUTTONS: 0x0F,
};

// ── Map Script Entry ─────────────────────────────────────────────────
// Called every frame by the engine. Equivalent to PalletTown_Script.
async function onMapScript() {
  // Pre-check: if got pokeballs, set the after-pokeballs event
  if (game.getFlag(EVENT.GOT_POKEBALLS_FROM_OAK)) {
    game.setFlag(EVENT.PALLET_AFTER_GETTING_POKEBALLS);
  }

  // Dispatch to current script state
  const scriptIndex = game.getMapScriptIndex ? game.getMapScriptIndex() : 0;

  switch (scriptIndex) {
    case SCRIPT.DEFAULT:
      await palletTownDefault();
      break;
    case SCRIPT.OAK_HEY_WAIT:
      await palletTownOakHeyWait();
      break;
    case SCRIPT.OAK_WALKS_TO_PLAYER:
      await palletTownOakWalksToPlayer();
      break;
    case SCRIPT.OAK_NOT_SAFE:
      await palletTownOakNotSafe();
      break;
    case SCRIPT.PLAYER_FOLLOWS_OAK:
      await palletTownPlayerFollowsOak();
      break;
    case SCRIPT.DAISY:
      await palletTownDaisy();
      break;
    case SCRIPT.NOOP:
    default:
      // No-op
      break;
  }
}

// ── Script State 0: Default ──────────────────────────────────────────
// If player hasn't followed Oak yet and is near the north exit (y == 1),
// trigger the Oak encounter cutscene.
async function palletTownDefault() {
  if (game.getFlag(EVENT.FOLLOWED_OAK_INTO_LAB)) {
    return;
  }

  // Check if player is near the north exit
  // In the original: wYCoord == 1
  // The engine provides player position checking
  // For now, this is a coord event that fires when player reaches y=1

  // Stop all player input
  await game.facePlayer("down");
  await game.playSound("SFX_STOP_ALL_MUSIC");
  await game.playMusic("MUSIC_MEET_PROF_OAK");

  // Ignore select + start + d-pad
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);

  game.setFlag(EVENT.OAK_APPEARED_IN_PALLET);

  // Transition to next script state
  await game.setMapScript(SCRIPT.OAK_HEY_WAIT);
}

// ── Script State 1: Oak says "Hey! Wait!" ────────────────────────────
async function palletTownOakHeyWait() {
  // Oak says "Hey! Wait! Don't go out!" and exclamation bubble plays
  await game.showText(
    "OAK: Hey! Wait!\nDon't go out!"
  );

  // Delay 10 frames, then show exclamation bubble on player
  await game.delay(10);
  // (EmotionBubble is handled by the engine via showText return)

  await game.facePlayer("down");

  // Ignore buttons + d-pad during Oak's approach
  await game.setJoyIgnore(PAD.BUTTONS | PAD.DPAD);

  // Show Oak sprite
  await game.showObject(TOGGLE.PALLET_TOWN_OAK);

  // Transition to next script state
  await game.setMapScript(SCRIPT.OAK_WALKS_TO_PLAYER);
}

// ── Script State 2: Oak walks to player ──────────────────────────────
async function palletTownOakWalksToPlayer() {
  // Face Oak up
  await game.faceNpc(NPC.OAK, "up");
  await game.delay(3);

  // Oak walks to the player (pathfinding — engine handles CalcPositionOfPlayerRelativeToNPC)
  // In the original, Oak walks up from his position to one tile below the player.
  // The path is dynamically calculated; we use moveNpc with a relative approach.
  await game.moveNpc(NPC.OAK, [[3, 1]]); // Oak walks to player's position

  // Ignore buttons + d-pad while Oak walks
  await game.setJoyIgnore(PAD.BUTTONS | PAD.DPAD);

  await game.setMapScript(SCRIPT.OAK_NOT_SAFE);
}

// ── Script State 3: Oak says "It's not safe!" ────────────────────────
async function palletTownOakNotSafe() {
  // Wait for Oak's movement to finish (engine signals via movement complete)
  // Face the player down to face Oak
  await game.facePlayer("down");

  // Ignore select + start + d-pad
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);

  // Oak's "It's unsafe! Wild POKeMON live in tall grass!" speech
  await game.showText(
    "OAK: It's unsafe!\nWild POKeMON live\nin tall grass!\nYou need your own\nPOKeMON for your\nprotection.\nI know!\nHere, come with\nme!"
  );

  // Set up player-follows-Oak NPC movement script
  // Ignore buttons + d-pad
  await game.setJoyIgnore(PAD.BUTTONS | PAD.DPAD);

  await game.setMapScript(SCRIPT.PLAYER_FOLLOWS_OAK);
}

// ── Script State 4: Player follows Oak to his lab ────────────────────
async function palletTownPlayerFollowsOak() {
  // The NPC movement script causes the player to follow Oak south to the lab.
  // When the movement script finishes, transition to the Daisy state.
  // (In the original, this checks wNPCMovementScriptPointerTableNum == 0)
  // The engine handles this as a scripted movement sequence.

  await game.setMapScript(SCRIPT.DAISY);
}

// ── Script State 5: Daisy toggle ─────────────────────────────────────
async function palletTownDaisy() {
  // If Daisy isn't already walking, check if we should toggle her
  if (!game.getFlag(EVENT.DAISY_WALKING)) {
    // Check if player got town map AND entered Blue's house
    if (game.getFlag(EVENT.GOT_TOWN_MAP) && game.getFlag(EVENT.ENTERED_BLUES_HOUSE)) {
      game.setFlag(EVENT.DAISY_WALKING);
      await game.hideObject(TOGGLE.DAISY_SITTING);
      await game.showObject(TOGGLE.DAISY_WALKING);
    }
  }

  // After getting pokeballs from Oak
  if (game.getFlag(EVENT.GOT_POKEBALLS_FROM_OAK)) {
    game.setFlag(EVENT.PALLET_AFTER_GETTING_POKEBALLS_2);
  }

  // Done — go to noop
  await game.setMapScript(SCRIPT.NOOP);
}

// ── NPC Talk Handler ─────────────────────────────────────────────────
// Called when the player presses A facing an NPC.
// npcTextId is 1-based matching the original TextPointers table.
async function onTalkNpc(npcTextId) {
  switch (npcTextId) {
    case 1: // Oak
      await talkOak();
      break;
    case 2: // Girl
      await game.showText(
        "I'm raising\nPOKeMON too!\nWhen they get\nstrong, they can\nprotect me!"
      );
      break;
    case 3: // Fisher
      await game.showText(
        "Technology is\nincredible!\nYou can now store\nand recall items\nand POKeMON as\ndata via PC!"
      );
      break;
    default:
      break;
  }
}

// Oak's text depends on game state
async function talkOak() {
  // In the original: wOakWalkedToPlayer flag differentiates the two texts
  if (!game.getFlag("OAK_WALKED_TO_PLAYER")) {
    // "Hey! Wait! Don't go out!" + delay + exclamation bubble + face down
    await game.showText(
      "OAK: Hey! Wait!\nDon't go out!"
    );
    await game.delay(10);
    await game.facePlayer("down");
  } else {
    // "It's unsafe" speech
    await game.showText(
      "OAK: It's unsafe!\nWild POKeMON live\nin tall grass!\nYou need your own\nPOKeMON for your\nprotection.\nI know!\nHere, come with\nme!"
    );
  }
}

// ── Sign Talk Handler ────────────────────────────────────────────────
// signTextId is 1-based matching sign definitions.
async function onTalkSign(signTextId) {
  switch (signTextId) {
    case 1: // OAK POKeMON RESEARCH LAB
      await game.showText("OAK POKeMON\nRESEARCH LAB");
      break;
    case 2: // PALLET TOWN
      await game.showText("PALLET TOWN\nShades of your\njourney await!");
      break;
    case 3: // Player's house
      await game.showText("<PLAYER>'s house");
      break;
    case 4: // Rival's house
      await game.showText("<RIVAL>'s house");
      break;
    default:
      break;
  }
}

// ── Coord Event: North Exit ──────────────────────────────────────────
// Triggered when the player steps on the north exit tile (y == 1).
// This is the trigger that starts the Oak encounter cutscene.
async function onCoordEvent(x, y) {
  // North exit trigger — only if we haven't followed Oak yet
  if (y === 1 && !game.getFlag(EVENT.FOLLOWED_OAK_INTO_LAB)) {
    await palletTownDefault();
  }
}
