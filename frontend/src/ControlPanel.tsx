import React, { useEffect } from 'react';
import './ControlPanel.css';

interface ControlPanelProps {
    toggleExecution: () => void;
    onRestart: () => void;
    isRunning: boolean;
}

const ControlPanel: React.FC<ControlPanelProps> = ({toggleExecution, onRestart, isRunning}) => {

    const label = isRunning ? 'Pause' : 'Run';

    useEffect(() => {
        const handleKeyPress = (e: KeyboardEvent) => {
            if (e.key === ' ') { // Space key
                toggleExecution();
            } else if (e.key === 'r') {
                onRestart();
            }
        };

        window.addEventListener('keydown', handleKeyPress);

        // Cleanup the event listener on component unmount
        return () => {
            window.removeEventListener('keydown', handleKeyPress);
        };
    }, [toggleExecution, onRestart]);
    return (
        <div className="control-panel">
            <ControlButton className={isRunning ? 'blinking' : ''} label={label + ' (Space)'} onClick={toggleExecution} />
            {/* <ControlButton label={'Step (Step)'} onClick={() => { }} /> */}
            <ControlButton label={'Restart (R)'} onClick={onRestart} />
        </div>
    );
}

export default ControlPanel;

interface ControlButtonProps {
    label: string;
    onClick: () => void;
    className?: string;
}

const ControlButton: React.FC<ControlButtonProps> = ({ label, onClick, className }) => {
    return (
        <button className={`control-button ${className}`} onClick={onClick}>
            {label}
        </button>
    );
}
