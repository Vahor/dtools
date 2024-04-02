/* prettier-ignore-start */

/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file is auto-generated by TanStack Router

import { createFileRoute } from '@tanstack/react-router'

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as DashboardLayoutImport } from './routes/dashboard/_layout'
import { Route as DashboardLayoutSettingsImport } from './routes/dashboard/_layout/settings'
import { Route as DashboardLayoutHomeImport } from './routes/dashboard/_layout/home'
import { Route as DashboardLayoutchatChatImport } from './routes/dashboard/_layout/(chat)/chat'
import { Route as DashboardLayoutchatChatIndexImport } from './routes/dashboard/_layout/(chat)/chat.index'
import { Route as DashboardLayoutchatChatNewImport } from './routes/dashboard/_layout/(chat)/chat.new'
import { Route as DashboardLayoutchatChatTabidImport } from './routes/dashboard/_layout/(chat)/chat.$tab_id'

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

const DashboardLayoutSettingsRoute = DashboardLayoutSettingsImport.update({
  path: '/settings',
  getParentRoute: () => DashboardLayoutRoute,
} as any)

const DashboardLayoutHomeRoute = DashboardLayoutHomeImport.update({
  path: '/home',
  getParentRoute: () => DashboardLayoutRoute,
} as any)

const DashboardLayoutchatChatRoute = DashboardLayoutchatChatImport.update({
  path: '/chat',
  getParentRoute: () => DashboardLayoutRoute,
} as any)

const DashboardLayoutchatChatIndexRoute =
  DashboardLayoutchatChatIndexImport.update({
    path: '/',
    getParentRoute: () => DashboardLayoutchatChatRoute,
  } as any)

const DashboardLayoutchatChatNewRoute = DashboardLayoutchatChatNewImport.update(
  {
    path: '/new',
    getParentRoute: () => DashboardLayoutchatChatRoute,
  } as any,
)

const DashboardLayoutchatChatTabidRoute =
  DashboardLayoutchatChatTabidImport.update({
    path: '/$tab_id',
    getParentRoute: () => DashboardLayoutchatChatRoute,
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
    '/dashboard/_layout/home': {
      preLoaderRoute: typeof DashboardLayoutHomeImport
      parentRoute: typeof DashboardLayoutImport
    }
    '/dashboard/_layout/settings': {
      preLoaderRoute: typeof DashboardLayoutSettingsImport
      parentRoute: typeof DashboardLayoutImport
    }
    '/dashboard/_layout/(chat)/chat': {
      preLoaderRoute: typeof DashboardLayoutchatChatImport
      parentRoute: typeof DashboardLayoutImport
    }
    '/dashboard/_layout/(chat)/chat/$tab_id': {
      preLoaderRoute: typeof DashboardLayoutchatChatTabidImport
      parentRoute: typeof DashboardLayoutchatChatImport
    }
    '/dashboard/_layout/(chat)/chat/new': {
      preLoaderRoute: typeof DashboardLayoutchatChatNewImport
      parentRoute: typeof DashboardLayoutchatChatImport
    }
    '/dashboard/_layout/(chat)/chat/': {
      preLoaderRoute: typeof DashboardLayoutchatChatIndexImport
      parentRoute: typeof DashboardLayoutchatChatImport
    }
  }
}

// Create and export the route tree

export const routeTree = rootRoute.addChildren([
  DashboardRoute.addChildren([
    DashboardLayoutRoute.addChildren([
      DashboardLayoutHomeRoute,
      DashboardLayoutSettingsRoute,
      DashboardLayoutchatChatRoute.addChildren([
        DashboardLayoutchatChatTabidRoute,
        DashboardLayoutchatChatNewRoute,
        DashboardLayoutchatChatIndexRoute,
      ]),
    ]),
  ]),
])

/* prettier-ignore-end */
