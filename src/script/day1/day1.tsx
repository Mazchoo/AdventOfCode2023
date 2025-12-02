import React from "react";
import { useWasm } from "../wasm_init";
import {
  get_calibration_value,
  get_calibration_letters
} from "../../../pkg/advent_code_wasm.js";

import DayCard from "../day_card.js";

export default function Day1() {
  const ready = useWasm("Day One");
  if (!ready) return <p>Loading WebAssembly…</p>;

  return (
    <DayCard
      title="Day 1"
      description="The elves have finally plucked up the courage to eliminate you by hurling you to your doom in a trebucket."
      part1Fn={get_calibration_value}
      part2Fn={get_calibration_letters}
    />
  );
}