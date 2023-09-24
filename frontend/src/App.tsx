import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Home from './Home';
import Visualization from './Visualization';
import Header from './Header';
import './App.css'

function App() {
    return (
        <Router>
            <div className='app'>
                <Header />

                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/visualize" element={<Visualization />} />
                    <Route path="/visualize/:program_name" element={<Visualization />} />
                </Routes>
            </div>
        </Router>
    );
}

export default App;
