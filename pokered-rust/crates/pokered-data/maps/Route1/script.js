// Route1.js — Route 1 map script (JSON-bound architecture)

const EVENT = {
  GOT_POTION_SAMPLE: "EVENT_GOT_POTION_SAMPLE",
};

// ── NPC Handlers ─────────────────────────────────────────────────────

export async function talkYoungster() {
  // First Youngster - Mart employee giving potion sample
  if (game.getFlag(EVENT.GOT_POTION_SAMPLE)) {
    await game.showText(
      "We also carry\nPOKe BALLs for\ncatching POKeMON!"
    );
    return;
  }

  await game.showText(
    "Hi! I work at a\nPOKeMON MART.\n\nIt's a convenient\nshop, so please\nvisit us in\nVIRIDIAN CITY.\n\nI know, I'll give\nyou a sample!\nHere you go!"
  );

  const given = await game.giveItem("POTION", 1);
  if (given) {
    game.setFlag(EVENT.GOT_POTION_SAMPLE);
    await game.showText("<PLAYER> got\na POTION!");
  } else {
    await game.showText(
      "You have too much\nstuff with you!"
    );
  }
}

export async function talkYoungster1() {
  // Second Youngster - explains ledges
  await game.showText(
    "See those ledges\nalong the road?\n\nIt's a bit scary,\nbut you can jump\nfrom them.\n\nYou can get back\nto PALLET TOWN\nquicker that way."
  );
}

// ── Sign Handlers ────────────────────────────────────────────────────

export async function sign1() {
  await game.showText(
    "ROUTE 1\nPALLET TOWN -\nVIRIDIAN CITY"
  );
}