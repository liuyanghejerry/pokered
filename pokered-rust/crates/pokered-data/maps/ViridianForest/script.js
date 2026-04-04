// ViridianForest.js — Viridian Forest map script (JSON-bound architecture)

const EVENT = {
  BEAT_FOREST_TRAINER_0: "EVENT_BEAT_VIRIDIAN_FOREST_TRAINER_0",
  BEAT_FOREST_TRAINER_1: "EVENT_BEAT_VIRIDIAN_FOREST_TRAINER_1",
  BEAT_FOREST_TRAINER_2: "EVENT_BEAT_VIRIDIAN_FOREST_TRAINER_2",
  GOT_ANTIDOTE: "EVENT_GOT_ANTIDOTE_VIRIDIAN_FOREST",
  GOT_POTION: "EVENT_GOT_POTION_VIRIDIAN_FOREST",
  GOT_POKEBALL: "EVENT_GOT_POKEBALL_VIRIDIAN_FOREST",
};

export async function talkYoungster() {
  await game.showText(
    "I came here with\nsome friends!\n\nThey're out for\nPOKeMON fights!"
  );
}

export async function talkYoungster1() {
  if (game.getFlag(EVENT.BEAT_FOREST_TRAINER_0)) {
    await game.showText(
      "Ssh! You'll scare\nthe bugs away!"
    );
  } else {
    await game.startTrainerBattle("BUG_CATCHER", 1);
    game.setFlag(EVENT.BEAT_FOREST_TRAINER_0);
  }
}

export async function talkYoungster2() {
  if (game.getFlag(EVENT.BEAT_FOREST_TRAINER_1)) {
    await game.showText(
      "Darn! I'm going\nto catch some\nstronger ones!"
    );
  } else {
    await game.startTrainerBattle("BUG_CATCHER", 2);
    game.setFlag(EVENT.BEAT_FOREST_TRAINER_1);
  }
}

export async function talkYoungster3() {
  if (game.getFlag(EVENT.BEAT_FOREST_TRAINER_2)) {
    await game.showText(
      "Sometimes, you\ncan find stuff on\nthe ground!\n\nI'm looking for\nthe stuff I\ndropped!"
    );
  } else {
    await game.startTrainerBattle("BUG_CATCHER", 3);
    game.setFlag(EVENT.BEAT_FOREST_TRAINER_2);
  }
}

export async function talkPokeBall() {
  if (game.getFlag(EVENT.GOT_ANTIDOTE)) {
    await game.showText("The item ball is empty.");
    return;
  }
  const given = await game.giveItem("ANTIDOTE", 1);
  if (given) {
    game.setFlag(EVENT.GOT_ANTIDOTE);
    await game.showText("<PLAYER> found\nANTIDOTE!");
  } else {
    await game.showText("You have too much\nstuff already!");
  }
}

export async function talkPokeBall1() {
  if (game.getFlag(EVENT.GOT_POTION)) {
    await game.showText("The item ball is empty.");
    return;
  }
  const given = await game.giveItem("POTION", 1);
  if (given) {
    game.setFlag(EVENT.GOT_POTION);
    await game.showText("<PLAYER> found\nPOTION!");
  } else {
    await game.showText("You have too much\nstuff already!");
  }
}

export async function talkPokeBall2() {
  if (game.getFlag(EVENT.GOT_POKEBALL)) {
    await game.showText("The item ball is empty.");
    return;
  }
  const given = await game.giveItem("POKE_BALL", 1);
  if (given) {
    game.setFlag(EVENT.GOT_POKEBALL);
    await game.showText("<PLAYER> found\nPOKE_BALL!");
  } else {
    await game.showText("You have too much\nstuff already!");
  }
}

export async function talkYoungster4() {
  await game.showText(
    "I ran out of POKe\nBALLs to catch\nPOKeMON with!\n\nYou should carry\nextras!"
  );
}

export async function sign1() {
  await game.showText(
    "TRAINER TIPS\n\nIf you want to\navoid battles,\nstay away from\ngrassy areas!"
  );
}

export async function sign2() {
  await game.showText(
    "For poison, use\nANTIDOTE! Get it\nat POKeMON MARTs!"
  );
}

export async function sign3() {
  await game.showText(
    "TRAINER TIPS\n\nContact PROF.OAK\nvia PC to get\nyour POKeDEX\nevaluated!"
  );
}

export async function sign4() {
  await game.showText(
    "TRAINER TIPS\n\nNo stealing of\nPOKeMON from\nother trainers!\nCatch only wild\nPOKeMON!"
  );
}

export async function sign5() {
  await game.showText(
    "LEAVING\nVIRIDIAN FOREST\nPEWTER CITY AHEAD"
  );
}

export async function sign6() {
  await game.showText(
    "TRAINER TIPS\n\nWeaken POKeMON\nbefore attempting\ncapture!\n\nWhen healthy,\nthey may escape!"
  );
}