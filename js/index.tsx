import * as R from "ramda";
import ReactDOM from "react-dom";
import React, { useEffect, useRef, MutableRefObject, useState } from "react";

import type { Game } from "../pkg/index";

const wasm = import("../pkg/index.js");

const useLife = (args: {
  size: number;
  fps: number;
  eventHandler: (event: Event) => void;
}) => {
  const updatesPerRender = 1;
  const [fps, setFps] = useState(3);
  const [game, setGame] = useState<undefined | Game>(undefined);
  const [size, setSize] = useState(args.size);

  const handleEvents = (events: Event[]) => events.forEach(args.eventHandler);

  useEffect(() => {
    const run = async () => {
      const _mod = await wasm;
      const mod = _mod as Exclude<typeof _mod, void>;

      const game = mod.Game.new(size);

      console.log("New game", { size });

      handleEvents(JSON.parse(game.get_state()));

      setGame(game);
    };

    run();
  }, [size]);

  useEffect(() => {
    let timeoutId: number | undefined;

    const run = async () => {
      const run = () => {
        if (!game) {
          return;
        }

        console.time("tick");
        const events = R.range(0, updatesPerRender).map(() => game.tick());
        console.timeEnd("tick");

        console.time("JSON parse");
        const decoded = R.flatten(events.map((el) => JSON.parse(el)));
        console.timeEnd("JSON parse");

        console.time("publish");
        handleEvents(decoded);
        console.timeEnd("publish");

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
  const nodes = useRef<{ [key: string]: HTMLElement }>({});
  const setNode = (key: string) => (node: HTMLElement) => {
    nodes.current[key] = node;
  };

  const life = useLife({
    size: 50,
    fps: 3,
    eventHandler: updateCell(nodes),
  });

  return (
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
      {
        <table style={{ borderCollapse: "collapse" }}>
          <tbody>
            {R.range(0, life.size).map((i) => (
              <tr key={i}>
                {R.range(0, life.size).map((j) => (
                  <Cell key={`${i}-${j}`} cellRef={setNode(`${i}-${j}`)} />
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      }
    </div>
  );
};

type CellProps = {
  cellRef: React.Ref<HTMLElement>;
};

const Cell = ({ cellRef }: CellProps) => {
  return (
    <td
      style={{
        width: "1rem",
        minWidth: "1rem",
        height: "1rem",
        minHeight: "1rem",
      }}
      ref={cellRef as React.Ref<HTMLTableCellElement>}
    ></td>
  );
};

type CellData = {
  data: [number, number, number];
  coords: [number, number];
};

type Event = { Died: CellData } | { Born: CellData };

const updateCell = (
  refs: MutableRefObject<{ [key in string]: HTMLElement | undefined }>
) => (e: Event) => {
  const data = "Died" in e ? e.Died : e.Born;
  const element = refs.current[`${data.coords[0]}-${data.coords[1]}`];

  if (!element) {
    return;
  }
  if ("Died" in e) {
    element.style.backgroundColor = "transparent";
  } else {
    element.style.backgroundColor = `rgb(${data.data.map((c) => c * 255)})`;
  }
};

ReactDOM.render(React.createElement(App), document.getElementById("root"));
