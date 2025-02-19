import init, { greet, get_calibration_value, get_calibration_letters } from '../../pkg/advent_code_wasm.js';

await init();
greet('Welcome to Day One');

const textInput = document.getElementById('textInput');
const partOutput1 = document.getElementById('outputPart1');
const partOutput2 = document.getElementById('outputPart2');

if (textInput) {
  textInput.addEventListener('input', (event) => {
    if (!event.target) return;
    const payload = (event.target as HTMLTextAreaElement).value;
    if (partOutput1) {
      const result = get_calibration_value(payload);
      partOutput1.innerHTML = `<strong>${result}</strong>`;
    }

    if (partOutput2) {
      const result = get_calibration_letters(payload);
      partOutput2.innerHTML = `<strong>${result}</strong>`;
    }
  });
}
