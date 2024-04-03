import { readEntitiesFile } from './utils';

interface RawItem {
  id: number;
  nameId: number;
  typeId: number;
  descriptionId: number;
  iconId: number;
  level: number;
  recipeIds: number[];
  dropMonsterIds: number[];
}

export type Item = RawItem;

let items: Record<number, Item> | undefined = undefined;

const loadItems = async () => {
  const rawItems = (await readEntitiesFile('Items.json')) as { data: RawItem[] };

  items = rawItems.data.reduce((acc, item) => {
    acc[item.id] = item;
    return acc;
  }, {} as Record<number, Item>);
};

const getItemById = (id: number): Item | undefined => {
  if (items === undefined) {
    throw new Error('Items not loaded');
  }
  return items[id];
};

export const itemsManager = {
  load: loadItems,
  getItemById,
} as const;
