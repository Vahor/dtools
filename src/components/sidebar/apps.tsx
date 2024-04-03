import { ScrollArea } from '@radix-ui/react-scroll-area';
import { GripIcon, MessageCircleIcon } from 'lucide-react';
import { ButtonTooltip } from '../ui/button-tooltip';
import { Link } from '@tanstack/react-router';

const links = [
  { name: 'Accueil', icon: GripIcon, path: '/dashboard/home' },
  { name: 'Chat', icon: MessageCircleIcon, path: '/dashboard/chat' },
] as const;
type Link = (typeof links)[number];

export const SidebarApps = () => {
  return (
    <ScrollArea className="flex flex-col gap-4 grow items-center">
      {links.map((link) => (
        <AppLink key={link.name} {...link} />
      ))}
    </ScrollArea>
  );
};

const AppLink = ({ name, icon, path }: Link) => {
  const Icon = icon;
  return (
    <ButtonTooltip tooltip={name} side="right" size="sm" asChild>
      <Link to={path}>
        <Icon className="size-5" fill="currentColor" />
      </Link>
    </ButtonTooltip>
  );
};
