import("../pkg/index.js").catch(console.error).then((mod) => {
  const game = mod.Game.new(100);

  setInterval(() => {
    const events = game.tick();
    console.log(JSON.parse(events));
  }, 1000);
});
