const EVENT = {
  DAISY_WALKING: "EVENT_DAISY_WALKING",
  GOT_TOWN_MAP: "EVENT_GOT_TOWN_MAP",
  ENTERED_BLUES_HOUSE: "EVENT_ENTERED_BLUES_HOUSE",
};

async function onEnter() {
  game.setFlag(EVENT.ENTERED_BLUES_HOUSE);
}

async function onTalkNpc(npcTextId) {
  switch (npcTextId) {
    case 1:
      await talkDaisySitting();
      break;
    case 2:
      await talkDaisyWalking();
      break;
    case 3:
      await talkTownMap();
      break;
  }
}

async function talkDaisySitting() {
  await game.showText(
    "Hi <PLAYER>!\n<RIVAL> is out at\nGrandpa's lab."
  );
}

async function talkDaisyWalking() {
  if (!game.getFlag(EVENT.GOT_TOWN_MAP)) {
    await game.showText(
      "Hi <PLAYER>!\nI just got a\nTOWN MAP.\nDid you get one\ntoo?\nHere, I'll give\nyou one!"
    );
    await game.giveItem("TOWN_MAP", 1);
    game.setFlag(EVENT.GOT_TOWN_MAP);
    await game.showText(
      "<PLAYER> got a\nTOWN MAP!"
    );
  } else {
    await game.showText(
      "POKeMON are living\nthings! If they\nget tired, give\nthem a rest!"
    );
  }
}

async function talkTownMap() {
  await game.showText("It's a big map!\nThis is useful!");
}

async function onTalkSign(_signTextId) {}
async function onMapScript() {}
async function onCoordEvent(_x, _y) {}
