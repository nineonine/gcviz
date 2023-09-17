import React from 'react';
import { Link } from 'react-router-dom';
import './Header.css';  // Import the CSS

const Header: React.FC = () => {
    return (
        <header className="header">
            <nav>
                <ul className="nav-list">
                    <li className="nav-item"><Link to="/">Home</Link></li>
                    <li className="nav-item"><Link to="/visualize">Visualize</Link></li>
                </ul>
            </nav>
        </header>
    );
}

export default Header;
