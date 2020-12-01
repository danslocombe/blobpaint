import { GetBrush, RecordMousePos } from "./brush.js";
import {BlobCanvas} from "blobrust";
import * as wasm from "../../node_modules/blobrust/blobrust_bg.wasm";
import GIF from 'gif.js';
import {GetPalette, GetPaletteForGif} from './palette.js';
import {GetBrushTab} from './ui.js';

const w = 256;
const h = 200;
var blobCanvas = BlobCanvas.new(w, h);

let canvas = document.getElementById('canvas');
canvas.oncontextmenu = () => false;

let ctx = canvas.getContext('2d', { alpha: false });
ctx.imageSmoothingEnabled = false;

let mouseX;
let mouseY;

let painting = false;
let right_mouse_button = false;

let prev = 0;
let fps_avg = 60;
let fps_k = 20;

let t = 0;

let gifconfig = {
    gif: null,
    rendering: false,
    blob: null,
    framecount: 0,
}

export function StartCapture(progressCallback, resetCallback, downloadLinkCallback) {
  let gpalette = GetPaletteForGif();
  gifconfig.gif = new GIF({
    workers: 4,
    globalPalette: gpalette,
    quality: 4,
  });
    
  progressCallback("Capturing Frames...")

  gifconfig.gif.on('finished', function(blob) {
    gifconfig.blob = blob;
    console.log("Finished rendering - " + blob.size);
    let url = URL.createObjectURL(blob);
    gifconfig.gif = null;
    gifconfig.rendering = false;
    progressCallback("View Capture");
    resetCallback(true);
    downloadLinkCallback(url)
  });

  gifconfig.gif.on('progress', function(p) {
      let progress = "Rendering " + Math.floor(p*100) + "%...";
      console.log(progress);
      progressCallback(progress);
  })

  const tt = 1 / (1000 * 1000 * blobCanvas.get_thresh_t_mult());
  gifconfig.framecount = tt * ((2 * 3.141) * 50);
  console.log("Recording gif with frames: " + gifconfig.framecount);
}

export function ResetCapture() {
    gifconfig.gif = null;
    gifconfig.rendering = false;
    gifconfig.blob = null;
}

function updateGif() {
  if (gifconfig.gif) {
      if (gifconfig.gif.frames.length > gifconfig.framecount) {
          if (!gifconfig.rendering) {
              gifconfig.gif.render();
              gifconfig.rendering = true;
          }
      }
      else {
        const delayMs = 20;
        gifconfig.gif.addFrame(canvas, {delay: delayMs, copy: true});
      }
  }
}

export function Tick(timestep) {
    const dt_ms = timestep - prev;
    prev = timestep;

    if (painting) {
      if (mouseX > 0 && mouseX < 1 && mouseY > 0 && mouseY < 1 ) {
        let brush = GetBrush();
        if (brush) {
          if (!right_mouse_button) {
            blobCanvas.apply_brush(mouseX, mouseY, brush);
          }
          else {
            blobCanvas.remove_brush(mouseX, mouseY, brush);
          }
        }
      }
    }

    RecordMousePos(mouseX, mouseY);

    t+=1;
    let framerate = fps_avg;
    if (gifconfig.gif != null)
    {
      // Record at 50 fps
      framerate = 50;
    }

    blobCanvas.tick((1000 * 1000) / framerate);

    Draw(dt_ms);
    
    updateGif();
    window.requestAnimationFrame(Tick);
}

function Draw(dt_ms) {
    const cols = GetPalette();
    const drawBufSize = blobCanvas.get_draw_buffer_size();
    blobCanvas.fill_draw_buffer();
    const drawBufPtr = blobCanvas.get_draw_buffer();
    const drawBuf = new Uint8Array(wasm.memory.buffer, drawBufPtr, drawBufSize * 3);

    let i = 0;
    while (i < drawBufSize * 3) {
      const x = drawBuf[i++];
      const y = drawBuf[i++];
      const col = drawBuf[i++];

      ctx.fillStyle = cols[col];
      ctx.fillRect(x, y, 2, 2);
    }

    if (dt_ms > 0) {
      const fps = 1000 / dt_ms; 
      fps_avg = (fps_avg * (fps_k) + fps) / (fps_k + 1)

      if (GetBrushTab() == 0)
      {
        ctx.fillStyle = "#000000";
        ctx.fillText(Math.floor(fps_avg), 10, 10)
      }
    }
}

export function SetBlobCanvasThreshBase(x) {
  blobCanvas.set_thresh_base(x);
}
export function SetBlobCanvasThreshTVar(x) {
  blobCanvas.set_thresh_t_var(x);
}
export function SetBlobCanvasThreshTMult(x) {
  blobCanvas.set_thresh_t_mult(x);
}
export function Undo() {
  blobCanvas.try_pop_undo();
}
export function ClearCanvas() {
  blobCanvas.clear();
}

export function FlipCanvas() {
  blobCanvas.flip_hoz();
}

canvas.addEventListener('mousemove', event => {
    let rect = canvas.getBoundingClientRect();
    mouseX = (event.clientX - rect.left) / rect.width;
    mouseY = (event.clientY - rect.top) / rect.height;
});

canvas.addEventListener('mousedown', event => {
    event.preventDefault();
    blobCanvas.push_undo();
    painting = true;
    right_mouse_button = (event.buttons & 0x2) != 0;
});

window.addEventListener('mouseup', event => {
    event.preventDefault();
    painting = false;
});

window.addEventListener('keydown', event => {
  if ((event.code === "KeyZ" || event.keyCode === 90) && event.ctrlKey) {
    Undo();
  }
});


document.getElementById("uiroot").addEventListener("touchmove", (e) => {
  if (!window.paused) {
    if (e.cancelable) {
      e.preventDefault();
    }
  }
});

function setMousePosFromTouch() {
  let rect = canvas.getBoundingClientRect();
  mouseX = event.touches[0].clientX / rect.width;
  mouseY = event.touches[0].clientY / rect.height;
}

canvas.addEventListener("touchstart", event => {
  if (event.cancelable) {
    event.preventDefault();
  }

  blobCanvas.push_undo();
  painting = true;
  setMousePosFromTouch();
});

canvas.addEventListener("touchend", event => {
  if (event.cancelable) {
    event.preventDefault();
  }

  painting = false;
});

canvas.addEventListener("touchmove", event => {
  if (event.cancelable) {
    event.preventDefault();
  }
  setMousePosFromTouch();
});
