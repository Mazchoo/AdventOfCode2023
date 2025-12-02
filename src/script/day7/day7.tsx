import React from "react";
import { useWasm } from "../wasm_init";
import {
  multiply_bids_and_order,
  multiply_wild_bids_and_order
} from "../../../pkg/advent_code_wasm.js";

import DayCard from "../day_card.js";

export default function Day7() {
  const ready = useWasm("Day Seven");
  if (!ready) return <p>Loading WebAssembly…</p>;

  return (
    <DayCard
      title="Day 7"
      description="While more camel gambling is going on, you steal a camel."
      part1Fn={multiply_bids_and_order}
      part2Fn={multiply_wild_bids_and_order}
    />
  );
}

