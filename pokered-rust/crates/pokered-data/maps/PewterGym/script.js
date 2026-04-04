// PewterGym.js — Pewter Gym map script (JSON-bound architecture)

const EVENT = {
  BEAT_BROCK: "EVENT_BEAT_BROCK",
  GOT_TM34: "EVENT_GOT_TM34",
  BEAT_GYM_TRAINER_0: "EVENT_BEAT_PEWTER_GYM_TRAINER_0",
};

export async function talkSuperNerd() {
  if (game.getFlag(EVENT.BEAT_BROCK)) {
    await game.showText(
      "I learned a lot\nfrom BROCK!\nROCK-type POKeMON\nare tough!"
    );
  } else {
    await game.startTrainerBattle("YOUNGSTER", 1);
    game.setFlag(EVENT.BEAT_GYM_TRAINER_0);
  }
}

export async function talkCooltrainerM() {
  // Brock - Gym Leader
  if (game.getFlag(EVENT.BEAT_BROCK)) {
    if (!game.getFlag(EVENT.GOT_TM34)) {
      await giveTM34();
    } else {
      await game.showText(
        "There are all\nkinds of trainers\nin the world!\nYou raise your\nPOKeMON well!"
      );
    }
    return;
  }

  await game.showText(
    "I'm BROCK!\nI'm PEWTER's GYM\nLEADER!\n\nI believe in rock\nhard defense and\ndetermination!\n\nThat's why my\nPOKeMON are all\nthe rock-type!\n\nDo you still want\nto challenge me?\nFine then! Show\nme your best!"
  );

  const result = await game.startGymBattle("BROCK", 1);
  if (result === "win") {
    game.setFlag(EVENT.BEAT_BROCK);
    await game.showText(
      "I took you for\ngranted.\nAs proof of your\nvictory, here's\nthe BOULDERBADGE!"
    );
    await giveTM34();
    await game.giveBadge("BOULDERBADGE");
  }
}

async function giveTM34() {
  const given = await game.giveItem("TM_34", 1);
  if (given) {
    game.setFlag(EVENT.GOT_TM34);
    await game.showText(
      "<PLAYER> received\nTM34!\n\nTM34 is BIDE!\nYour POKeMON will\nabsorb damage in\nbattle then pay\nit back double!"
    );
  } else {
    await game.showText("You have too much\nstuff already!");
  }
}

export async function talkGymGuide() {
  if (game.getFlag(EVENT.BEAT_BROCK)) {
    await game.showText(
      "You beat BROCK!\nI'm impressed!\nThat BOULDERBADGE\nmakes all POKeMON\nup to L20 obey!"
    );
  } else {
    const choice = await game.showTextChoice(
      "Hi there! I'm the\nGYM GUIDE! I show\ntrainers the way!\nWant my advice?",
      ["YES", "NO"]
    );

    if (choice === 0) {
      await game.showText(
        "All right! Let me\ntell you about\nBROCK! He uses\nrock-type POKeMON!\nWater and grass\nPOKeMON work best!"
      );
    } else {
      await game.showText("It's a free\nservice! Come back\nanytime!");
    }
  }
}