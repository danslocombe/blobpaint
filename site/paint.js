const js = import("./node_modules/blobrust/blobrust.js");
js.then(js => {
  //js.greet("WebAssembly");
  var blobCanvas = js.BlobCanvas.new(128, 128);

  let canvas = document.getElementById('canvas');
  let ctx = canvas.getContext('2d', { alpha: false });
  ctx.imageSmoothingEnabled = false;

  const h = 128;
  const w = 128;

  const cols = 
  [
    "#FFFF88",
    "#FFAA88",
    "#AA8844",
  ]

  let mouseX;
  let mouseY;

  let brush = js.Brush.new();

  let prev = 0;
  let fps_avg = 60;
  let fps_k = 20;

  function tick(timestep) {
    //prev = blobCanvas.sample_pixel(10, 10);
    //ctx.fillStyle = cols[0];
    //ctx.fillRect(10, 10, 20, 20);
    const dt_ms = timestep - prev;
    prev = timestep;

      if (mouseX > 0 && mouseX < 1 && mouseY > 0 && mouseY < 1 ) {
        blobCanvas.apply_brush(mouseX, mouseY, brush);
      }

      // Draw
      for (let i = 0; i < 500; i++)
      {
        let x = Math.floor(Math.random() * 128);
        let y = Math.floor(Math.random() * 128);
        const sampled = blobCanvas.sample_pixel(x, y);
        ctx.fillStyle = cols[sampled];
        ctx.fillRect(x, y, 2, 2);
      }
      //for (let y = 0 ; y < h-1; y++) {
      //  for (let x = 0 ; x < w-1; x++) {
      //    //if (x == 10) {
      //      if (js.rand_unit() < 0.05) {
      //    //if (Math.random() < 0.05) {
      //      const sampled = blobCanvas.sample_pixel(x, y);
      //      ctx.fillStyle = cols[sampled];
      //      ctx.fillRect(x, y, 2, 2);
      //    }
      //  }
      //}

      if (dt_ms > 0)
      {
        const fps = 1000 / dt_ms; 
        fps_avg = (fps_avg * (fps_k) + fps) / (fps_k + 1)

        ctx.fillStyle = "#000000";
        ctx.fillText(Math.floor(fps_avg), 10, 10)
        //console.log(fps)
      }

      window.requestAnimationFrame(tick);
  }

  canvas.addEventListener('mousemove', function(evt) {
    let rect = canvas.getBoundingClientRect();
    mouseX = (evt.clientX - rect.left) / rect.width;
    mouseY = (evt.clientY - rect.top) / rect.height;
  }, false);

  tick();
});