import { Brush } from "../../node_modules/blobrust/blobrust.js"

let brushPaintbrush = Brush.new_inv(50, 4, 5);
let brushOutliner = Brush.new_outliner();
let brush = brushPaintbrush;

export function ResetPaintbrush() {
    brush = brushPaintbrush;
}

export function ResetOutliner() {
    brush = brushOutliner;
}

export function SetCurve(x) {
    brush.set_curve(x);
}

export function SetMult(x) {
    brush.set_mult(x);
}

export function SetColor(x) {
    brush.set_color(x);
}

export function GetBrush() {
    return brush;
}

export function RenderBrushGraph(canvas) {
  let ctx = canvas.getContext('2d');
  //ctx.imageSmoothingEnabled = false;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = "#882222"
  ctx.lineWidth = 3;
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