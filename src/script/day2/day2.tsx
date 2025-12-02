import React from "react";
import { useWasm } from "../wasm_init";
import {
  get_sum_valid_cube_configs,
  get_mininmum_product_each_game
} from "../../../pkg/advent_code_wasm.js";

import DayCard from "../day_card.js";

export default function Day2() {
  const ready = useWasm("Day Two");
  if (!ready) return <p>Loading WebAssembly…</p>;

  return (
    <DayCard
      title="Day 2"
      description="The elves are playing a game with cubes, and you need to figure out which games are possible."
      part1Fn={get_sum_valid_cube_configs}
      part2Fn={get_mininmum_product_each_game}
    />
  );
}