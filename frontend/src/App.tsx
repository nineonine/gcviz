import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Home from './Home';
import Visualization from './Visualization';
import Header from './Header';

function GCVizApp() {
    return (
        <Router>
            <>
                <Header />
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/visualize" element={<Visualization />} />
                    <Route path="/visualize/:program_name" element={<Visualization />} />
                </Routes>
            </>
        </Router>
    );
}

export default GCVizApp;
