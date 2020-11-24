import { GetBrush, RecordMousePos } from "./brush.js";
import {BlobCanvas} from "blobrust";
import GIF from 'gif.js';

const w = 256;
const h = 200;
var blobCanvas = BlobCanvas.new(w, h);

let canvas = document.getElementById('canvas');
canvas.oncontextmenu = () => false;

let ctx = canvas.getContext('2d', { alpha: false });
ctx.imageSmoothingEnabled = false;


const cols = 
[
    "#FFFF88",
    "#FFAA88",
    "#AA8844",
    "#000000",
]

let mouseX;
let mouseY;

let painting = false;
let right_mouse_button = false;

let t = 0;

let prev = 0;
let fps_avg = 60;
let fps_k = 20;

let gifconfig = {
    gif: null,
    rendering: false,
    blob: null,
}

export function StartCapture(progressCallback, resetCallback, downloadLinkCallback) {
  gifconfig.gif = new GIF({
    workers: 4,
    globalPalette: [0xFF, 0xFF, 0x88, 0xFF, 0xAA, 0x88, 0xAA, 0x88, 0x44, 0x00, 0x00, 0x00],
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
}

export function ResetCapture() {
    gifconfig.gif = null;
    gifconfig.rendering = false;
    gifconfig.blob = null;
}

function updateGif() {
  if (gifconfig.gif) {
      //const targetframes = 80 * 2 * (50 / fps_avg);
      const targetframes = 100;
      if (gifconfig.gif.frames.length > targetframes) {
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

    // Draw
    for (let y = 0 ; y < h-1; y++) {
      for (let x = 0 ; x < w-1; x++) {
        if (Math.random() < 0.05) {
          const sampled = blobCanvas.sample_pixel(x, y);
          ctx.fillStyle = cols[sampled];
          ctx.fillRect(x, y, 2, 2);
        }
      }
    }

    if (dt_ms > 0) {
      const fps = 1000 / dt_ms; 
      fps_avg = (fps_avg * (fps_k) + fps) / (fps_k + 1)

      ctx.fillStyle = "#000000";
      ctx.fillText(Math.floor(fps_avg), 10, 10)
    }
    
    //if (t % 2 == 0)
    {
        updateGif();
    }

    window.requestAnimationFrame(Tick);
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

canvas.addEventListener('mousemove', event => {
    let rect = canvas.getBoundingClientRect();
    mouseX = (event.clientX - rect.left) / rect.width;
    mouseY = (event.clientY - rect.top) / rect.height;
});

canvas.addEventListener('mousedown', event => {
    event.preventDefault();
    blobCanvas.push_undo();
    painting = true;
    right_mouse_button = event.buttons & 0x2;
});

window.addEventListener('mouseup', event => {
    event.preventDefault();
    painting = false;
});

window.addEventListener('keydown', event => {
  if ((event.code === "KeyZ" || event.keyCode === 90) && event.ctrlKey) {
    let _success = blobCanvas.try_pop_undo();
  }
});