import React from 'react';
import './Visualization.css';

import InfoBlock from './InfoBlock';
import EventStream from './EventStream';
import HeapGrid from './HeapGrid';
import ControlPanel from './ControlPanel';
import { CellStatus } from './types';

const Visualization: React.FC = () => {
    return (
        <div className="visualization">
            <div className="top-section">
                <div className="left-panel">
                    <InfoBlock />
                    <EventStream />
                </div>
                <HeapGrid memory={new Array(1024).fill({status: CellStatus.Free})}/>
            </div>
            <ControlPanel />
        </div>
    );
}

export default Visualization;
