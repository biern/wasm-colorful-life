import * as R from "ramda";
import ReactDOM from "react-dom";
import React, { useState, useEffect } from "react";
import { EventEmitter } from "events";
import styled from "styled-components";

const wasm = import("../pkg/index.js");

export const App = () => {
  const [emitter, setEmitter] = useState<EventEmitter | undefined>(undefined);
  const size = 30;

  useEffect(() => {
    const run = async () => {
      const publishEvents = (events: Event[]) =>
        events.forEach((e) => emitter.emit("cell", e));

      const emitter = new EventEmitter();

      setEmitter(emitter);

      const _mod = await wasm;
      const mod = _mod as Exclude<typeof _mod, void>;

      const game = mod.Game.new(size);

      publishEvents(JSON.parse(game.get_state()));

      setInterval(() => {
        console.time("tick");
        const events = game.tick();
        console.timeEnd("tick");

        console.time("decode");
        const decode = JSON.parse(events);
        console.timeEnd("decode");

        console.time("publish");
        publishEvents(decode);
        console.timeEnd("publish");
      }, 500);
    };

    run();
  }, []);

  return emitter ? (
    <table style={{ borderCollapse: "collapse" }}>
      <tbody>
        {R.range(0, size).map((i) => (
          <tr key={i}>
            {R.range(0, size).map((j) => (
              <Cell key={`${i}-${j}`} x={i} y={j} emitter={emitter} />
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  ) : (
    <div />
  );
};

type CellData = {
  color: [number, number, number];
  coords: [number, number];
};

type Event = { Died: CellData } | { Born: CellData };

type CellProps = { x: number; y: number; emitter: EventEmitter };

type CellState =
  | {
      kind: "Dead";
    }
  | { kind: "Alive"; color: [number, number, number] };

/* const CellDisplay = styled.td<{ cell: CellState }>`
 *   width: 1rem;
 *   height: 1rem;
 *   background-color: ${(props) =>
 *     props.cell.kind === "Dead"
 *       ? "none"
 *       : `rgba(${props.cell.color.map((c) => c * 255)})`};
 * `;
 *  */
const Cell = ({ x, y, emitter }: CellProps) => {
  const [state, setState] = useState<CellState>({ kind: "Dead" });

  useEffect(() => {
    const listener = (e: Event) => {
      const data = "Died" in e ? e.Died : e.Born;

      if (data.coords[0] === x && data.coords[1] === y) {
        /* setTimeout(() => setState("Died" in e ? "Died" : "Born"), 0); */
        setTimeout(
          setState(
            "Died" in e
              ? { kind: "Dead" }
              : { kind: "Alive", color: data.color }
          ),
          0
        );
      }
    };

    emitter.addListener("cell", listener);

    return () => {
      emitter.removeListener("cell", listener);
    };
  }, [emitter]);

  /* return <CellDisplay cell={state} />; */
  return (
    <td
      style={{
        width: "1rem",
        height: "1rem",
        transition: "background-color 0.5s",
        backgroundColor:
          state.kind === "Dead"
            ? "transparent"
            : `rgba(${state.color.map((c) => c * 255)})`,
      }}
    ></td>
  );
};

ReactDOM.render(React.createElement(App), document.getElementById("root"));
