import * as R from "ramda";
import ReactDOM from "react-dom";
import React, { useEffect, useRef, MutableRefObject } from "react";

const wasm = import("../pkg/index.js");

type RefMap = { [key: string]: MutableRefObject<HTMLTableCellElement | null> };

export const App = () => {
  const size = 50;
  const updatesPerRender = 1;

  const refs: RefMap = {};

  for (const i of R.range(0, size)) {
    for (const j of R.range(0, size)) {
      refs[`${i}-${j}`] = useRef(null);
    }
  }

  useEffect(() => {
    const run = async () => {
      const _mod = await wasm;
      const mod = _mod as Exclude<typeof _mod, void>;

      const game = mod.Game.new(size);

      const eventHandler = updateCell(refs);
      const handleEvents = (events: Event[]) => events.forEach(eventHandler);

      handleEvents(JSON.parse(game.get_state()));

      setInterval(() => {
        console.time("tick");
        const events = R.range(0, updatesPerRender).map(() => game.tick());
        console.timeEnd("tick");

        console.time("JSON parse");
        const decoded = R.flatten(events.map((el) => JSON.parse(el)));
        console.timeEnd("JSON parse");

        console.time("publish");
        handleEvents(decoded);
        console.timeEnd("publish");
      }, 200);
    };

    run();
  }, []);

  return (
    <div>
      <table style={{ borderCollapse: "collapse" }}>
        <tbody>
          {R.range(0, size).map((i) => (
            <tr key={i}>
              {R.range(0, size).map((j) => (
                <Cell
                  key={`${i}-${j}`}
                  x={i}
                  y={j}
                  cellRef={refs[`${i}-${j}`]}
                />
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

type CellData = {
  color: [number, number, number];
  coords: [number, number];
};

type Event = { Died: CellData } | { Born: CellData };

type CellProps = {
  x: number;
  y: number;
  cellRef: MutableRefObject<HTMLTableCellElement | null>;
};

const Cell = ({ x, y, cellRef }: CellProps) => {
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

const updateCell = (refs: RefMap) => (e: Event) => {
  const data = "Died" in e ? e.Died : e.Born;
  const element = refs[`${data.coords[0]}-${data.coords[1]}`]?.current;

  if (!element) {
    console.log(refs, `${data.coords[0]}-${data.coords[1]}`);

    console.log("Invalid data", data.coords);
    return;
  }
  if ("Died" in e) {
    element.style.backgroundColor = "transparent";
  } else {
    element.style.backgroundColor = `rgb(${data.color.map((c) => c * 255)})`;
  }
};

ReactDOM.render(React.createElement(App), document.getElementById("root"));
