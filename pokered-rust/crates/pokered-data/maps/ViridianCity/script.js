// ViridianCity.js — Viridian City map script (JSON-bound architecture)

const EVENT = {
  GOT_POKEDEX: "EVENT_GOT_POKEDEX",
  VIRIDIAN_GYM_OPEN: "EVENT_VIRIDIAN_GYM_OPEN",
  GOT_TM42: "EVENT_GOT_TM42",
  BEAT_VIRIDIAN_GYM_GIOVANNI: "EVENT_BEAT_VIRIDIAN_GYM_GIOVANNI",
};

const PAD = { SELECT: 0x04, START: 0x08, DPAD: 0xF0, BUTTONS: 0x0F };

// ── Map Scripts ──────────────────────────────────────────────────────

export async function scriptDefault() {
  // Check if Gym should open (player has all badges except Earth Badge)
  if (game.getFlag(EVENT.VIRIDIAN_GYM_OPEN)) return;

  const badges = game.getBadges ? game.getBadges() : 0;
  // If player has 7 badges (all except Earth Badge), open the Gym
  if (badges === 0x7F) { // 0111 1111 = 7 badges
    game.setFlag(EVENT.VIRIDIAN_GYM_OPEN);
  }
}

// ── NPC Handlers ─────────────────────────────────────────────────────

export async function talkYoungster() {
  await game.showText(
    "Those POKe BALLs\nat your waist!\nYou have POKeMON!\n\nIt's great that\nyou can carry and\nuse POKeMON any\ntime, anywhere!"
  );
}

export async function talkGambler() {
  // Gambler near Gym - comments on Gym status
  const badges = game.getBadges ? game.getBadges() : 0;
  const hasAllButEarthBadge = badges === 0x7F;

  if (hasAllButEarthBadge || game.getFlag(EVENT.BEAT_VIRIDIAN_GYM_GIOVANNI)) {
    await game.showText(
      "VIRIDIAN GYM's\nLEADER returned!"
    );
  } else {
    await game.showText(
      "This POKeMON GYM\nis always closed.\n\nI wonder who the\nLEADER is?"
    );
  }
}

export async function talkYoungster1() {
  const choice = await game.showTextChoice(
    "You want to know\nabout the 2 kinds\nof caterpillar\nPOKeMON?",
    ["YES", "NO"]
  );

  if (choice === 1) {
    await game.showText("Oh, OK then!");
  } else {
    await game.showText(
      "CATERPIE has no\npoison, but\nWEEDLE does.\n\nWatch out for its\nPOISON STING!"
    );
  }
}

export async function talkGirl() {
  if (!game.getFlag(EVENT.GOT_POKEDEX)) {
    await game.showText(
      "Oh Grandpa! Don't\nbe so mean!\nHe hasn't had his\ncoffee yet."
    );
  } else {
    await game.showText(
      "When I go shop in\nPEWTER CITY, I\nhave to take the\nwinding trail in\nVIRIDIAN FOREST."
    );
  }
}

export async function talkGamblerAsleep() {
  // Old Man blocking the path (before getting Pokedex)
  await game.showText(
    "You can't go\nthrough here!\n\nThis is private\nproperty!"
  );
  // Force player to move down
  await game.movePlayer(["down"]);
}

export async function talkFisher() {
  if (game.getFlag(EVENT.GOT_TM42)) {
    await game.showText(
      "TM42 contains\nDREAM EATER...\n...Snore..."
    );
    return;
  }

  await game.showText(
    "Yawn!\nI must have dozed\noff in the sun.\n\nI had this dream\nabout a DROWZEE\neating my dream.\nWhat's this?\nWhere did this TM\ncome from?\n\nThis is spooky!\nHere, you can\nhave this TM."
  );

  const given = await game.giveItem("TM_42", 1);
  if (given) {
    game.setFlag(EVENT.GOT_TM42);
    await game.showText("<PLAYER> received\nTM42!");
  } else {
    await game.showText(
      "You have too much\nstuff already."
    );
  }
}

export async function talkGambler1() {
  // Old Man after getting Pokedex (coffee drunk)
  const choice = await game.showTextChoice(
    "Ahh, I've had my\ncoffee now and I\nfeel great!\n\nSure you can go\nthrough!\n\nAre you in a\nhurry?",
    ["YES", "NO"]
  );

  if (choice === 0) {
    // In a hurry - let them pass
    await game.showText(
      "Time is money...\nGo along then."
    );
  } else {
    // Not in a hurry - offer catching tutorial
    await game.showText(
      "I see you're using\na POKeDEX.\n\nWhen you catch a\nPOKeMON, POKeDEX\nis automatically\nupdated.\n\nWhat? Don't you\nknow how to catch\nPOKeMON?\n\nI'll show you\nhow to then."
    );
    // Start catching tutorial
    await startCatchingTutorial();
  }
}

async function startCatchingTutorial() {
  // Set up battle for Old Man tutorial
  await game.showText(
    "First, you need\nto weaken the\ntarget POKeMON."
  );
  // TODO: Implement actual tutorial battle
  // For now, just show the text
}

// ── Sign Handlers ────────────────────────────────────────────────────

export async function sign1() {
  // City sign
  await game.showText(
    "VIRIDIAN CITY\nThe Eternally\nGreen Paradise"
  );
}

export async function sign2() {
  // Trainer Tips 1
  await game.showText(
    "TRAINER TIPS\n\nCatch POKeMON\nand expand your\ncollection!\n\nThe more you have,\nthe easier it is\nto fight!"
  );
}

export async function sign3() {
  // Pokecenter sign
  await game.showText(
    "HEAL YOUR POKeMON!\nPOKeMON CENTER"
  );
}

export async function sign4() {
  // Mart sign
  await game.showText(
    "FOR ALL YOUR\nPOKeMON NEEDS\nPOKeMON MART"
  );
}

export async function sign5() {
  // Trainer Tips 2
  await game.showText(
    "TRAINER TIPS\n\nThe battle moves\nof POKeMON are\nlimited by their\nPOWER POINTs, PP.\n\nTo replenish PP,\nrest your tired\nPOKeMON at a\nPOKeMON CENTER!"
  );
}

export async function sign6() {
  // Gym sign
  await game.showText(
    "VIRIDIAN CITY\nPOKeMON GYM"
  );
}