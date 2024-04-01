/* prettier-ignore-start */

/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file is auto-generated by TanStack Router

import { createFileRoute } from '@tanstack/react-router'

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as DashboardLayoutImport } from './routes/dashboard/_layout'
import { Route as FeatureschatChatImport } from './routes/features/(chat)/chat'
import { Route as DashboardLayoutSettingsImport } from './routes/dashboard/_layout/settings'
import { Route as DashboardLayoutHomeImport } from './routes/dashboard/_layout/home'
import { Route as DashboardLayoutChatImport } from './routes/dashboard/_layout/chat'

// Create Virtual Routes

const DashboardImport = createFileRoute('/dashboard')()

// Create/Update Routes

const DashboardRoute = DashboardImport.update({
  path: '/dashboard',
  getParentRoute: () => rootRoute,
} as any)

const DashboardLayoutRoute = DashboardLayoutImport.update({
  id: '/_layout',
  getParentRoute: () => DashboardRoute,
} as any)

const FeatureschatChatRoute = FeatureschatChatImport.update({
  path: '/features/chat',
  getParentRoute: () => rootRoute,
} as any)

const DashboardLayoutSettingsRoute = DashboardLayoutSettingsImport.update({
  path: '/settings',
  getParentRoute: () => DashboardLayoutRoute,
} as any)

const DashboardLayoutHomeRoute = DashboardLayoutHomeImport.update({
  path: '/home',
  getParentRoute: () => DashboardLayoutRoute,
} as any)

const DashboardLayoutChatRoute = DashboardLayoutChatImport.update({
  path: '/chat',
  getParentRoute: () => DashboardLayoutRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/dashboard': {
      preLoaderRoute: typeof DashboardImport
      parentRoute: typeof rootRoute
    }
    '/dashboard/_layout': {
      preLoaderRoute: typeof DashboardLayoutImport
      parentRoute: typeof DashboardRoute
    }
    '/dashboard/_layout/chat': {
      preLoaderRoute: typeof DashboardLayoutChatImport
      parentRoute: typeof DashboardLayoutImport
    }
    '/dashboard/_layout/home': {
      preLoaderRoute: typeof DashboardLayoutHomeImport
      parentRoute: typeof DashboardLayoutImport
    }
    '/dashboard/_layout/settings': {
      preLoaderRoute: typeof DashboardLayoutSettingsImport
      parentRoute: typeof DashboardLayoutImport
    }
    '/features/(chat)/chat': {
      preLoaderRoute: typeof FeatureschatChatImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export const routeTree = rootRoute.addChildren([
  DashboardRoute.addChildren([
    DashboardLayoutRoute.addChildren([
      DashboardLayoutChatRoute,
      DashboardLayoutHomeRoute,
      DashboardLayoutSettingsRoute,
    ]),
  ]),
  FeatureschatChatRoute,
])

/* prettier-ignore-end */
