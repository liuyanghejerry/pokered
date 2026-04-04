// PalletTown.js — Pallet Town map script (JSON-bound architecture)

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
  PALLET_TOWN_OAK: 0,
  DAISY_SITTING: 1,
  DAISY_WALKING: 2,
};

const PAD = {
  SELECT: 0x04,
  START: 0x08,
  DPAD: 0xF0,
  BUTTONS: 0x0F,
};

// Lab entrance coordinates (where Oak and player end up)
const LAB_ENTRANCE_X = 12;
const LAB_ENTRANCE_Y = 11;

// ── onLoad callback (called once on map entry) ──────────────────────

export async function palletTownOnLoad() {
  if (game.getFlag(EVENT.GOT_POKEBALLS_FROM_OAK)) {
    game.setFlag(EVENT.PALLET_AFTER_GETTING_POKEBALLS);
  }
}

// ── Script Chain Functions ───────────────────────────────────────────

async function palletTownOakHeyWait() {
  await game.showText("OAK: Hey! Wait!\nDon't go out!");
  await game.delay(10);
  await game.facePlayer("down");
  await game.setJoyIgnore(PAD.BUTTONS | PAD.DPAD);
  await game.showObject(TOGGLE.PALLET_TOWN_OAK);
  await palletTownOakWalksToPlayer();
}

async function palletTownOakWalksToPlayer() {
  const pos = game.getPlayerPosition();
  const oakTargetX = pos.x;
  const oakTargetY = pos.y + 1;

  await game.faceNpc(NPC.OAK, "up");
  await game.delay(3);
  const oakSpawnX = 8;
  const oakSpawnY = 2;
  await game.moveNpc(NPC.OAK, [[oakSpawnX, oakSpawnY], [oakTargetX, oakTargetY]]);
  await game.setJoyIgnore(PAD.BUTTONS | PAD.DPAD);
  await palletTownOakNotSafe();
}

async function palletTownOakNotSafe() {
  await game.facePlayer("down");
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);
  await game.showText(
    "OAK: It's unsafe!\nWild POKeMON live\nin tall grass!\nYou need your own\nPOKeMON for your\nprotection.\nI know!\nHere, come with\nme!"
  );
  await game.setJoyIgnore(PAD.BUTTONS | PAD.DPAD);
  await palletTownPlayerFollowsOak();
}

async function palletTownPlayerFollowsOak() {
  const pos = game.getPlayerPosition();
  const playerX = pos.x;
  const playerY = pos.y;
  const oakX = playerX;
  const oakY = playerY + 1;

  const oakPath = [];
  for (let y = oakY + 1; y <= 7; y++) {
    oakPath.push([oakX, y]);
  }
  if (oakX > 9) {
    for (let x = oakX - 1; x >= 9; x--) {
      oakPath.push([x, 7]);
    }
  }
  for (let y = 8; y <= 12; y++) {
    oakPath.push([9, y]);
  }
  for (let x = 10; x <= LAB_ENTRANCE_X; x++) {
    oakPath.push([x, 12]);
  }
  oakPath.push([LAB_ENTRANCE_X, LAB_ENTRANCE_Y]);

  const playerPath = [];
  playerPath.push([playerX, playerY - 1]);
  for (let x = playerX + 1; x <= 13; x++) {
    playerPath.push([x, 0]);
  }
  for (let y = 1; y <= 7; y++) {
    playerPath.push([13, y]);
  }
  playerPath.push([12, 7]);
  for (let y = 8; y <= LAB_ENTRANCE_Y; y++) {
    playerPath.push([12, y]);
  }

  game.startNpcMove(NPC.OAK, oakPath);
  await game.movePlayer(playerPath);
  await game.awaitNpcMove(NPC.OAK);
  await game.hideObject(TOGGLE.PALLET_TOWN_OAK);
  game.setFlag(EVENT.FOLLOWED_OAK_INTO_LAB);
  await palletTownDaisy();
}

async function palletTownDaisy() {
  if (!game.getFlag(EVENT.DAISY_WALKING)) {
    if (game.getFlag(EVENT.GOT_TOWN_MAP) && game.getFlag(EVENT.ENTERED_BLUES_HOUSE)) {
      game.setFlag(EVENT.DAISY_WALKING);
      await game.hideObject(TOGGLE.DAISY_SITTING);
      await game.showObject(TOGGLE.DAISY_WALKING);
    }
  }
  if (game.getFlag(EVENT.GOT_POKEBALLS_FROM_OAK)) {
    game.setFlag(EVENT.PALLET_AFTER_GETTING_POKEBALLS_2);
  }
}

// ── NPC handlers (bound via npcs[] in PalletTown.json) ───────────────

export async function talkOak() {
  if (!game.getFlag("OAK_WALKED_TO_PLAYER")) {
    await game.showText("OAK: Hey! Wait!\nDon't go out!");
    await game.delay(10);
    await game.facePlayer("down");
  } else {
    await game.showText(
      "OAK: It's unsafe!\nWild POKeMON live\nin tall grass!\nYou need your own\nPOKeMON for your\nprotection.\nI know!\nHere, come with\nme!"
    );
  }
}

export async function talkGirl() {
  await game.showText(
    "I'm raising\nPOKeMON too!\nWhen they get\nstrong, they can\nprotect me!"
  );
}

export async function talkFisher() {
  await game.showText(
    "Technology is\nincredible!\nYou can now store\nand recall items\nand POKeMON as\ndata via PC!"
  );
}

// ── Sign handlers (bound via signs[] in PalletTown.json) ─────────────

export async function signOakLab() {
  await game.showText("OAK POKeMON\nRESEARCH LAB");
}

export async function signPalletTown() {
  await game.showText("PALLET TOWN\nShades of your\njourney await!");
}

export async function signPlayersHouse() {
  await game.showText("<PLAYER>'s house");
}

export async function signRivalsHouse() {
  await game.showText("<RIVAL>'s house");
}

// ── Coord event handlers (bound via coordEvents[] in PalletTown.json) ─

export async function coordNorthExit() {
  if (game.getFlag(EVENT.FOLLOWED_OAK_INTO_LAB)) {
    return;
  }
  await game.facePlayer("down");
  await game.playSound("SFX_STOP_ALL_MUSIC");
  await game.playMusic("MUSIC_MEET_PROF_OAK");
  await game.setJoyIgnore(PAD.SELECT | PAD.START | PAD.DPAD);
  game.setFlag(EVENT.OAK_APPEARED_IN_PALLET);
  await palletTownOakHeyWait();
}
