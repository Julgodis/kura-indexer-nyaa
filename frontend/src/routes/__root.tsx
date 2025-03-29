import * as React from 'react'
import { Outlet, createRootRoute } from '@tanstack/react-router'
import { ThemeProvider } from '@/components/theme-provider'
import { Header } from '@/components/layout/header'
import { Footer } from '@/components/layout/footer'

export const Route = createRootRoute({
  component: RootComponent,
})

function RootComponent() {
  return (
    <React.Fragment>
      <div className="mx-auto container">
        <Header />
        <main className="container mx-auto py-6 px-4">
          <Outlet />
        </main>
        <Footer />
      </div>
    </React.Fragment>
  )
}
