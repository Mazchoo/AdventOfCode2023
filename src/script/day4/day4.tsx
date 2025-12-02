import React from "react";
import { useWasm } from "../wasm_init";
import {
  sum_scratch_card_values,
  sum_scratch_194_cards_recursive
} from "../../../pkg/advent_code_wasm.js";

import DayCard from "../day_card.js";

export default function Day4() {
  const ready = useWasm("Day Four");
  if (!ready) return <p>Loading WebAssembly…</p>;

  return (
    <DayCard
      title="Day 4"
      description="You find yourself on a gondola, and the elf gives you scratch cards to pass the time."
      part1Fn={sum_scratch_card_values}
      part2Fn={sum_scratch_194_cards_recursive}
    />
  );
}