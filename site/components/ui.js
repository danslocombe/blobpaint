import React from 'react';
import ReactDOM from 'react-dom';
import Button from '@material-ui/core/Button';

import { makeStyles } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';
import Slider from '@material-ui/core/Slider';

import { SetCurve } from './brush.js';

let brushConfig = {
  "dirty": false,
  "type" : "Inv",
  "curve" : 12,
  "mult" : 5,
  "size" : 50,
};

function UI() {
  return (
    [
    <Button variant="contained" color="primary" key="0">
      Hello World
    </Button>,
      <DiscreteSlider key="1"/>
    ]
  );
}


const useStyles = makeStyles({
root: {
    width: 300,
},
});

function valuetext(value) {
return `${value}°C`;
}

function updateBrushConfig() {
  brushConfig.dirty = true;
  let canvas = document.getElementById('brushcurve');
  let ctx = canvas.getContext('2d', { alpha: false });

  
  //ctx.imageSmoothingEnabled = false;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = "#882222"
  ctx.lineWidth = 3;
  let yprev = undefined;
  const step = 5;
  for (let x = 0; x < canvas.width; x+=step) {
    let xi = x / 200;
    let val = (1 / brushConfig.mult) / (1 + brushConfig.curve*xi);
    let y = canvas.height - val * 500;
    if (x > 0) {
      ctx.beginPath();
      ctx.moveTo(x-step, yprev);
      ctx.lineTo(x, y);
      ctx.stroke();
    }

    yprev = y;
  }
  
}

function DiscreteSlider() {
    const classes = useStyles();
  
    return (
        <div className={classes.root}>
        <Typography id="continuous-slider" gutterBottom>
            Brush Curve
        </Typography>
        <Slider
            defaultValue={brushConfig.curve}
            //getAriaValueText={valuetext}
            aria-labelledby="continuous-slider"
            valueLabelDisplay="auto"
            onChange={ (e, val) => {brushConfig.curve = val; SetCurve(val); updateBrushConfig()}}
            min={1}
            max={40}
        />
        <Typography id="continuous-slider" gutterBottom>
            Brush Mult
        </Typography>
        <Slider
            defaultValue={brushConfig.curve}
            //getAriaValueText={valuetext}
            aria-labelledby="continuous-slider"
            valueLabelDisplay="auto"
            onChange={ (e, val) => {brushConfig.mult = val; ; updateBrushConfig()}}
            min={1}
            max={250}
        />
        <canvas id="brushcurve">
        </canvas>
        </div>
    );
}

ReactDOM.render(<UI />, document.querySelector('#uiroot'));

export function GetBrushConfig() {
  return brushConfig;
}