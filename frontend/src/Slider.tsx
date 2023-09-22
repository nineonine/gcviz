import React from 'react';
import './Slider.css';

interface SliderProps {
    minValue: number;
    maxValue: number;
    intervalRate: number;
    updateIntervalRate: (value: number) => void;
}

const Slider: React.FC<SliderProps> = ({ minValue, maxValue, intervalRate, updateIntervalRate }) => {

    const handleInput = (event: React.ChangeEvent<HTMLInputElement>) => {
        updateIntervalRate(Number(event.target.value));
    };

    return (
        <div className='slider-block'>
            <span id='slider-label'>Program speed</span>
            <input
                type="range"
                className="slider"
                min={minValue}
                max={maxValue}
                onInput={handleInput}
            />
            <span id='slider-value'>{intervalRate} ms/frame</span>
        </div>
    );
}

export default Slider;
