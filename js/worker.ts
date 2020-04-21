import type { Game } from "../pkg/index";

const loadMod = import("../pkg/index.js").then((_mod) => {
  return _mod as Exclude<typeof _mod, void>;
});

let game: Game | undefined = undefined;

self.addEventListener("message", async (ev) => {
  const mod = await loadMod;

  if (ev.data.kind === "new-game") {
    game = mod.Game.new(ev.data.size);

    postGameEvents(game.get_state());
  } else if (ev.data.kind === "tick") {
    if (game) {
      console.time("simulation tick");
      postGameEvents(game.tick());
      console.timeEnd("simulation tick");
    }
  } else {
    console.error("Unknown event", ev.data);
  }
});

const postGameEvents = (events: string) => {
  self.postMessage({ kind: "game-events", events });
};
