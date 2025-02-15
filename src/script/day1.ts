import init, { greet, get_calibration_value } from '../../pkg/advent_code_wasm.js';

await init();
greet('Day One');

const textInput = document.getElementById('textInput');
if (textInput) {
  textInput.addEventListener('input', (event) => {
    if (!event.target) return;
    const payload = (event.target as HTMLTextAreaElement).value;
    get_calibration_value(payload);
  });
}
