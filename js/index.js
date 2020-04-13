import("../pkg/index.js").catch(console.error).then((mod) => {
  const game = mod.Game.new();

  setInterval(() => console.log(game.tick().map(({x, y}) => ({x, y}))), 1000);
});
