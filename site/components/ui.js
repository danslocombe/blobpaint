import React from 'react';
import ReactDOM from 'react-dom';
import Button from '@material-ui/core/Button';

import { makeStyles } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';
import Slider from '@material-ui/core/Slider';

function App() {
  return (
    <Button variant="contained" color="primary">
      Hello World
    </Button>
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

function DiscreteSlider() {
    const classes = useStyles();
  
    return (
        <div className={classes.root}>
        <Typography id="continuous-slider" gutterBottom>
            Temperature
        </Typography>
        <Slider
            defaultValue={5}
            getAriaValueText={valuetext}
            aria-labelledby="continuous-slider"
            valueLabelDisplay="auto"
            onChange={ (e, val) => {}}
            min={0}
            max={100}
        />
        </div>
    );
}

ReactDOM.render(<DiscreteSlider />, document.querySelector('#uiroot'));

let brush = null;

function GetBrush() {
  return null;
}


export { GetBrush };