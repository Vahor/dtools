import { cn } from '@/lib/utils';
import * as React from 'react';

export type InputProps = React.InputHTMLAttributes<HTMLInputElement> & {
  icon?: React.ElementType;
  wrapperClassName?: string;
};

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ icon: Icon, wrapperClassName, className, type, ...props }, ref) => {
    return (
      <div className={cn('relative', wrapperClassName)}>
        {Icon && (
          <Icon className="text-sub pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 transform" />
        )}
        <input
          type={type}
          className={cn(
            'focus-ring placeholder:text-soft text-xs file:text-xs data-[invalid=true]:border-error-base flex h-10 w-full rounded-md border bg-white px-3 py-2 file:border-0 file:bg-transparent file:font-medium disabled:cursor-not-allowed disabled:opacity-50',
            Icon && 'pl-8',
            className,
          )}
          ref={ref}
          {...props}
        />
      </div>
    );
  },
);
Input.displayName = 'Input';

export { Input };
