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
import Accordion from '@material-ui/core/Accordion';
import AccordionSummary from '@material-ui/core/AccordionSummary';
import AccordionDetails from '@material-ui/core/AccordionDetails';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';

import Paper from '@material-ui/core/Paper';
import Tabs from '@material-ui/core/Tabs';
import Tab from '@material-ui/core/Tab';

import { createMuiTheme, makeStyles, ThemeProvider } from '@material-ui/core/styles';

import { GetBrush, RenderBrushGraph, ResetOutliner, ResetPaintbrush, ResetSmudger, ResetColorer, SetSize, SetOutlinerHeight} from './brush.js';
import {StartCapture, ResetCapture, SetBlobCanvasThreshBase, SetBlobCanvasThreshTVar, SetBlobCanvasThreshTMult, Undo, ClearCanvas, FlipCanvas} from "./paint.js";
import {GetPaletteName, NextPalette, PrevPalette} from './palette.js';

const theme = createMuiTheme({
  overrides: {
  },
  /*
  palette: {
    primary: {
      main: "#FFFF88",
    },
    secondary: {
      main: "#FFAA88",
    }
  }
  */
});

theme.typography.h3 = {
  'font-family': "monospace",
};
theme.typography.h4 = {
  'font-family': "monospace",
};
theme.typography.h5 = {
  'font-family': "monospace",
};
theme.typography.h6 = {
  'font-family': "monospace",
};

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

let brushTab = 0;
export function GetBrushTab() {
  return brushTab;
}

function BrushTabs() {
  const [value, setValue] = React.useState(0);
  
  const handleChange = (event, newValue) => {
    setValue(newValue);
    brushTab = newValue;
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
      <Tab label="Draw" />
      <Tab label="Export" />
    </Tabs>
      <PaintBrush value={value} index={0}/>
      <ExportUI value={value} index={1}/>
    </Paper>
  );
}

const threshBaseDefault = 50;
const threshVarianceDefault = 40;
const threshSpeedDefault = 30;
SetBlobCanvasThreshBase(threshBaseDefault / 100);
SetBlobCanvasThreshTVar(threshVarianceDefault / 1000);
SetBlobCanvasThreshTMult(threshSpeedDefault / (10 * 1000 * 1000));

function PaintBrush(props) {
    const value = props.value;
    const index = props.index;
    const classes = useStyles();
    const [color, setColorReactState] = React.useState("primary");
    const [tool, setToolState] = React.useState("paintbrush");
    let setTool = (tool) =>  {
      setToolState(tool);
      switch(tool) {
        case "paintbrush":
          return ResetPaintbrush();
        case "outliner":
          return ResetOutliner();
        case "smudger":
          return ResetSmudger();
        case "colorer":
          return ResetColorer();
      }
    }
    
    return (
        <div className={classes.root} hidden={value !== index}>
        <RadioGroup aria-label="gender" name="gender1" value={tool} onChange={setTool}>
        <Accordion>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}
          aria-controls="panel1a-content"
          id="panel1a-header"
        >
          <FormControlLabel
            value="paintbrush"
            aria-label="Paintbrush"
            onClick={(event) => {event.stopPropagation(); setTool("paintbrush")}}
            onFocus={(event) => {event.stopPropagation(); setTool("paintbrush")}}
            control={<Radio />}
            label="BlobBrush"
          />
        </AccordionSummary>
        <AccordionDetails>
          <div>
          <Typography>Size</Typography>
          <Slider
              defaultValue={38}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {SetSize("paintbrush", val)}}
              min={8}
              max={200}
          />
          <Typography variant="h4">Shape</Typography>
          <Typography>Curve</Typography>
          <Slider
              defaultValue={12}
              //getAriaValueText={valuetext}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {GetBrush().set_curve(val / 10); renderBrushConfig()}}
              min={8}
              max={32}
          />
          <Typography>Strength</Typography>
          <Slider
              defaultValue={40}
              //getAriaValueText={valuetext}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {GetBrush().set_mult(val / 100); renderBrushConfig()}}
              min={4}
              max={64}
          />
          <canvas id="brushcurve">
          </canvas>
          <FormControl component="fieldset">
          <FormLabel component="legend" color={color}>Colour</FormLabel>
          <RadioGroup row aria-label="position" name="position" defaultValue={color} onChange={(e, val) => {
              GetBrush("paintbrush").set_color(val == "primary" ? 0.0 : 1.0);
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
        </AccordionDetails>
        </Accordion>
        <Accordion>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}
          aria-controls="panel1a-content"
          id="panel1a-header"
        >
          <FormControlLabel
            value="outliner"
            aria-label="Outliner"
            onClick={(event) => {event.stopPropagation(); setTool("outliner")}}
            onFocus={(event) => {event.stopPropagation(); setTool("outliner")}}
            control={<Radio />}
            label="Pen"
          />
        </AccordionSummary>
        <AccordionDetails>
          <Typography>Size</Typography>
          <Slider
              defaultValue={8}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {SetSize("outliner", val)}}
              min={2}
              max={64}
          />
        </AccordionDetails>
        </Accordion>
        <Accordion>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}
          aria-controls="panel1a-content"
          id="panel1a-header"
        >
          <FormControlLabel
            value="colorer"
            aria-label="Colorer"
            onClick={(event) => {event.stopPropagation(); setTool("colorer")}}
            onFocus={(event) => {event.stopPropagation(); setTool("colorer")}}
            control={<Radio />}
            label="Colourer"
          />
        </AccordionSummary>
        <AccordionDetails>
          <div className={classes.root}>
          <Typography>Size</Typography>
          <Slider
              defaultValue={32}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {SetSize("colorer", val)}}
              min={4}
              max={80}
          />
          <Typography>Colour</Typography>
          <Slider
              defaultValue={50}
              valueLabelDisplay="auto"
              onChange={ (e, val) => {GetBrush("colorer").set_color(val / 100)}}
              min={0}
              max={100}
          />
          </div>
        </AccordionDetails>
        </Accordion>
        <Accordion>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}
          aria-controls="panel1a-content"
          id="panel1a-header"
        >
          <FormControlLabel
            value="smudger"
            aria-label="Smudger"
            onClick={(event) => {event.stopPropagation(); setTool("smudger")}}
            onFocus={(event) => {event.stopPropagation(); setTool("smudger")}}
            control={<Radio />}
            label="Smudger"
          />
        </AccordionSummary>
        <AccordionDetails>
          <Typography>Size</Typography>
          <Slider
              defaultValue={64}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {SetSize("smudger", val)}}
              min={24}
              max={140}
          />
        </AccordionDetails>
        </Accordion>
        </RadioGroup>
        <Accordion>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}
          aria-controls="panel1a-content"
          id="panel1a-header"
        >
          <Typography>
            Blobbiness
          </Typography>
        </AccordionSummary>
        <AccordionDetails>
          <div className={classes.root}>
          <Typography variant="h4">Threshold</Typography>
          <br/>
          <Typography>Baseline</Typography>
          <Slider
              defaultValue={50}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {SetBlobCanvasThreshBase(val / 100); SetOutlinerHeight(val / 100)}}
              min={1}
              max={100}
          />
          <Typography>Variance</Typography>
          <Slider
              defaultValue={40}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {SetBlobCanvasThreshTVar(val / 1000)}}
              min={1}
              max={120}
          />
          <Typography>Speed</Typography>
          <Slider
              defaultValue={40}
              aria-labelledby="continuous-slider"
              valueLabelDisplay="auto"
              onChange={ (e, val) => {SetBlobCanvasThreshTMult(val / (10 * 1000 * 1000))}}
              min={1}
              max={100}
          />
          </div>
        </AccordionDetails>
        </Accordion>
        <PaletteUI />
        <ResetUndoUI />
        <Typography>
        Source code on <a href="https://github.com/danslocombe/blobpaint/">github</a>.
        </Typography>
        <Typography>
        Blobpaint Gallery <a href="https://danslocom.be/blobpaint-alpha/gallery">here</a>!
        </Typography>
        <Typography>
        Dan Slocombe 2021
        </Typography>
        </div>
    );
}

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
    <Typography>
      <u>
    Export GIF
</u>
    </Typography>
    <Typography>
    Frame count set based on threshold animation speed. Exports one full cycle.
    </Typography>
    <div className={classSpacing.root}>
    <Button key="0" variant="contained" color="primary" onClick={(evt) => {
        if (downloadLink.length > 0) {
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

function PaletteUI() {
  const [paletteName, setPaletteName] = React.useState(GetPaletteName())
 // const classes = useStyles();
  const classSpacing = useStylesSpacing();
  return (
    <Accordion>
    <AccordionSummary
      expandIcon={<ExpandMoreIcon />}
      aria-controls="panel1a-content"
      id="panel1a-header"
    >
      <Typography>
        Colour Palette
      </Typography>
    </AccordionSummary>
    <AccordionDetails>
    <div className={classSpacing.root}>
    <Typography>{paletteName}</Typography>
    <Button key="0" variant="outlined" color="primary" onClick={(evt) => {PrevPalette(); setPaletteName(GetPaletteName)}}> Prev </Button>
    <Button key="1" variant="outlined" color="secondary" onClick={(evt) => {NextPalette(); setPaletteName(GetPaletteName)}}> Next </Button>
    </div>
      </AccordionDetails>
      </Accordion>
  );
}

function ResetUndoUI() {
  const classes = useStyles();
  const classSpacing = useStylesSpacing();
  return (
    <div className={classes.root}>
    <div className={classSpacing.root}>
    <Button key="0" variant="outlined" color="primary" onClick={(evt) => {FlipCanvas()}}> FLIP </Button>
    <Button key="1" variant="outlined" color="primary" onClick={(evt) => {Undo()}}> Undo </Button>
    <Button key="2" variant="outlined" color="secondary" onClick={(evt) => {ClearCanvas()}}> Clear </Button>
    </div>
    </div>
  );
}

ReactDOM.render(<UI />, document.querySelector('#uiroot'));
