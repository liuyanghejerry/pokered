const EVENT = {
  GOT_STARTER: "EVENT_GOT_STARTER",
};

async function onTalkNpc(npcTextId) {
  switch (npcTextId) {
    case 1:
      await talkMom();
      break;
    case 2:
      await talkTV();
      break;
  }
}

async function talkMom() {
  if (!game.getFlag(EVENT.GOT_STARTER)) {
    await game.showText(
      "MOM: Right.\nAll boys leave\nhome some day.\nIt said so on TV.\nPROF.OAK, next\ndoor, is looking\nfor you."
    );
  } else {
    await game.showText(
      "MOM: <PLAYER>,\nyou should take a\nnap before you go.\n...Right.\nAll boys leave\nhome some day.\nIt said so on TV."
    );
  }
}

async function talkTV() {
  await game.showText(
    "There's a movie\non TV. Four boys\nare walking on\nrailroad tracks.\nI better go too."
  );
}

async function onTalkSign(_signTextId) {}
async function onMapScript() {}
async function onCoordEvent(_x, _y) {}
