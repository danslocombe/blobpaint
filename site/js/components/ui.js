import React from 'react';
import ReactDOM from 'react-dom';
import Button from '@material-ui/core/Button';
import Slider from '@material-ui/core/Slider';
import Radio from '@material-ui/core/Radio';
import RadioGroup from '@material-ui/core/RadioGroup';
import FormControlLabel from '@material-ui/core/FormControlLabel';
import FormControl from '@material-ui/core/FormControl';
import FormLabel from '@material-ui/core/FormLabel';
import Typography from '@material-ui/core/Typography';

import Paper from '@material-ui/core/Paper';
import Tabs from '@material-ui/core/Tabs';
import Tab from '@material-ui/core/Tab';

import { createMuiTheme, makeStyles, ThemeProvider } from '@material-ui/core/styles';

import { SetCurve, SetMult, SetColor, RenderBrushGraph, ResetOutliner, ResetPaintbrush } from './brush.js';
import {StartCapture, ResetCapture} from "./tick.js";

const theme = createMuiTheme({
  overrides: {
  },
});

theme.typography.h3 = {
  'font-family': "monospace",
}

console.log(theme);

theme.typography.fontFamily = "monospace";
theme.typography.body1['font-family'] = "monospace";

const useStyles = makeStyles({
root: {
    width: 320,
    fontSize: 18,
    //fontFamily: "monospace",
    lineHeight: "80%",
    marginBlockStart: "10px",
    marginBlockEnd: "10px",
},
});

const useStylesSpacing = makeStyles({
  root: {
    '& > *': {
      margin: theme.spacing(0.5),
    },
  }
})

function NestedUI() {
  return(
    [
      <ThemeProvider theme={theme} key="0">
      <BrushTabs key="1"/>
      </ThemeProvider>
    ]);
}

function UI() {
    const classes = useStyles();
  return (
    <div className={classes.root}>
    <NestedUI />
    </div>
  );
}

function renderBrushConfig() {
  let canvas = document.getElementById('brushcurve');
  RenderBrushGraph(canvas);
}

setTimeout(renderBrushConfig, 10);

function BrushTabs() {
  const [value, setValue] = React.useState(0);
  
  const handleChange = (event, newValue) => {
    if (newValue === 0) {
      ResetPaintbrush();
    }
    else {
      ResetOutliner();
    }
    setValue(newValue);
  };

  return (
    <Paper>
      <Tabs
      value={value}
      indicatorColor="primary"
      textColor="primary"
      onChange={handleChange}
      aria-label="tabs"
    >
      <Tab label="Create" />
      <Tab label="Export" />
    </Tabs>
      <PaintBrush value={value} index={0}/>
      <ExportUI value={value} index={1}/>
    </Paper>
  );
}

function PaintBrush(props) {
    const value = props.value;
    const index = props.index;
    const classes = useStyles();
    const [color, setColorReactState] = React.useState("primary");
    
    return (
        <div className={classes.root} hidden={value !== index}>
        <Typography variant="h3">Brush</Typography>
        <Typography>Curve</Typography>
        <Slider
            defaultValue={20}
            //getAriaValueText={valuetext}
            aria-labelledby="continuous-slider"
            valueLabelDisplay="auto"
            onChange={ (e, val) => {SetCurve(val); renderBrushConfig()}}
            min={1}
            max={40}
        />
        <Typography>Strength</Typography>
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
        <FormControl component="fieldset">
        <FormLabel component="legend" color={color}>Colour</FormLabel>
        <RadioGroup row aria-label="position" name="position" defaultValue={color} onChange={(e, val) => {
            SetColor(val == "primary" ? 0.0 : 1.0);
            setColorReactState(val);
          }
        }>
          <FormControlLabel
            value="primary"
            control={<Radio color="primary" />}
            label="Primary"
            labelplacement="start"
          />
          <FormControlLabel
            value="secondary"
            control={<Radio color="secondary" />}
            label="Secondary"
            labelplacement="start"
          />
        </RadioGroup>
      </FormControl>
        </div>
    );
}

ReactDOM.render(<UI />, document.querySelector('#uiroot'));


function ExportUI(props) {
  const value = props.value;
  const index = props.index;
  const classes = useStyles();
  const classSpacing = useStylesSpacing();
  const [enableReset, setReset] = React.useState(false);
  const [text, setText] = React.useState("Record");
  const [downloadLink, setDownloadLink] = React.useState("");
  return (
    <div className={classes.root} hidden={value !== index}>
    <Typography variant="h3">
    Export GIF
    </Typography>
    <div className={classSpacing.root}>
    <Button key="0" variant="contained" color="primary" onClick={(evt) => {
        if (downloadLink.length > 0) {
          console.log("Downloading - " + downloadLink)
          window.open(downloadLink);
        }
        else {
          StartCapture(setText, setReset, setDownloadLink)
        }
    }}>
      {text}
    </Button>
    <Button key="1" disabled={!enableReset} variant="contained" color="secondary" onClick={
        (evt) => {
          ResetCapture();
          setReset(false);
          setText("Record");
          setDownloadLink("");
        }
      }>
      Reset
    </Button>
    </div>
    </div>
  );
}