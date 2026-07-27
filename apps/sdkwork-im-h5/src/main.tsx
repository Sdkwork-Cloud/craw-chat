import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.tsx';
import { bootstrapImH5CapabilityIntegrations } from './bootstrap';
import '@sdkwork/i18n-pc-react/styles.css';
import '@sdkwork/ui-pc-react/styles.css';
import './index.css';

function renderImH5App(): void {
  const root = createRoot(document.getElementById('root')!);
  if (import.meta.env.DEV) {
    root.render(
      <StrictMode>
        <App />
      </StrictMode>,
    );
  } else {
    root.render(<App />);
  }
}

void bootstrapImH5CapabilityIntegrations()
  .then(() => {
    renderImH5App();
  })
  .catch((error: unknown) => {
    console.error('[sdkwork-im-h5] capability bootstrap failed', error);
    renderImH5App();
  });
