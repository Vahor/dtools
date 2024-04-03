import { basename } from 'path';
import { fontFamily } from 'tailwindcss/defaultTheme.js';


export default {
  darkMode: ['class'],
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    container: {
      center: true,
      padding: {
        DEFAULT: '1rem',
        sm: '2rem',
        lg: '4rem',
        xl: '7.5rem',
      },
      screens: {
        sm: '640px',
        md: '768px',
        lg: '1024px',
        xl: '1280px',
        '2xl': '1400px',

        tall: { raw: '(min-height: 800px)' },
      },
    },
    colors: {
      "transparent": "transparent",
      "static-black": "var(--static-static-black)",
      "static-white": "var(--static-static-white)",

      "white": "var(--static-static-white)",
      "black": "var(--static-static-black)",

      "faded-dark": "var(--state-faded-dark)",
      "faded-base": "var(--state-faded-base)",
      "faded-light": "var(--state-faded-light)",
      "faded-lighter": "var(--state-faded-lighter)",

      "information-dark": "var(--state-information-dark)",
      "information-base": "var(--state-information-base)",
      "information-light": "var(--state-information-light)",
      "information-lighter": "var(--state-information-lighter)",

      "warning-dark": "var(--state-warning-dark)",
      "warning-base": "var(--state-warning-base)",
      "warning-light": "var(--state-warning-light)",
      "warning-lighter": "var(--state-warning-lighter)",

      "error-dark": "var(--state-error-dark)",
      "error-base": "var(--state-error-base)",
      "error-light": "var(--state-error-light)",
      "error-lighter": "var(--state-error-lighter)",

      "success-dark": "var(--state-success-dark)",
      "success-base": "var(--state-success-base)",
      "success-light": "var(--state-success-light)",
      "success-lighter": "var(--state-success-lighter)",

      "away-dark": "var(--state-away-dark)",
      "away-base": "var(--state-away-base)",
      "away-light": "var(--state-away-light)",
      "away-lighter": "var(--state-away-lighter)",

      "feature-dark": "var(--state-feature-dark)",
      "feature-base": "var(--state-feature-base)",
      "feature-light": "var(--state-feature-light)",
      "feature-lighter": "var(--state-feature-lighter)",

      "verified-dark": "var(--state-verified-dark)",
      "verified-base": "var(--state-verified-base)",
      "verified-light": "var(--state-verified-light)",
      "verified-lighter": "var(--state-verified-lighter)",

      "highlighted-dark": "var(--state-highlighted-dark)",
      "highlighted-base": "var(--state-highlighted-base)",
      "highlighted-light": "var(--state-highlighted-light)",
      "highlighted-lighter": "var(--state-highlighted-lighter)",

      "stable-dark": "var(--state-stable-dark)",
      "stable-base": "var(--state-stable-base)",
      "stable-light": "var(--state-stable-light)",
      "stable-lighter": "var(--state-stable-lighter)",

      "primary-darker": "var(--primary-darker)",
      "primary-dark": "var(--primary-dark)",
      "primary-base": "var(--primary-base)",

      "primary-alpha-10": "var(--primary-alpha-10)",
      "primary-alpha-16": "var(--primary-alpha-16)",
      "primary-alpha-24": "var(--primary-alpha-24)",
    },
    extend: {
      backgroundColor: {
        "strong": "var(--bg-strong-950)",
        "surface": "var(--bg-surface-800)",
        "sub": "var(--bg-sub-300)",
        "soft": "var(--bg-soft-200)",
        "active": "var(--bg-active-100)",
        "weak": "var(--bg-weak-50)",
        "white": "var(--bg-white-0)",
      },
      textColor: {
        "strong": "var(--text-strong-950)",
        "sub": "var(--text-sub-600)",
        "soft": "var(--text-soft-400)",
        "disabled": "var(--text-disabled-300)",
        "white": "var(--text-white-0)"
      },
      borderColor: {
        "strong": "var(--stroke-strong-950)",
        "sub": "var(--stroke-sub-300)",
        "soft": "var(--stroke-soft-200)",
        "white": "var(--stroke-white-0)",
      },
      transitionProperty: {
        height: 'height',
        spacing: 'margin, padding',
      },
      borderRadius: {
        lg: 'var(--radius)',
        md: 'calc(var(--radius) - 2px)',
        sm: 'calc(var(--radius) - 4px)',
      },
      fontFamily: {
        sans: ["Inter", ...fontFamily.sans], // TODO: add inter
        mono: [...fontFamily.mono],
      },
      boxShadow: {
        strokeImportant: '0 1px 3px 0 var(--tw-shadow-stroke-important)',
        strokePrimary: '0 1px 2px 0 var(--tw-shadow-stroke-primary)',
        strokeError: '0 1px 2px 0 var(--tw-shadow-stroke-error)',
        focusImportant:
          '0 0 0 2px var(--tw-shadow-focus-important), 0 0 0 4px var(--tw-shadow-focus-important)',
        focusPrimary:
          '0 0 0 2px var(--tw-shadow-focus-primary), 0 0 0 4px var(--tw-shadow-focus-primary)',
        focusError:
          '0 0 0 2px var(--tw-shadow-focus-error), 0 0 0 4px var(--tw-shadow-focus-error)',

        outline: 'var(--tw-shadow-stroke-primary) 0 0 0 6px',
        'outline-lg': 'var(--tw-shadow-stroke-primary) 0 0 0 8px',
      },
      keyframes: {
        'accordion-down': {
          from: { height: '0' },
          to: { height: 'var(--radix-accordion-content-height)' },
        },
        'accordion-up': {
          from: { height: 'var(--radix-accordion-content-height)' },
          to: { height: '0' },
        },
        'collapsible-down': {
          from: { height: '0' },
          to: { height: 'var(--radix-collapsible-content-height)' },
        },
        'collapsible-up': {
          from: { height: 'var(--radix-collapsible-content-height)' },
          to: { height: '0' },
        },
      },
      animation: {
        'accordion-down': 'accordion-down 0.2s ease-out',
        'accordion-up': 'accordion-up 0.2s ease-out',
        'collapsible-down': 'collapsible-down 0.2s ease-out',
        'collapsible-up': 'collapsible-up 0.2s ease-out',
      },
    },
  },
  plugins: [
    require('tailwindcss-animate'),
    require('@tailwindcss/container-queries'),
  ],
};
