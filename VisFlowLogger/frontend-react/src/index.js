import React from 'react';
import ReactDOM from 'react-dom/client';
import LandingPage from "./pages/LandingPage";
import './index.css';
import OperationsView from "./pages/OperationsList";

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <OperationsView />
  </React.StrictMode>
);

