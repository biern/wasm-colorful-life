import("../pkg/index.js").catch(console.error).then((mod) => {
  const game = mod.Game.new();

  setInterval(() => {
    const events = game.tick();
    console.log(JSON.parse(events));
  }, 1000);
});
