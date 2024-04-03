import { PropsWithChildren } from 'react';
import { TooltipProvider } from '../ui/tooltip';

export const SidebarLayout = (props: PropsWithChildren) => {
  return (
    <nav className="flex min-h-full w-[4.5rem] shrink-0 grow-0 flex-col pt-10 px-2 bg-weak">
      <div className="no-scrollbar flex grow flex-col space-y-3 overflow-x-hidden">
        <TooltipProvider>{props.children}</TooltipProvider>
      </div>
    </nav>
  );
};
