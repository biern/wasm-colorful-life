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
    const run = async () => {
      if (!args.canvas) {
        return;
      }
      const _mod = await wasm;
      const mod = _mod as Exclude<typeof _mod, void>;

      const game = mod.Game.new(size, args.canvas);

      console.log("New game", { size });

      setGame(game);

      const redrawTimer = setInterval(() => {
        window.requestAnimationFrame(() => {
          const cellSize =
            Math.min(args.canvas!.width, args.canvas!.height) / size / 2;

          args.canvas!.width = window.innerWidth * 2;
          args.canvas!.height = window.innerHeight * 2;
          args.canvas!.getContext("2d")!.scale(2, 2);
          game.draw(cellSize);
        });
      }, 10);
    };

    run();
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

export const App = () => {
  const [canvas, setCanvas] = useState<HTMLCanvasElement | null>(null);

  useLife({
    size: 200,
    fps: 12,
    canvas,
  });

  return (
    <div style={{ height: "100vh" }}>
      <canvas style={{ height: "100vh", width: "100%" }} ref={setCanvas} />
    </div>
  );
};

ReactDOM.render(React.createElement(App), document.getElementById("root"));
