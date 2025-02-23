import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'

const rootDom = document.getElementById('root');

if (!rootDom) {
  throw new Error('Root element not found')
}

const root = createRoot(rootDom);

root.render(
  <StrictMode>
    <App />
  </StrictMode>,
)
