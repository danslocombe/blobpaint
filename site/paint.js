import UIMod from "./components/ui.js";

const js = import("./node_modules/blobrust/blobrust.js");
js.then(js => {
  const w = 256;
  const h = 200;
  var blobCanvas = js.BlobCanvas.new(w, h);

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

  // Inv params
  //let brush = js.Brush.new_inv(50, 0.5, 12);

  // Sqrt params
  //let brush = js.Brush.new_sqrt(50, 8, 0.255);
  
  let painting = false;
  let right_mouse_button = false;

  let prev = 0;
  let fps_avg = 60;
  let fps_k = 20;

  function tick(timestep) {
    const dt_ms = timestep - prev;
    prev = timestep;

    blobCanvas.incr_t();

    if (painting) {
      if (mouseX > 0 && mouseX < 1 && mouseY > 0 && mouseY < 1 ) {
        //let brush = UIMod.GetBrush();
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

    // Draw
    /*
    for (let i = 0; i < 2000; i++)
    {
      let x = Math.floor(Math.random() * 128);
      let y = Math.floor(Math.random() * 128);
      const sampled = blobCanvas.sample_pixel(x, y);
      ctx.fillStyle = cols[sampled];
      ctx.fillRect(x, y, 2, 2);
    }
    */

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

    window.requestAnimationFrame(tick);
  }


  // TODO Ccapture.js
  // https://www.npmjs.com/package/ccapture.js-npmfixed#using-the-code

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

  tick();
});
