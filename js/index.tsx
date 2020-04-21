import * as R from "ramda";
import ReactDOM from "react-dom";
import React, { useEffect, useRef, MutableRefObject, useState } from "react";
import { Canvas, useFrame, useThree } from "react-three-fiber";
import { Mesh, MeshStandardMaterial, Color } from "three";

const worker = new Worker("./worker.js");

import "reset-css";

const useLife = (args: {
  size: number;
  fps: number;
  eventHandler: (event: Event) => void;
}) => {
  const [fps, setFps] = useState(args.fps);
  const [size, setSize] = useState(args.size);

  const handleEvents = (events: Event[]) => events.forEach(args.eventHandler);

  useEffect(() => {
    const onWorkerMessage = (ev: any) => {
      const message = ev.data;
      if (message.kind === "game-events") {
        handleEvents(JSON.parse(message.events));
      }
    };

    worker.addEventListener("message", onWorkerMessage);

    () => worker.removeEventListener("message", onWorkerMessage);
  });

  useEffect(() => {
    worker.postMessage({ kind: "new-game", size });
  }, [size]);

  useEffect(() => {
    let timeoutId: number | undefined;

    const run = async () => {
      const run = () => {
        worker.postMessage({ kind: "tick" });

        timeoutId = setTimeout(run, 1000 / fps);
      };

      run();
    };

    run();

    return () => clearTimeout(timeoutId);
  }, [fps, size]);

  return {
    fps,
    setFps,
    size,
    setSize,
  };
};

type MeshNode = JSX.IntrinsicElements["mesh"];

export const App = () => {
  const mouse = useRef<[number, number]>([0, 0]);
  const nodes = useRef<{ [key: string]: MeshNode }>({});
  const setNode = (key: string) => (node: MeshNode) => {
    nodes.current[key] = node;
  };

  const life = useLife({
    size: 80,
    fps: 12,
    eventHandler: updateCell(nodes),
  });

  return (
    <div style={{ height: "100vh" }}>
      <Canvas
        camera={{ position: [0, 0, 65] }}
        onMouseMove={(e) =>
          (mouse.current = [
            e.clientX - window.innerWidth / 2,
            e.clientY - window.innerHeight / 2,
          ])
        }
      >
        <Rig mouse={mouse} />
        <ambientLight />
        <pointLight position={[0, 0, 150]} intensity={0.55} />
        <group position={[-life.size / 2, -life.size / 2, 0]}>
          {R.range(0, life.size).map((i) =>
            R.range(0, life.size).map((j) => (
              <Cell
                key={`${i}-${j}`}
                position={[i, j, 0]}
                cellRef={setNode(`${i}-${j}`)}
              />
            ))
          )}
        </group>
      </Canvas>
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

const Rig = ({
  mouse,
}: {
  mouse: React.MutableRefObject<[number, number]>;
}) => {
  const { camera } = useThree();
  useFrame(() => {
    camera.position.x += (mouse.current[0] / 50 - camera.position.x) * 0.05;
    camera.position.y += (-mouse.current[1] / 50 - camera.position.y) * 0.05;
    /* camera.lookAt(0, 0, 0); */
  });
  return null;
};

type CellProps = {
  position: [number, number, number];
  cellRef: React.Ref<MeshNode>;
};

const Cell = ({ cellRef, position }: CellProps) => {
  return (
    <mesh ref={cellRef as React.Ref<Mesh>} position={position} visible={false}>
      <boxBufferGeometry attach="geometry" args={[1, 1, 1]} />
      <meshStandardMaterial attach="material" />
    </mesh>
  );
};

type CellData = {
  data: [number, number, number];
  coords: [number, number];
};

type Event = { Died: CellData } | { Born: CellData };

const updateCell = (
  refs: MutableRefObject<{ [key in string]: JSX.IntrinsicElements["mesh"] }>
) => (e: Event) => {
  const data = "Died" in e ? e.Died : e.Born;
  const element = refs.current[`${data.coords[0]}-${data.coords[1]}`];

  if (!element) {
    return;
  }

  if ("Died" in e) {
    element.visible = false;
  } else {
    element.visible = true;
    setMeshColor(element, new Color(...data.data));
  }
};

const setMeshColor = (mesh: MeshNode, color: Color) => {
  if (!mesh.material) {
    return;
  }

  const material = Array.isArray(mesh.material)
    ? mesh.material[0]
    : mesh.material;

  (material as MeshStandardMaterial).color = color;
};

ReactDOM.render(React.createElement(App), document.getElementById("root"));
