// Museum2F.js — Museum 2F map script (JSON-bound architecture)

export async function talkYoungster() {
  await game.showText(
    "MOON STONE?\n\nWhat's so special\nabout it?"
  );
}

export async function talkGramps() {
  await game.showText(
    "July 20, 1969!\n\nThe 1st lunar\nlanding!\n\nI bought a color\nTV to watch it!"
  );
}

export async function talkScientist() {
  await game.showText(
    "We have a space\nexhibit now."
  );
}

export async function talkBrunetteGirl() {
  await game.showText(
    "I want a PIKACHU!\nIt's so cute!\n\nI asked my Daddy\nto catch me one!"
  );
}

export async function talkHiker() {
  await game.showText(
    "Yeah, a PIKACHU\nsoon, I promise!"
  );
}

export async function sign1() {
  await game.showText(
    "SPACE SHUTTLE\nCOLUMBIA"
  );
}

export async function sign2() {
  await game.showText(
    "Meteorite that\nfell on MT.MOON.\n(MOON STONE?)"
  );
}