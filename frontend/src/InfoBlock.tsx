import React from 'react';
import './InfoBlock.css';
import { InfoBlockData } from './types';

const InfoBlock: React.FC<InfoBlockData> = ({ gc_type, alignment, heap_size, allocd_objects, free_memory }) => {
    return (
        <div className='info-block'>
            <InfoEntry label={'GC Type'} value={gc_type} />
            <InfoEntry label={'Alignment'} value={`${alignment}`} />
            <InfoEntry label={'Heap size'} value={`${heap_size}`} />
            <InfoEntry label={'Allocated objects'} value={`${allocd_objects}`} />
            <InfoEntry label={'Free memory'} value={`${free_memory}`} />
        </div>
    );
}

export default InfoBlock;

interface InfoEntryProps {
    label: string
    value: string
}

const InfoEntry: React.FC<InfoEntryProps> = ({ label, value }) => {
    return (
        <div className='info-entry'>
            <div className='info-entry-label'>{label}:</div>
            <div className='info-entry-value'>{value}</div>
        </div>
    );
}
