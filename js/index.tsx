import ReactDOM from "react-dom";
import React, { useEffect, useRef, useState } from "react";

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
  const cameraRef = useRef<{ position: [number, number] }>({
    position: [0, 0],
  });

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

      timeoutId = redraw(args.canvas, cameraRef, game, size, 10);
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
    moveCamera: (delta: [number, number]) => {
      cameraRef.current.position = cameraRef.current.position.map(
        (value, i) => value + delta[i]
      ) as [number, number];
    },
  };
};

const redraw = (
  canvas: HTMLCanvasElement,
  cameraRef: React.MutableRefObject<{ position: [number, number] }>,
  game: Game,
  size: number,
  interval: number
) => {
  const context = canvas.getContext("2d")!;
  return setInterval(() => {
    window.requestAnimationFrame(() => {
      const scale = 2;
      const cellSize = Math.min(canvas.width, canvas.height) / size / scale;

      context.resetTransform();
      context.clearRect(0, 0, canvas.width, canvas.height);

      canvas.width = window.innerWidth * scale;
      canvas.height = window.innerHeight * scale;
      context.translate(
        cameraRef.current.position[0] * scale,
        cameraRef.current.position[1] * scale
      );
      context.scale(2, 2);
      game.draw(cellSize);
    });
  }, interval);
};

export const App = () => {
  const [canvas, setCanvas] = useState<HTMLCanvasElement | null>(null);

  const life = useLife({
    size: 150,
    fps: 12,
    canvas,
  });

  return (
    <div style={{ height: "100vh" }}>
      <canvas
        style={{ height: "100vh", width: "100%" }}
        ref={setCanvas}
        {...draggable(life.moveCamera)}
      />
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
            max="300"
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

const draggable = (onDrag: (delta: [number, number]) => void) => {
  const move = (e: React.MouseEvent) => {
    onDrag([e.movementX, e.movementY]);
  };

  return {
    onMouseDown: (e: React.MouseEvent) => {
      e.target.addEventListener("mousemove", move as any);
    },
    onMouseUp: (e: React.MouseEvent) => {
      e.target.removeEventListener("mousemove", move as any);
    },
  };
};

ReactDOM.render(React.createElement(App), document.getElementById("root"));
