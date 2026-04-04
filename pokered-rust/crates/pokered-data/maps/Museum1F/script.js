// Museum1F.js — Museum 1F map script (JSON-bound architecture)

const EVENT = {
  BOUGHT_MUSEUM_TICKET: "EVENT_BOUGHT_MUSEUM_TICKET",
  GOT_OLD_AMBER: "EVENT_GOT_OLD_AMBER",
};

export async function talkScientist() {
  if (!game.getFlag(EVENT.BOUGHT_MUSEUM_TICKET)) {
    await game.showText("Please go to the\nother side!");
    return;
  }
  await game.showText("Take plenty of\ntime to look!");
}

export async function talkGambler() {
  await game.showText(
    "That is one\nmagnificent\nfossil!"
  );
}

export async function talkScientist1() {
  const pos = game.getPlayerPos ? game.getPlayerPos() : { x: 0, y: 0 };

  if (pos.y === 4 && pos.x >= 12) {
    const choice = await game.showTextChoice(
      "You can't sneak\nin the back way!\n\nOh, whatever!\nDo you know what\nAMBER is?",
      ["YES", "NO"]
    );

    if (choice === 0) {
      await game.showText(
        "There's a lab\nsomewhere trying\nto resurrect\nancient POKeMON\nfrom AMBER."
      );
    } else {
      await game.showText(
        "AMBER is fossil-\nized tree sap."
      );
    }
    return;
  }

  if (game.getFlag(EVENT.BOUGHT_MUSEUM_TICKET)) {
    await game.showText("Take plenty of\ntime to look!");
    return;
  }

  await game.showText(
    "It's ¥50 for a\nchild's ticket.\n\nWould you like to\ncome in?"
  );

  const choice = await game.showTextChoice("", ["YES", "NO"]);

  if (choice === 1) {
    await game.showText("Come again!");
    await game.movePlayer(["down"]);
    return;
  }

  const money = game.getMoney ? game.getMoney() : 0;
  if (money < 50) {
    await game.showText(
      "You don't have\nenough money."
    );
    await game.movePlayer(["down"]);
    return;
  }

  await game.takeMoney(50);
  game.setFlag(EVENT.BOUGHT_MUSEUM_TICKET);
  await game.showText("Right, ¥50!\nThank you!");
}

export async function talkScientist2() {
  if (game.getFlag(EVENT.GOT_OLD_AMBER)) {
    await game.showText("Ssh! Get the OLD\nAMBER checked!");
    return;
  }

  await game.showText(
    "Ssh! I think that\nthis chunk of\nAMBER contains\nPOKeMON DNA!\n\nIt would be great\nif POKeMON could\nbe resurrected\nfrom it!\n\nBut, my colleagues\njust ignore me!\n\nSo I have a favor\nto ask!\n\nTake this to a\nPOKeMON LAB and\nget it examined!"
  );

  const given = await game.giveItem("OLD_AMBER", 1);
  if (given) {
    game.setFlag(EVENT.GOT_OLD_AMBER);
    await game.showText("<PLAYER> received\nOLD AMBER!");
  } else {
    await game.showText("You don't have\nspace for this!");
  }
}

export async function talkOldAmber() {
  await game.showText(
    "The AMBER is\nclear and gold!"
  );
}