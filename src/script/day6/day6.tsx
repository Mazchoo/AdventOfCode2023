import React from "react";
import { useWasm } from "../wasm_init";
import {
  boat_race_ownage,
  boat_race_one_time
} from "../../../pkg/advent_code_wasm.js";

import DayCard from "../day_card.js";

export default function Day6() {
  const ready = useWasm("Day Six");
  if (!ready) return <p>Loading WebAssembly…</p>;

  return (
    <DayCard
      title="Day 6"
      description="You're at a boat race and need to figure out how to win."
      part1Fn={boat_race_ownage}
      part2Fn={boat_race_one_time}
    />
  );
}