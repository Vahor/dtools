import { createRootRoute, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { useEffect } from 'react'
import { commands } from '../commands';
import { TitleBar } from '@/components/titlebar';

export const Route = createRootRoute({
  component: () => <App />,
})

function App() {
  useEffect(() => {
    commands.appReady();
    document.body.classList.add('dark');
    document.body.setAttribute('data-theme', 'blue');
  }, []);

  return (
    <div className='flex h-screen cursor-default select-none overflow-hidden rounded-md'>
      <TitleBar />
      <div className='relative flex w-full h-full overflow-hidden'>
        <Outlet />
      </div>
      <TanStackRouterDevtools position='bottom-right' />
    </div>
  )
}

