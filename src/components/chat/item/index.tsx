import type { ChatEvent } from "@/commands";
import DofusItemIcon from "@/components/dofus/dofus-item-icon";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@/components/ui/hover-card";
import { itemsManager, type Item } from "@/lib/entities/items";
import { translate } from "@/lib/entities/translations";
import { HoverCardPortal } from "@radix-ui/react-hover-card";

type ChatObject = NonNullable<ChatEvent['objects']>[number];

const ChatMessageObject = ({ object }: { object: ChatObject }) => {
  const objectId = object.objectGID;
  const item = itemsManager.getItemById(parseInt(objectId));
  if (!item) {
    return <WithDecoration>Unknown</WithDecoration>
  }

  const itemName = translate(item?.nameId)
  return <WithPopover item={item}><WithDecoration>{itemName}</WithDecoration></WithPopover>
}

const WithPopover = ({ item, children }: { item: Item, children: React.ReactNode }) => {
  return (
    <HoverCard openDelay={50} closeDelay={50}>
      <HoverCardTrigger>
        <div className="relative inline cursor-pointer">
          {children}
        </div>
      </HoverCardTrigger>
      <HoverCardPortal>
        <HoverCardContent>
          <DofusItemIcon iconId={item.iconId} />
        </HoverCardContent>
      </HoverCardPortal>
    </HoverCard>
  );
}

const WithDecoration = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="relative inline font-bold shrink-0">
      [{children}]
    </div>
  );
}


export default ChatMessageObject;
