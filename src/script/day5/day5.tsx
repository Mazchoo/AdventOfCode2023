import React from "react";
import { useWasm } from "../wasm_init";
import {
  lowest_seed_location,
  lowest_seed_range_location
} from "../../../pkg/advent_code_wasm.js";

import DayCard from "../day_card.js";

export default function Day5() {
  const ready = useWasm("Day Five");
  if (!ready) return <p>Loading WebAssembly…</p>;

  return (
    <DayCard
      title="Day 5"
      description="You need to find the lowest location number for the seeds in the almanac."
      part1Fn={lowest_seed_location}
      part2Fn={lowest_seed_range_location}
    />
  );
}