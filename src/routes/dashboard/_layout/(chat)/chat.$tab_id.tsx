import { createFileRoute } from '@tanstack/react-router'
import { useEffect, useRef, useState } from 'react';
import { useChatStore } from '@/stores/chat.store';
import { PageTitle } from '@/components/page-title';
import { ScrollArea } from '@/components/ui/scroll-area';
import { ChatEvent, ChatTabConfig, commands, events } from '@/commands';

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat/$tab_id')({
  component: ChatComponent,
  errorComponent: () => <div>Chat not found</div>,
});

function ChatComponent() {
  const { tab_id } = Route.useParams();
  const tabs = useChatStore(state => state.tabs);
  const tab = tabs[tab_id];


  const [chatMessages, setChatMessages] = useState<ChatEvent[]>([])
  const mounted = useRef(false);


  useEffect(() => {
    if (mounted.current) return;
    mounted.current = true;
    // TODO: set active tab
    commands.setActiveChatTab(tab_id);
    events.chatEvent.listen((event) => {
      setChatMessages((prev) => [...prev, event.payload])
    })
    return () => {
      commands.setActiveChatTab(null);
    }
  }, [])

  if (!tab) {
    {/* TODO: handle error */ }
    return <div>Chat not found</div>
  }

  return (
    <div className='flex flex-col gap-6 w-full'>
      <PageTitle title={tab.name} description={`${chatMessages.length} messages`}>
        <div className='flex gap-2'>
          <span>mute</span>
          <span>persistant</span>
          <span>config</span>
        </div>
      </PageTitle>
      <div className='px-6 flex justify-between h-full pb-4'>
        <ScrollArea className='flex-1'>
          <WelcomeTo tab={tab} />
          {chatMessages.map((message) => {
            const id = `${message.timestamp}-${message.sender_name}`
            return <ChatMessage key={id} message={message} />
          })}
        </ScrollArea>
      </div>
    </div>
  )
}

const WelcomeTo = ({ tab }: { tab: ChatTabConfig }) => {
  return (
    <div className='flex flex-col'>
      <h2 className='font-bold text-xl'>Bienvenue dans {tab.name} !</h2>
      <p className='text-soft text-sm'>Ce groupe est vide pour le moment. Des messages vont apparaitre ici.</p>
      <p className='text-soft text-sm'>Utilisez le mode persistant pour conserver l'historique des messages.</p>
    </div>
  )
}

const ChatMessage = ({ message }: { message: ChatEvent }) => {
  return (
    <div className='flex pt-4 border-t'>
      {message.sender_name}: {message.content}
    </div>
  );
}
