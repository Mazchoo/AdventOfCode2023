import { useState, useEffect } from "react";
import init, { greet } from "../../pkg/advent_code_wasm.js";

export function useWasm(dayName: string) {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    async function loadWasm() {
      await init();
      greet(`Welcome to ${dayName}`);
      setReady(true);
    }
    loadWasm();
  }, [dayName]);

  return ready;
}
