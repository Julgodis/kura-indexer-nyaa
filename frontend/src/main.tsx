import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'

import { RouterProvider, createRouter } from '@tanstack/react-router'

// Import the generated route tree
import { routeTree } from './routeTree.gen'
import { ThemeProvider } from './components/theme-provider'
import {
  QueryClient,
  QueryClientProvider,
} from '@tanstack/react-query'

// Create a new router instance
const router = createRouter({ routeTree })

export const queryClient = new QueryClient()

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <RouterProvider router={router} />
      </ThemeProvider>
    </QueryClientProvider>
  </StrictMode>,
)


export function urlTransform(url: string) {
  const baseUrl = import.meta.env.VITE_API_URL
  if (url.startsWith("/")) {
    url = baseUrl + url.slice(1)
  }
  return url
}
