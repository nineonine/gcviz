import React, { useEffect } from 'react';
import './ControlPanel.css';

interface ControlPanelProps {
    toggleExecution: () => void;
    onRestart: () => void;
    isRunning: boolean;
    onStep: () => void;
}

const ControlPanel: React.FC<ControlPanelProps> = ({ toggleExecution, onRestart, isRunning, onStep }) => {

    const label = isRunning ? 'Pause' : 'Run';

    useEffect(() => {
        const handleKeyPress = (e: KeyboardEvent) => {
            if (e.key === ' ') { // Space key
                toggleExecution();
            } else if (e.key === 'r') {
                onRestart();
            } else if (e.key === 's') {
                onStep();
            }
        };

        window.addEventListener('keydown', handleKeyPress);

        // Cleanup the event listener on component unmount
        return () => {
            window.removeEventListener('keydown', handleKeyPress);
        };
    }, [toggleExecution, onRestart, onStep]);
    return (
        <div className="control-panel">
            <ControlButton className={isRunning ? 'blinking' : ''} label={label + ' (Space)'} onClick={toggleExecution} />
            <ControlButton label={'Step (s)'} onClick={onStep} />
            <ControlButton label={'Restart (r)'} onClick={onRestart} />
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
