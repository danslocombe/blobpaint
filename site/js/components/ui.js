import React from 'react';
import ReactDOM from 'react-dom';
import Button from '@material-ui/core/Button';

import { createMuiTheme, makeStyles, ThemeProvider } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';
import Slider from '@material-ui/core/Slider';

import { SetCurve, SetMult, RenderBrushGraph } from './brush.js';
import { CssBaseline } from '@material-ui/core';

/*
let brushConfig = {
  "dirty": false,
  "type" : "Inv",
  "curve" : 12,
  "mult" : 5,
  "size" : 50,
};
*/

const useStyles = makeStyles({
root: {
    width: 300,
    fontSize: 18,
    fontFamily: "monospace",
    lineHeight: "80%",
    marginBlockStart: "10px",
    marginBlockEnd: "10px",
},
});

function UI() {
    const classes = useStyles();
  
  return (
    [
      <DiscreteSlider key="1"/>,
    <Button variant="contained" color="primary" key="0">
      Record
    </Button>
    ]
  );
}

function renderBrushConfig() {
  let canvas = document.getElementById('brushcurve');
  RenderBrushGraph(canvas);
}

setTimeout(renderBrushConfig, 10);

function DiscreteSlider() {
    const classes = useStyles();
  
    return (
        <div className={classes.root}>
        <div>Brush Curve</div>
        <Slider
            defaultValue={20}
            //getAriaValueText={valuetext}
            aria-labelledby="continuous-slider"
            valueLabelDisplay="auto"
            onChange={ (e, val) => {SetCurve(val); renderBrushConfig()}}
            min={1}
            max={40}
        />
        <div>Brush Mult</div>
        <Slider
            defaultValue={100}
            //getAriaValueText={valuetext}
            aria-labelledby="continuous-slider"
            valueLabelDisplay="auto"
            onChange={ (e, val) => {SetMult(val / 100); renderBrushConfig()}}
            min={1}
            max={250}
        />
        <canvas id="brushcurve">
        </canvas>
        </div>
    );
}

ReactDOM.render(<UI />, document.querySelector('#uiroot'));