import init, { emulator_start } from "./pkg/rvemu_wasm.js";

const termContainer = document.getElementById("terminal");
const term  = new Terminal({cursorBlink: true});

const kernelReader = new FileReader();
const fsImgReader = new FileReader();

const fitAddon = new FitAddon.FitAddon();
const deleteLine = "\x1b[2K\r";

// Input buffer detects user input while executing the emulator.
const inputBuffer = document.getElementById("inputBuffer");
// Output buffer detects the result of cpu state after the emulation is done.
const outputBuffer = document.getElementById("outputBuffer");
// Options for the observer (which mutations to observe)
const config = { childList: true, subtree: true };

// Create an observer instance linked to the callback function which detect
// mutations.
const outputObserver = new MutationObserver((mutationsList, observer) => {
  for(let mutation of mutationsList) {
    if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
      term.write(deleteLine);
      const firstChild = mutation.addedNodes[0];
      const texts = firstChild.innerText.split("\n");
      for (let i=0; i<texts.length; i++) {
        term.writeln(texts[i]);
      }
      outputBuffer.removeChild(firstChild);
      term.writeln("");
    }
  }
});

function loadDisk() {
  return new Promise((resolve, reject) => {
    fsImgReader.onloadend = e => {
      console.log("Loaded fs.img");
      resolve(new Uint8Array(fsImgReader.result));
    };

    // Fetch fs.img.
    fetch("./apps/fs.img")
      .then(response => response.blob())
      .then(blob => {
        const fsImgFile = new File([blob], "fs.img");
        fsImgReader.fileName = "fs.img";
        fsImgReader.readAsArrayBuffer(fsImgFile);
      });
  });
}

async function initEmulator() {
  // Initialize for wasm.
  await init();

  const fsImgData = await loadDisk();

  // Start observing the target node for configured mutations
  outputObserver.observe(outputBuffer, config);

  // Fetch kernel image.
  fetch("./apps/xv6.text")
    .then(response => response.blob())
    .then(blob => {
      const kernelFile = new File([blob], "xv6");
      kernelReader.fileName = "xv6";
      kernelReader.readAsArrayBuffer(kernelFile);
    });

  kernelReader.onloadend = e => {
    console.log("Starting to execute xv6 ...");

    const kernelData = new Uint8Array(kernelReader.result);

    try {
      emulator_start(kernelData, fsImgData);
    } catch(err) {
      console.log(err);
    }
  };
}

function initTerminal() {
  term.loadAddon(fitAddon);
  term.open(termContainer);
  fitAddon.fit();

  if (term._initialized) {
      return;
  }
  term._initialized = true;

  term.writeln("Welcome to rvemu (RISC-V online emulator)!");
  term.writeln("This is a work-in-progress project. You can see the progress at https://github.com/d0iasm/rvemu");
  term.writeln("Bug reports and feature requests are always welcome: https://github.com/d0iasm/rvemu/issues");
  term.writeln("");
  term.writeln("Loading operating system ...");

  term.onKey(e => {
    const printable = !e.domEvent.altKey && !e.domEvent.altGraphKey && !e.domEvent.ctrlKey && !e.domEvent.metaKey;

    const span = document.createElement('span');
    if (e.key == "") {
      // Control characters (enter, backspace, etc.).
      span.innerText = e.domEvent.keyCode;
    } else {
      // Normal printable characters.
      span.innerText = e.key.charCodeAt(0);
    }
    inputBuffer.appendChild(span);
  });
}

onmessage = e => {
  const c = String.fromCharCode(e.data);
  if (c == "\n") {
    term.writeln("");
  } else {
    term.write(c);
  }
}

initTerminal();
initEmulator();
