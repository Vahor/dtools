import { Separator } from '../ui/separator';
import { SidebarApps } from './apps';
import { SidebarFooter } from './footer';
import { SidebarHeader } from './header';
import { SidebarLayout } from './layout';

export const Sidebar = () => {
  return (
    <SidebarLayout>
      <SidebarHeader />
      <SidebarApps />
      <Separator />
      <SidebarFooter />
    </SidebarLayout>
  );
};
