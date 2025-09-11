import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { QueryClient, QueryClientProvider } from 'react-query';
import { enableMapSet } from 'immer';

import App from './App';
import { useAppStore } from './stores/appStore';
import './styles/global.css';

// Enable Immer MapSet plugin
enableMapSet();

// Debug: trace main entry
console.log('[RAIN] main.tsx loaded');

// Create React Query client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 3,
      staleTime: 5 * 60 * 1000, // 5 minutes
    },
  },
});

// Create Material-UI theme
const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#3b82f6',
    },
    secondary: {
      main: '#10b981',
    },
    background: {
      default: '#0f172a',
      paper: '#1e293b',
    },
  },
  typography: {
    fontFamily: '"JetBrains Mono", "Fira Code", "Consolas", monospace',
  },
  components: {
    MuiCssBaseline: {
      styleOverrides: {
        body: {
          scrollbarWidth: 'thin',
          '&::-webkit-scrollbar': {
            width: '8px',
          },
          '&::-webkit-scrollbar-track': {
            background: '#1e293b',
          },
          '&::-webkit-scrollbar-thumb': {
            backgroundColor: '#475569',
            borderRadius: '4px',
          },
        },
      },
    },
  },
});

function Root() {
  const { theme } = useAppStore();

  const currentTheme = React.useMemo(() => {
    return theme === 'dark' ? darkTheme : createTheme({
      palette: {
        mode: 'light',
      },
    });
  }, [theme]);

  return (
    <React.StrictMode>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider theme={currentTheme}>
          <CssBaseline />
          <BrowserRouter future={{ v7_startTransition: true, v7_relativeSplatPath: true }}>
            <App />
          </BrowserRouter>
        </ThemeProvider>
      </QueryClientProvider>
    </React.StrictMode>
  );
}

try {
  const rootEl = document.getElementById('root');
  if (!rootEl) {
    console.error('[RAIN] #root element not found');
  } else {
    console.log('[RAIN] mounting React root');
    ReactDOM.createRoot(rootEl).render(<Root />);
    console.log('[RAIN] React root mounted');
  }
} catch (e) {
  console.error('[RAIN] Failed to mount React app:', e);
}