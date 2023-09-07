import React from 'react';
import './App.css';
import './Styles/GlobalStyle.css'

import Header from "./components/Header";
import AboutMe from "./components/AboutMe";

import ApplicationSwitcher from "./components/ApplicationSwitcher";

function App() {
  return (
    <div className="App">
        <script src="https://kit.fontawesome.com/dd6b78a06a.js" crossOrigin="anonymous"></script>
        <Header></Header>
        <AboutMe/>
        <ApplicationSwitcher></ApplicationSwitcher>
    </div>
  );
}

export default App;
