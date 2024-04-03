import type { PropsWithChildren } from 'react';
import { TooltipProvider } from '../ui/tooltip';
import { cn } from '@/lib/utils';

export const InnerSidebarLayout = (props: PropsWithChildren<{ className?: string }>) => {
  return (
    <aside
      className={cn(
        'flex min-h-full w-64 shrink-0 grow-0 flex-col border-r-2 pt-6 px-4 bg-white',
        props.className,
      )}
    >
      <div className="no-scrollbar flex grow flex-col space-y-3 overflow-x-hidden">
        <TooltipProvider>{props.children}</TooltipProvider>
      </div>
    </aside>
  );
};
