import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
import "@sdkwork/im-h5-commons";
import App from './App.tsx';
import './index.css';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
