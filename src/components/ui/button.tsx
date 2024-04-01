import { cn } from '@/lib/utils';
import { Slot } from '@radix-ui/react-slot';
import type { VariantProps } from 'class-variance-authority';
import { cva } from 'class-variance-authority';
import * as React from 'react';

const buttonVariants = cva(
  [
    'group inline-flex items-center justify-center text-sm no-underline font-medium transition-colors',
    "border border-transparent",
    'disabled:border-disabled data-[disabled=true]:border-disabled disabled:hover:border-disabled data-[disabled=true]:hover:border-disabled',
    'disabled:cursor-not-allowed data-[disabled=true]:cursor-not-allowed',
    'disabled:pointer-events-none data-[disabled=true]:pointer-events-none',
    'disabled:bg-weak data-[disabled=true]:bg-weak',
    'disabled:text-disabled data-[disabled=true]:text-disabled',
  ],
  {
    variants: {
      variant: {
        default: [
          "text-sub data-[status=active]:text-strong",
        ]
      },
      size: {
        lg: 'h-12 space-x-3 px-6 py-3',
        md: 'h-10 space-x-2 px-4 py-2.5',
        sm: 'h-9 space-x-1 px-2.5 py-2',
        xs: 'h-8 space-x-0.5 px-1.5 py-1.5',
        icon: 'h-10 w-10',
      },
      rounded: {
        default: 'rounded-md',
        full: 'rounded-full',
      },
      ring: {
        default: [
          'focus-ring disabled:ring-0 data-[disabled=true]:ring-disabled',
          'data-[state=open]:ring-2 data-[state=open]:ring-shadow-focus-primary data-[state=open]:ring-offset-1',
        ],
        none: '',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'md',
      rounded: 'default',
      ring: 'default',
    },
  },
);

export interface ButtonProps
  extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, 'style'>,
  VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, ring, size, rounded, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : 'button';
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, ring, rounded }), className)}
        ref={ref}
        {...props}
      />
    );
  },
);
Button.displayName = 'Button';

export { Button, buttonVariants };
