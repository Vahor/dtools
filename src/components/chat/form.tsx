import { useFieldArray, useForm } from 'react-hook-form';
import z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { Form, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '../ui/form';
import { Input } from '../ui/input';
import { Switch } from '../ui/switch';
import { Button } from '../ui/button';
import { ChatTabConfig, ChatTabFilterTree, ChatTabFilterType } from '@/commands';
import {
  CopyIcon,
  CornerDownRightIcon,
  EllipsisVerticalIcon,
  PlusIcon,
  TrashIcon,
} from 'lucide-react';
import { Separator } from '../ui/separator';
import { ButtonTooltip } from '../ui/button-tooltip';
import { TooltipProvider } from '../ui/tooltip';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '../ui/dropdown-menu';
import { Select, SelectContent, SelectItem, SelectValue, SelectTrigger } from '../ui/select';
import { chatManager } from '@/lib/entities/channels';

const filterSchema = z.union([
  z.object({
    type: z.literal('channel'),
    value: z.number(),
  }),
  z.object({
    type: z.literal('player'),
    value: z.string(),
  }),
  z.object({
    type: z.literal('word'),
    value: z.string(),
  }),
  z.object({
    type: z.literal('item'),
    value: z.number(),
  }),
]);

const filterTreeLeafSchema = z.object({
  filter: filterSchema,
});

const filterTreeSchema: z.ZodType<ChatTabFilterTree> = z.union([
  // use lazy
  z.lazy(() =>
    z.object({
      leaf: filterSchema,
    }),
  ),
  z.lazy(() =>
    z.object({
      and: z.array(filterTreeSchema),
    }),
  ),
  z.lazy(() =>
    z.object({
      or: z.array(filterTreeSchema),
    }),
  ),
]);

const schema = z.object({
  name: z.string().min(3).max(20),
  notification: z.boolean(),
  keepHistory: z.boolean(),
  filters: filterTreeSchema.optional(),
});

export type ChatFormValues = z.infer<typeof schema>;

export const fromChatTabConfig = (config: ChatTabConfig): ChatFormValues => {
  return {
    name: config.name,
    notification: config.options.notify,
    keepHistory: config.options.keepHistory,
    filters: config.filters ?? { and: [] },
  };
};

export const toChatTabConfig = (values: ChatFormValues, order: number): ChatTabConfig => {
  return {
    name: values.name,
    options: {
      notify: values.notification,
      keepHistory: values.keepHistory,
    },
    filters: values.filters,
    order,
  };
};

interface ChatFormProps {
  initialValues?: ChatFormValues;
  onSubmit: (values: ChatFormValues) => Promise<void>;
  submitText: string;
}

export const ChatForm = ({ initialValues, onSubmit, submitText }: ChatFormProps) => {
  const form = useForm({
    resolver: zodResolver(schema),
    defaultValues: initialValues ?? {
      name: '',
      notification: false,
      keepHistory: false,
      filters: { and: [] },
    },
  });

  const isLoading = form.formState.isSubmitting;

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="flex flex-col gap-6 h-full">
        <div className="grid grid-cols-2 gap-6">
          <FormField
            name="name"
            disabled={isLoading}
            control={form.control}
            render={({ field }) => (
              <FormItem>
                <FormLabel>Nom du groupe</FormLabel>
                <Input
                  // TODO: placeholder
                  autoCorrect="off"
                  type="text"
                  {...field}
                />
                <FormMessage />
              </FormItem>
            )}
          />

          <div className="flex items-center gap-4 flex-col">
            <FormField
              name="notification"
              disabled={isLoading}
              control={form.control}
              render={({ field }) => (
                <FormItem className="flex items-center gap-4">
                  <div className="flex-center gap-2 flex-col">
                    <FormLabel className="text-sm">Notifications</FormLabel>
                    <FormDescription>Recevoir des notifications à chaque message</FormDescription>
                    <FormMessage />
                  </div>
                  <Switch
                    {...field}
                    onCheckedChange={(e) => field.onChange(e)}
                    defaultChecked={field.value}
                    value={field.value ? 'on' : 'off'}
                  />
                </FormItem>
              )}
            />

            <FormField
              name="keepHistory"
              disabled={isLoading}
              control={form.control}
              render={({ field }) => (
                <FormItem className="flex items-center gap-4">
                  <div className="flex-center gap-2 flex-col">
                    <FormLabel className="text-sm">Historique</FormLabel>
                    <FormDescription>Ce groupe gardera l'historique des messages</FormDescription>
                    <FormMessage />
                  </div>
                  <Switch
                    {...field}
                    onCheckedChange={(e) => field.onChange(e)}
                    defaultChecked={field.value}
                    value={field.value ? 'on' : 'off'}
                  />
                </FormItem>
              )}
            />
          </div>
        </div>

        <Separator className="my-4" />

        <TooltipProvider>
          <FilterInput form={form} disabled={isLoading} />
        </TooltipProvider>

        <div>
          <Button variant="filled-primary" type="submit" disabled={isLoading}>
            {submitText}
          </Button>
        </div>
      </form>
    </Form>
  );
};

const FilterInput = ({
  form,
  disabled,
}: {
  form: ReturnType<typeof useForm<ChatFormValues>>;
  disabled: boolean;
}) => {
  // It's too complex so I'll only have a top-level "and" and everything inside will be "or"
  const { fields, append, remove } = useFieldArray({
    control: form.control,
    name: 'filters.and',
  });

  const canAdd = fields.length < ATTRIBUTES_COLORS.length;

  const copy = (index: number) => {
    const item = fields[index];
    append(item);
  };

  return (
    <div>
      <div className="flex items-center justify-between pb-4">
        <div className="flex gap-1 flex-col">
          <h2 className="text-lg font-bold">Filtres</h2>
          <p className="text-sm text-soft">Les messages devront correspondre à tous les filtres</p>
        </div>
        <Button
          variant="filled-neutral"
          onClick={() => append({ or: [] })}
          type="button"
          disabled={!canAdd || disabled}
        >
          <PlusIcon className="h-4 w-4" />
          <span>Ajouter un filtre</span>
        </Button>
      </div>

      <div className="flex flex-col gap-4">
        {fields.map((field, index) => (
          <div key={field.id} className="flex gap-4 items-start">
            <div className="flex items-center gap-1 pt-1 shrink-0">
              <FieldAction onDelete={() => remove(index)} onCopy={() => copy(index)} />
              <AttributeIcon index={index} />
            </div>

            <FilterField index={index} control={form.control} />
          </div>
        ))}
      </div>
    </div>
  );
};

const FilterField = ({
  index,
  control,
}: {
  index: number;
  control: ReturnType<typeof useForm<ChatFormValues>>['control'];
}) => {
  const { fields, append, remove, update } = useFieldArray({
    control: control,
    name: `filters.and.${index}.or`,
  });
  const addFilter = (type: ChatTabFilterType) => {
    append({ leaf: type });
  };

  return (
    <div className="w-full">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button type="button" className="border-dashed border-sub" size="sm">
            <PlusIcon className="h-4 w-4" />
            <span>Ajouter un filtre</span>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent side="bottom" align="start">
          <DropdownMenuItem onClick={() => addFilter({ type: 'channel', value: 0 })}>
            <span>Channel</span>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => addFilter({ type: 'player', value: '' })}>
            <span>Joueur</span>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => addFilter({ type: 'word', value: '' })}>
            <span>Mot</span>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => addFilter({ type: 'item', value: 0 })}>
            <span>Item</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <div className="flex gap-2 flex-col pt-2 pl-2 w-full">
        {fields.map((value, fieldIndex) => {
          return (
            <div key={fieldIndex} className="flex items-start gap-2">
              <div className="pt-2" aria-hidden="true">
                {fieldIndex == 0 ? (
                  <CornerDownRightIcon className="h-4 w-4 text-sub" />
                ) : (
                  <span className="text-sm w-4 text-sub">ou</span>
                )}
              </div>
              <FilterFieldSingle
                index={fieldIndex}
                value={value}
                update={update}
                remove={remove}
                append={append}
              />
            </div>
          );
        })}
      </div>
    </div>
  );
};

const LangMapping = {
  channel: 'Channel',
  player: 'Joueur',
  word: 'Mot',
  item: 'Item',
};

const FilterFieldSingle = ({
  index,
  value,
  update,
  remove,
  append,
}: {
  index: number;
  value: ChatTabFilterTree;
  update: (index: number, value: ChatTabFilterTree) => void;
  remove: (index: number) => void;
  append: (value: ChatTabFilterTree) => void;
}) => {
  const leaf = 'leaf' in value ? value.leaf : null;
  if (!leaf) return null;

  const deleteFilter = () => {
    remove(index);
  };

  const copyFilter = () => {
    append(value);
  };

  const updateFilter = (newValue: ChatTabFilterType) => {
    const newTree = { leaf: newValue };
    update(index, newTree);
  };

  const Field = () => {
    if (leaf.type === 'channel') {
      return <ChannelSelect value={leaf} update={updateFilter} />;
    }
  };

  return (
    <div className="flex gap-2 justify-between w-full">
      <div className="flex gap-2 items-center">
        <Button className="border-dashed border-sub shrink-0" size="sm" type="button">
          <span>{LangMapping[leaf.type]}</span>
        </Button>
        <Button className="size-6 p-0" type="button" asChild>
          <span>=</span>
        </Button>
        <Field />
      </div>
      <FieldAction onDelete={deleteFilter} onCopy={copyFilter} side="left" />
    </div>
  );
};

interface FieldUpdateProps {
  value: ChatTabFilterType;
  update: (value: ChatTabFilterType) => void;
}

const ChannelSelect = ({ update, value }: FieldUpdateProps) => {
  const channels = chatManager.get();

  return (
    <Select
      onValueChange={(value) => update({ type: 'channel', value: parseInt(value) })}
      value={value.value.toString()}
    >
      <SelectTrigger>
        <SelectValue placeholder="Channel" />
      </SelectTrigger>
      <SelectContent>
        {channels.map((channel) => (
          <SelectItem key={channel.id} value={channel.id.toString()}>
            {channel.name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

const FieldAction = ({
  onDelete,
  onCopy,
  side = 'bottom',
}: {
  onDelete: () => void;
  onCopy: () => void;
  side?: 'left' | 'bottom';
}) => {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger>
        <ButtonTooltip tooltip="Actions" className="size-6 p-0" type="button" asChild>
          <EllipsisVerticalIcon className="h-4 w-4" />
        </ButtonTooltip>
      </DropdownMenuTrigger>
      <DropdownMenuContent side={side} align="start">
        <DropdownMenuItem onClick={onDelete}>
          <TrashIcon className="h-4 w-4" />
          <span>Supprimer</span>
        </DropdownMenuItem>
        <DropdownMenuItem onClick={onCopy}>
          <CopyIcon className="h-4 w-4" />
          <span>Copier</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

const ATTRIBUTES_COLORS = ['#D6E6FF', '#D7F9F8', '#FFFFEA', '#E5D4EF', '#FBE0E0'] as const;
const AttributeIcon = ({ index }: { index: number }) => {
  return (
    <div
      className="inline-flex size-6 items-center justify-center rounded-full"
      style={{ backgroundColor: ATTRIBUTES_COLORS[index] }}
    >
      <span className="text-sm font-medium text-disabled">{String.fromCharCode(65 + index)}</span>
    </div>
  );
};
