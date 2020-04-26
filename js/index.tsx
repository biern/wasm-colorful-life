import ReactDOM from "react-dom";
import React, { useEffect, useRef, MutableRefObject, useState } from "react";

import type { Game } from "../pkg/index";

const wasm = import("../pkg/index.js");

import "reset-css";

const useLife = (args: {
  size: number;
  fps: number;
  canvas: HTMLCanvasElement | null;
}) => {
  const [fps, setFps] = useState(args.fps);
  const [game, setGame] = useState<undefined | Game>(undefined);
  const [size, setSize] = useState(args.size);

  useEffect(() => {
    let timeoutId: number | undefined;

    const run = async () => {
      if (!args.canvas) {
        return;
      }
      const _mod = await wasm;
      const mod = _mod as Exclude<typeof _mod, void>;

      const game = mod.Game.new(size, args.canvas);

      console.log("New game", { size });

      setGame(game);

      timeoutId = redraw(args.canvas, game, size, 10);
    };

    run();

    return () => clearInterval(timeoutId);
  }, [size, args.canvas]);

  useEffect(() => {
    let timeoutId: number | undefined;

    const run = async () => {
      const run = () => {
        if (!game) {
          return;
        }

        console.time("tick");
        game.tick();
        console.timeEnd("tick");

        timeoutId = setTimeout(run, 1000 / fps);
      };

      run();
    };

    run();

    return () => clearTimeout(timeoutId);
  }, [game, fps, size]);

  return {
    fps,
    setFps,
    game,
    size,
    setSize,
  };
};

const redraw = (
  canvas: HTMLCanvasElement,
  game: Game,
  size: number,
  interval: number
) => {
  const context = canvas.getContext("2d")!;
  return setInterval(() => {
    window.requestAnimationFrame(() => {
      const cellSize = Math.min(canvas.width, canvas.height) / size / 2;

      context.clearRect(0, 0, canvas.width, canvas.height);

      canvas.width = window.innerWidth * 2;
      canvas.height = window.innerHeight * 2;
      context.scale(2, 2);
      game.draw(cellSize);
    });
  }, interval);
};

export const App = () => {
  const [canvas, setCanvas] = useState<HTMLCanvasElement | null>(null);
  const [camera, setCamera] = useState<[number, number]>([0, 0]);

  const life = useLife({
    size: 200,
    fps: 12,
    canvas,
  });

  return (
    <div style={{ height: "100vh" }}>
      <canvas style={{ height: "100vh", width: "100%" }} ref={setCanvas} />
      <div style={{ position: "absolute", top: "1rem" }}>
        <div>
          <label>FPS:</label>
          <input
            type="range"
            min="1"
            max="33"
            value={life.fps}
            onChange={(e) => {
              life.setFps(Number(e.target.value));
            }}
          />
          ({life.fps})
        </div>
        <div>
          <label>Size:</label>
          <input
            type="range"
            min="30"
            max="200"
            step="10"
            value={life.size}
            onChange={(e) => {
              life.setSize(Number(e.target.value));
            }}
          />
          ({life.size})
        </div>
      </div>
    </div>
  );
};

ReactDOM.render(React.createElement(App), document.getElementById("root"));
