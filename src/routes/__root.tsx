import { createRootRoute, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { useEffect } from 'react'
import { commands } from '../commands';

export const Route = createRootRoute({
  component: () => <App />,
})

function App() {
  useEffect(() => {
    commands.appReady();
    // add dark mode class to document.body
    document.body.classList.add('dark');
  }, []);

  return (
    <>
      <div className="h-6 w-full" data-tauri-drag-region></div>
      <Outlet />
      <TanStackRouterDevtools />
    </>
  )
}

