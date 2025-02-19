import init, { greet, get_sum_valid_cube_configs, get_mininmum_product_each_game } from '../../pkg/advent_code_wasm.js';

await init();
greet('Welcome to Day Two');

const textInput = document.getElementById('textInput');
const partOutput1 = document.getElementById('outputPart1');
const partOutput2 = document.getElementById('outputPart2');

if (textInput) {
  textInput.addEventListener('input', (event) => {
    if (!event.target) return;
    const payload = (event.target as HTMLTextAreaElement).value;
    if (partOutput1) {
      const result = get_sum_valid_cube_configs(payload);
      partOutput1.innerHTML = `<strong>${result}</strong>`;
    }

    if (partOutput2) {
      const result = get_mininmum_product_each_game(payload);
      partOutput2.innerHTML = `<strong>${result}</strong>`;
    }
  });
}
