import React from 'react';
import './InfoBlock.css';

const InfoBlock: React.FC = () => {
    return (
        <div className='info-block'>
            <InfoEntry label={'GC Type'} value={"MarkSweep"} />
            <InfoEntry label={'Aligntment'} value={"16"} />
            <InfoEntry label={'Heap size'} value={"1024"} />
            <InfoEntry label={'Allocated objects'} value={"10"} />
            <InfoEntry label={'Free memory'} value={"1024"} />
            <InfoEntry label={'Execution speed'} value={"1frame/1s"} />
        </div>
    );
}

export default InfoBlock;

interface InfoEntryProps {
    label: string
    value: string
}

const InfoEntry: React.FC<InfoEntryProps> = ({label, value }) => {
    return (
        <div className='info-entry'>
            <div className='info-entry-label'>{label}:</div>
            <div className='info-entry-value'>{value}</div>
        </div>
    );
}
