import { Brush } from "../../node_modules/blobrust/blobrust.js"

let brushPaintbrush = Brush.new_inv(32, 6, 1.25);
let brushOutliner = Brush.new_outliner();
let brushSmudger = Brush.new_smudger();
let brush = brushPaintbrush;

export function SetSize(brush, size) {
  switch (brush) {
    case "paintbrush":
      brushPaintbrush.set_size(size);
      break;
    case "outliner":
      brushOutliner.set_size(size);
      break;
    case "smudger":
      brushSmudger.set_size(size);
      break;
  }
}

export function SetOutlinerSize(size) {
  brushOutliner.set_size(size);
}

export function SetOutlinerHeight(h) {
  brushOutliner.set_outliner_height(h);
}

export function ResetPaintbrush() {
    brush = brushPaintbrush;
}

export function ResetOutliner() {
    brush = brushOutliner;
}

export function ResetSmudger() {
    brush = brushSmudger;
}

export function GetBrush() {
    return brush;
}

let mousePrevSmoothX = 0;
let mousePrevSmoothY = 0;
let mouseSmoothX = 0;
let mouseSmoothY = 0;
const mouseSmoothK = 5;

function lerpk(x0, x1, k) {
  return (x0 * k + x1) / (k+1);
}

export function RecordMousePos(x, y) {
  mouseSmoothX = isNaN(mouseSmoothX) ? x : mouseSmoothX;
  mouseSmoothY = isNaN(mouseSmoothY) ? y : mouseSmoothY;

  mousePrevSmoothX = mouseSmoothX;
  mousePrevSmoothY = mouseSmoothY;

  mouseSmoothX = lerpk(mouseSmoothX, x, mouseSmoothK);
  mouseSmoothY = lerpk(mouseSmoothY, y, mouseSmoothK);

  const dMult = 200;
  let dx = -dMult * (mouseSmoothX - mousePrevSmoothX);
  let dy = -dMult * (mouseSmoothY - mousePrevSmoothY);

  brush.set_smudger_dx(dx);
  brush.set_smudger_dy(dy);
}

export function RenderBrushGraph(canvas) {
  let ctx = canvas.getContext('2d');
  //ctx.imageSmoothingEnabled = false;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = "#882222";
  ctx.strokeStyle = "#363636";
  ctx.lineWidth = 2;
  let yprev = undefined;
  const step = 5;
  for (let x = 0; x < canvas.width; x+=step) {
    let dist = x / 200;
    //let val = (1 / brushConfig.mult) / (1 + brushConfig.curve*xi);
    let val = brush.sample(dist);
    let y = canvas.height - val * 200;
    if (x > 0) {
      ctx.beginPath();
      ctx.moveTo(x-step, yprev);
      ctx.lineTo(x, y);
      ctx.stroke();
    }

    yprev = y;
  }
}