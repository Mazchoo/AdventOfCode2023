import init, { greet, get_calibration_value } from '../../pkg/advent_code_wasm.js';

await init();
greet('Day One');

const textInput = document.getElementById('textInput');
const part1Output = document.getElementById('outputPart1');

if (textInput) {
  textInput.addEventListener('input', (event) => {
    if (!event.target) return;
    const payload = (event.target as HTMLTextAreaElement).value;
    if (part1Output) {
      const result = get_calibration_value(payload);
      part1Output.innerHTML = `<strong>${result}</strong>`;
    }
  });
}
