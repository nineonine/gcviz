import React, { useEffect } from 'react';
import './Toast.css';

interface ToastProps {
    show: boolean;
    message: string;
    onClose: () => void;
}

const CLOSE_TOAST_TIME = 3000;

const Toast: React.FC<ToastProps> = ({ show, message, onClose }) => {
    useEffect(() => {
        if (show) {
            const timerId = setTimeout(() => {
                onClose();
            }, CLOSE_TOAST_TIME);

            return () => {
                clearTimeout(timerId);  // clear the timer if the component is unmounted or the message changes
            };
        }
    }, [show, onClose]);

    return (
        <div className={`toast ${show ? 'show' : ''}`}>
            {message}
            <button onClick={onClose} className="toast-close-btn">X</button>
        </div>
    );
}

export default Toast;
