// PewterCity.js — Pewter City map script (JSON-bound architecture)

const EVENT = {
  BEAT_BROCK: "EVENT_BEAT_BROCK",
  BOUGHT_MUSEUM_TICKET: "EVENT_BOUGHT_MUSEUM_TICKET",
};

export async function talkCooltrainerF() {
  await game.showText(
    "It's rumored that\nCLEFAIRYs came\nfrom the moon!\n\nThey appeared\nafter MOON STONE\nfell on MT.MOON."
  );
}

export async function talkCooltrainerM() {
  await game.showText(
    "There aren't many\nserious POKeMON\ntrainers here!\n\nThey're all like\nBUG CATCHERs,\nbut PEWTER GYM's\nBROCK is totally\ninto it!"
  );
}

export async function talkSuperNerd() {
  const choice = await game.showTextChoice(
    "Did you check out\nthe MUSEUM?",
    ["YES", "NO"]
  );

  if (choice === 0) {
    await game.showText(
      "Weren't those\nfossils from MT.\nMOON amazing?"
    );
  } else {
    await game.showText(
      "Really?\nYou absolutely\nhave to go!"
    );
    // TODO: Guide player to museum
  }
}

export async function talkSuperNerd1() {
  const choice = await game.showTextChoice(
    "Psssst!\nDo you know what\nI'm doing?",
    ["YES", "NO"]
  );

  if (choice === 0) {
    await game.showText("That's right!\nIt's hard work!");
  } else {
    await game.showText(
      "I'm spraying REPEL\nto keep POKeMON\nout of my garden!"
    );
  }
}

export async function talkYoungster() {
  if (game.getFlag(EVENT.BEAT_BROCK)) {
    await game.showText(
      "If you have the\nright stuff, go\ntake on BROCK!"
    );
  } else {
    await game.showText(
      "You're a trainer\nright? BROCK's\nlooking for new\nchallengers!\nFollow me!"
    );
    // TODO: Guide player to Gym (complex script)
  }
}

export async function sign1() {
  await game.showText(
    "TRAINER TIPS\n\nAny POKeMON that\ntakes part in\nbattle, however\nshort, earns EXP!"
  );
}

export async function sign2() {
  await game.showText(
    "NOTICE!\n\nThieves have been\nstealing POKeMON\nfossils at MT.\nMOON! Please call\nPEWTER POLICE\nwith any info!"
  );
}

export async function sign3() {
  await game.showText("PEWTER MUSEUM\nOF SCIENCE");
}

export async function sign4() {
  await game.showText("HEAL YOUR POKeMON!\nPOKeMON CENTER");
}

export async function sign5() {
  await game.showText(
    "FOR ALL YOUR\nPOKeMON NEEDS\nPOKeMON MART"
  );
}

export async function sign6() {
  await game.showText(
    "PEWTER CITY\nPOKeMON GYM\nLEADER: BROCK\n\nThe Rock Solid\nPOKeMON Trainer!"
  );
}

export async function sign7() {
  await game.showText(
    "PEWTER CITY\nA Stone Gray\nCity"
  );
}