import { createRootRoute, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { useEffect } from 'react'
import { commands } from '../commands';
import { TitleBar } from '@/components/titlebar';
import { loadTranslation } from '@/lib/entities/translations';
import { useChatStore } from '@/stores/chat.store';
import { useConfigStore } from '@/stores/config.store';
import dayjs from 'dayjs'
import locale_fr from 'dayjs/locale/fr';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime)
dayjs.locale(locale_fr)

export const Route = createRootRoute({
  component: () => <App />,
})

function App() {

  const setChatTabs = useChatStore(state => state.setTabs);
  const setConfig = useConfigStore(state => state.setConfig);

  useEffect(() => {
    document.body.classList.add('dark');
    document.body.setAttribute('data-theme', 'blue');

    Promise.all([
      loadTranslation("fr"),
      commands.listChatTabs(),
      commands.getGlobalConfig(),
    ]).then(([_, tabs, config]) => {
      commands.appReady();
      setChatTabs(tabs);
      setConfig(config);
    });

  }, []);

  return (
    <div className='flex h-screen cursor-default select-none overflow-hidden'>
      <TitleBar />
      <div className='relative flex w-full h-full overflow-hidden'>
        <Outlet />
      </div>
      <TanStackRouterDevtools position='bottom-right' />
    </div>
  )
}


