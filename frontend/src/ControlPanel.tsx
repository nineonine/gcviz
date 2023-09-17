import React from 'react';
import './ControlPanel.css';

const ControlPanel: React.FC = () => {
    return (
        <div className="control-panel">
            <ControlButton label={'Run (Space)'} onClick={() => {}}/>
            <ControlButton label={'Step (S)'} onClick={() => {}}/>
            <ControlButton label={'Restart (R)'} onClick={() => {}}/>
        </div>
    );
}

export default ControlPanel;

interface ControlButtonProps {
    label: string;
    onClick: () => void;
}

const ControlButton: React.FC<ControlButtonProps> = ({label, onClick}) => {
    return (
        <button className="control-button" onClick={onClick}>
            {label}
        </button>
    );
}
