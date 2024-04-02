import { ChatTabConfig, commands } from '@/commands';
import { ChatForm, type ChatFormValues } from '@/components/chat/form';
import { PageTitle } from '@/components/page-title';
import { createFileRoute, useRouter } from '@tanstack/react-router'
import { useChatStore } from '@/stores/chat.store';

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat/new')({
  component: ChatNew,
})

function ChatNew() {
  const router = useRouter();
  const addTab = useChatStore(state => state.addTab);
  const tabs = useChatStore(state => state.tabs);

  const onSubmit = async (values: ChatFormValues) => {
    const config: ChatTabConfig = {
      name: values.name,
      options: {
        notify: values.notification,
        keepHistory: values.keepHistory
      },
      order: Object.keys(tabs).length
    }

    const tabId = await commands.createChatTab(config);
    addTab(tabId, config);
    router.navigate({ to: '/dashboard/chat/$tab_id', params: { tab_id: tabId } });
  }


  return (
    <div className='flex flex-col gap-6 w-full'>
      <PageTitle title='Nouveau groupe' />
      <div className='px-6 flex justify-between items-center h-full pb-4'>
        <ChatForm onSubmit={onSubmit} />
      </div>
    </div>
  )
}
