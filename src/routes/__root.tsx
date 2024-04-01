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
  }, []);

  return (
    <>
      <Outlet />
      <TanStackRouterDevtools />
    </>
  )
}

