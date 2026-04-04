// Route2.js — Route 2 map script (JSON-bound architecture)

const EVENT = {
  GOT_MOON_STONE_ROUTE2: "EVENT_GOT_MOON_STONE_ROUTE2",
  GOT_HP_UP_ROUTE2: "EVENT_GOT_HP_UP_ROUTE2",
};

export async function talkPokeBall() {
  if (game.getFlag(EVENT.GOT_MOON_STONE_ROUTE2)) {
    await game.showText("The item ball is empty.");
    return;
  }
  const given = await game.giveItem("MOON_STONE", 1);
  if (given) {
    game.setFlag(EVENT.GOT_MOON_STONE_ROUTE2);
    await game.showText("<PLAYER> found\nMOON_STONE!");
  } else {
    await game.showText("You have too much\nstuff already!");
  }
}

export async function talkPokeBall1() {
  if (game.getFlag(EVENT.GOT_HP_UP_ROUTE2)) {
    await game.showText("The item ball is empty.");
    return;
  }
  const given = await game.giveItem("HP_UP", 1);
  if (given) {
    game.setFlag(EVENT.GOT_HP_UP_ROUTE2);
    await game.showText("<PLAYER> found\nHP_UP!");
  } else {
    await game.showText("You have too much\nstuff already!");
  }
}

export async function sign1() {
  await game.showText(
    "ROUTE 2\nVIRIDIAN CITY -\nPEWTER CITY"
  );
}

export async function sign2() {
  await game.showText("DIGLETT's CAVE");
}