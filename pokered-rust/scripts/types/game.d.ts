/**
 * pokered-script Game API Type Definitions
 *
 * These declarations describe the global `game` object injected by the
 * pokered-script Boa engine.  They are provided for editor autocompletion
 * and type-checking of map script files (PalletTown.js, OaksLab.js, …).
 *
 * Async methods return a Promise that resolves once the game loop has
 * finished executing the corresponding command (e.g. the text box has
 * been dismissed, the NPC has finished moving, the battle is over).
 *
 * Sync methods execute immediately inside the JS runtime and never yield
 * to the game loop.
 */

// ---------------------------------------------------------------------------
// Direction helpers (used by faceNpc / facePlayer)
// ---------------------------------------------------------------------------

/** Cardinal direction string accepted by facing commands. */
type Direction = "up" | "down" | "left" | "right";

/** Screen-fade type accepted by fadeScreen. */
type FadeType = "in" | "out";

// ---------------------------------------------------------------------------
// Global game object
// ---------------------------------------------------------------------------

declare const game: GameAPI;

interface GameAPI {
  // -----------------------------------------------------------------------
  // Text & Dialogue
  // -----------------------------------------------------------------------

  /**
   * Display a text box with the given string and wait for the player to
   * dismiss it.
   *
   * @param text - The dialogue text to display.  Supports control codes
   *               such as `<PLAYER>`, `<RIVAL>`, `<LINE>`, `<PAGE>`.
   */
  showText(text: string): Promise<void>;

  /**
   * Show a multiple-choice prompt and return the 0-based index of the
   * option the player selected.
   *
   * @param options - Array of choice labels (e.g. `["YES", "NO"]`).
   * @returns The index of the selected option.
   */
  showChoice(options: string[]): Promise<number>;

  // -----------------------------------------------------------------------
  // Items & Pokémon
  // -----------------------------------------------------------------------

  /**
   * Give an item to the player.
   *
   * @param itemId   - Item identifier string (e.g. `"POTION"`, `"TOWN_MAP"`).
   * @param quantity - Number of items to give (1-99).
   */
  giveItem(itemId: string, quantity: number): Promise<void>;

  /**
   * Remove an item from the player's bag.
   *
   * @param itemId   - Item identifier string.
   * @param quantity - Number of items to take.
   */
  takeItem(itemId: string, quantity: number): Promise<void>;

  /**
   * Give a Pokémon to the player.
   *
   * @param species - Species identifier string (e.g. `"BULBASAUR"`).
   * @param level   - Level of the Pokémon (1-100).
   */
  givePokemon(species: string, level: number): Promise<void>;

  // -----------------------------------------------------------------------
  // Event Flags
  // -----------------------------------------------------------------------

  /**
   * Check a flag's value synchronously.
   *
   * @param flag - Flag name (e.g. `"GOT_STARTER"`, `"BEAT_BROCK"`).
   * @returns `true` if the flag is set, `false` otherwise.
   */
  getFlag(flag: string): boolean;

  /**
   * Set a flag to `true` synchronously.
   *
   * @param flag - Flag name to set.
   */
  setFlag(flag: string): void;

  /**
   * Reset a flag to `false` synchronously.
   *
   * @param flag - Flag name to reset.
   */
  resetFlag(flag: string): void;

  // -----------------------------------------------------------------------
  // Object Visibility
  // -----------------------------------------------------------------------

  /**
   * Make a hidden map object (NPC / item ball) visible.
   *
   * @param objectIndex - 0-based object index within the current map.
   */
  showObject(objectIndex: number): Promise<void>;

  /**
   * Hide a visible map object.
   *
   * @param objectIndex - 0-based object index within the current map.
   */
  hideObject(objectIndex: number): Promise<void>;

  // -----------------------------------------------------------------------
  // NPC & Player Movement
  // -----------------------------------------------------------------------

  /**
   * Move an NPC along a path of waypoints.  Resolves when movement is done.
   *
   * @param npcId - NPC identifier string (e.g. `"OAK"`, `"MOM"`).
   * @param path  - Array of `[x, y]` coordinate pairs forming the path.
   */
  moveNpc(npcId: string, path: [number, number][]): Promise<void>;

  /**
   * Turn an NPC to face a direction.
   *
   * @param npcId     - NPC identifier string.
   * @param direction - Direction to face.
   */
  faceNpc(npcId: string, direction: Direction): Promise<void>;

  /**
   * Turn the player character to face a direction.
   *
   * @param direction - Direction to face.
   */
  facePlayer(direction: Direction): Promise<void>;

  // -----------------------------------------------------------------------
  // Audio
  // -----------------------------------------------------------------------

  /**
   * Start playing a background music track.
   *
   * @param musicId - Music identifier string (e.g. `"PALLET_TOWN"`, `"BATTLE_TRAINER"`).
   */
  playMusic(musicId: string): Promise<void>;

  /**
   * Play a one-shot sound effect.
   *
   * @param soundId - Sound effect identifier string (e.g. `"GET_ITEM"`, `"SAVE"`).
   */
  playSound(soundId: string): Promise<void>;

  /** Stop the currently playing music immediately. */
  stopMusic(): Promise<void>;

  /** Fade out the currently playing music over several frames. */
  fadeOutMusic(): Promise<void>;

  // -----------------------------------------------------------------------
  // Battle
  // -----------------------------------------------------------------------

  /**
   * Start a trainer battle.  Resolves when the battle is finished.
   *
   * @param trainerId - Trainer identifier string (e.g. `"RIVAL_1"`, `"BROCK"`).
   * @returns A result string indicating the battle outcome (e.g. `"win"`, `"lose"`).
   */
  startBattle(trainerId: string): Promise<string>;

  // -----------------------------------------------------------------------
  // Timing & Transitions
  // -----------------------------------------------------------------------

  /**
   * Pause script execution for a number of frames (1 frame ≈ 16.7 ms at 60 fps).
   *
   * @param frames - Number of frames to wait.
   */
  delay(frames: number): Promise<void>;

  /**
   * Warp the player to a different map location.
   *
   * @param map - Map identifier string (e.g. `"PalletTown"`, `"OaksLab"`).
   * @param x   - Destination tile X coordinate.
   * @param y   - Destination tile Y coordinate.
   */
  warpTo(map: string, x: number, y: number): Promise<void>;

  /**
   * Perform a screen fade (in or out).
   *
   * @param fadeType - `"in"` to fade from black, `"out"` to fade to black.
   */
  fadeScreen(fadeType: FadeType): Promise<void>;

  // -----------------------------------------------------------------------
  // Healing
  // -----------------------------------------------------------------------

  /** Fully heal all Pokémon in the player's party. */
  heal(): Promise<void>;

  // -----------------------------------------------------------------------
  // Map Script State
  // -----------------------------------------------------------------------

  /**
   * Set the current map's active script index.  This controls which
   * `onMapScript_N` function is called each frame.
   *
   * @param scriptIndex - 0-based index of the script state to activate.
   */
  setMapScript(scriptIndex: number): Promise<void>;

  // -----------------------------------------------------------------------
  // Input Control
  // -----------------------------------------------------------------------

  /**
   * Ignore joypad buttons matching the given bitmask.  Use this to
   * prevent the player from moving during cutscenes.
   *
   * @param mask - Bitmask of buttons to ignore (0xFF = ignore all).
   */
  setJoyIgnore(mask: number): Promise<void>;

  /** Clear the joypad ignore mask, restoring normal input. */
  clearJoyIgnore(): Promise<void>;
}

// ---------------------------------------------------------------------------
// Map script callback signatures
//
// Each map JS file exports these top-level async functions.  They are
// called by the engine at the appropriate time.
// ---------------------------------------------------------------------------

/**
 * Called each frame while the map script state equals N.
 * Advance the state with `await game.setMapScript(N+1)`.
 */
declare function onMapScript_0(): Promise<void>;
declare function onMapScript_1(): Promise<void>;
declare function onMapScript_2(): Promise<void>;
// ... additional onMapScript_N as needed

/**
 * Called when the player presses A on an NPC with the given text ID.
 *
 * @param textId - 0-based text ID assigned to the NPC in map data.
 */
declare function onTalkNpc(textId: number): Promise<void>;

/**
 * Called when the player presses A on a sign tile with the given text ID.
 *
 * @param textId - 0-based text ID assigned to the sign in map data.
 */
declare function onTalkSign(textId: number): Promise<void>;

/**
 * Called when the player steps on a coord-event tile.
 *
 * @param x - Tile X coordinate.
 * @param y - Tile Y coordinate.
 */
declare function onCoordEvent(x: number, y: number): Promise<void>;
