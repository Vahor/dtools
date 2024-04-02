import { type ChatTabConfig } from '@/commands';
import { InnerSidebar } from '@/components/inner-sidebar';
import { Button } from '@/components/ui/button';
import { ButtonTooltip } from '@/components/ui/button-tooltip';
import { useChatStore } from '@/stores/chat.store';
import { Link, Outlet, createFileRoute } from '@tanstack/react-router'
import { PlusIcon } from 'lucide-react';

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat')({
  component: ChatComponent,
})

function ChatComponent() {
  return (
    <>
      <InnerSidebar>
        <ChatSidebar />
      </InnerSidebar>
      <Outlet />
    </>
  );
}


const ChatSidebar = () => {

  const channels = useChatStore(state => state.tabs);
  const channelsOrdered = Object.entries(channels).sort(([, a], [, b]) => a.order - b.order);

  return (
    <div className='flex flex-col gap-2'>
      <div className='flex justify-between items-center'>
        <p className='uppercase text-sm text-sub'>Groupes</p>
        <ButtonTooltip tooltip='Créer un groupe' side='right' size='sm' hover="default" asChild>
          <Link to='/dashboard/chat/new'>
            <PlusIcon className='size-5' />
          </Link>
        </ButtonTooltip>
      </div>
      <div className='flex flex-col gap-2'>
        {channelsOrdered.map(([id, tab]) => (
          <ChatTab key={id} tab={tab} id={id} />
        ))}
      </div>
    </div>
  )
}

const ChatTab = ({ tab, id }: { tab: ChatTabConfig, id: string }) => {
  return (
    <Button asChild size="xs" className="w-full flex justify-start items-center" hover="default" active="default">
      <Link to={`/dashboard/chat/$tab_id`} params={{ tab_id: id }}>
        {tab.name}
      </Link>
    </Button>
  )
}
