import { commands } from '@/commands';
import { useChatStore } from '@/stores/chat.store';
import { createFileRoute, useRouter } from '@tanstack/react-router'

export const Route = createFileRoute('/dashboard/_layout/(chat)/chat/')({
  component: ChatIndex
})


function ChatIndex() {
  const router = useRouter();
  const active_tab = commands.getLastOpenChatTab();
  const tabs = useChatStore(state => state.tabs);

  active_tab.then((tab) => {
    const future_tab = Object.keys(tabs).find((t) => t === tab);

    if (future_tab === undefined) {
      const first_tab = Object.keys(tabs)[0];
      if (first_tab !== undefined) {
        router.navigate({ to: `/dashboard/chat/$tab_id`, params: { tab_id: first_tab } });
      } else {
        router.navigate({ to: '/dashboard/chat/new' });
      }
    }
    else {
      router.navigate({ to: `/dashboard/chat/$tab_id`, params: { tab_id: future_tab } });
    }
  });

  return null;
}
