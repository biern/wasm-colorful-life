import * as R from "ramda";
import ReactDOM from "react-dom";
import React, { useEffect, useRef, MutableRefObject, useState } from "react";

import type { Game } from "../pkg/index";

const wasm = import("../pkg/index.js");

type RefMap = { [key: string]: MutableRefObject<HTMLTableCellElement | null> };

const useLife = (args: {
  size: number;
  fps: number;
  eventHandler: (refs: RefMap) => (event: Event) => void;
}) => {
  const updatesPerRender = 1;
  const [fps, setFps] = useState(3);
  const [game, setGame] = useState<undefined | Game>(undefined);

  const refs: RefMap = {};

  for (const i of R.range(0, args.size)) {
    for (const j of R.range(0, args.size)) {
      refs[`${i}-${j}`] = useRef(null);
    }
  }

  const eventHandler = args.eventHandler(refs);
  const handleEvents = (events: Event[]) => events.forEach(eventHandler);

  useEffect(() => {
    const run = async () => {
      const _mod = await wasm;
      const mod = _mod as Exclude<typeof _mod, void>;

      const game = mod.Game.new(args.size);

      handleEvents(JSON.parse(game.get_state()));

      setGame(game);
    };

    run();
  }, []);

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
  }, [game, fps]);

  return {
    fps,
    setFps,
    game,
    refs,
  };
};

export const App = () => {
  const size = 50;
  const life = useLife({ size, fps: 3, eventHandler: updateCell });

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
      <table style={{ borderCollapse: "collapse" }}>
        <tbody>
          {R.range(0, size).map((i) => (
            <tr key={i}>
              {R.range(0, size).map((j) => (
                <Cell key={`${i}-${j}`} cellRef={life.refs[`${i}-${j}`]} />
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

type CellProps = {
  cellRef: MutableRefObject<HTMLTableCellElement | null>;
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
      ref={cellRef}
    ></td>
  );
};

type CellData = {
  data: [number, number, number];
  coords: [number, number];
};

type Event = { Died: CellData } | { Born: CellData };

const updateCell = (refs: RefMap) => (e: Event) => {
  const data = "Died" in e ? e.Died : e.Born;
  const element = refs[`${data.coords[0]}-${data.coords[1]}`]?.current;

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
