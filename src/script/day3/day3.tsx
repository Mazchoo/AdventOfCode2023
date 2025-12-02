import React from "react";
import { useWasm } from "../wasm_init";
import {
  get_sum_touching_numbers,
  get_gear_multiplication
} from "../../../pkg/advent_code_wasm.js";

import DayCard from "../day_card.js";

export default function Day3() {
  const ready = useWasm("Day Three");
  if (!ready) return <p>Loading WebAssembly…</p>;

  return (
    <DayCard
      title="Day 3"
      description="The elves left a lift unattended and you make your escape."
      part1Fn={get_sum_touching_numbers}
      part2Fn={get_gear_multiplication}
    />
  );
}