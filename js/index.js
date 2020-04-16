import("../pkg/index.js").catch(console.error).then((mod) => {
  const game = mod.Game.new(100);

  const initial = game.get_state();

  console.log('initial', initial);

  setInterval(() => {
    const events = game.tick();
    console.log('events', events);
  }, 1000);
});
