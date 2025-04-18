/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as HealthImport } from './routes/health'
import { Route as ProxyImport } from './routes/_proxy'
import { Route as IndexImport } from './routes/index'
import { Route as ProxyMirrorIndexImport } from './routes/_proxy/$mirror/index'
import { Route as ProxyMirrorViewIdImport } from './routes/_proxy/$mirror/view.$id'
import { Route as ProxyMirrorUserIdImport } from './routes/_proxy/$mirror/user.$id'

// Create/Update Routes

const HealthRoute = HealthImport.update({
  id: '/health',
  path: '/health',
  getParentRoute: () => rootRoute,
} as any)

const ProxyRoute = ProxyImport.update({
  id: '/_proxy',
  getParentRoute: () => rootRoute,
} as any)

const IndexRoute = IndexImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => rootRoute,
} as any)

const ProxyMirrorIndexRoute = ProxyMirrorIndexImport.update({
  id: '/$mirror/',
  path: '/$mirror/',
  getParentRoute: () => ProxyRoute,
} as any)

const ProxyMirrorViewIdRoute = ProxyMirrorViewIdImport.update({
  id: '/$mirror/view/$id',
  path: '/$mirror/view/$id',
  getParentRoute: () => ProxyRoute,
} as any)

const ProxyMirrorUserIdRoute = ProxyMirrorUserIdImport.update({
  id: '/$mirror/user/$id',
  path: '/$mirror/user/$id',
  getParentRoute: () => ProxyRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      id: '/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof IndexImport
      parentRoute: typeof rootRoute
    }
    '/_proxy': {
      id: '/_proxy'
      path: ''
      fullPath: ''
      preLoaderRoute: typeof ProxyImport
      parentRoute: typeof rootRoute
    }
    '/health': {
      id: '/health'
      path: '/health'
      fullPath: '/health'
      preLoaderRoute: typeof HealthImport
      parentRoute: typeof rootRoute
    }
    '/_proxy/$mirror/': {
      id: '/_proxy/$mirror/'
      path: '/$mirror'
      fullPath: '/$mirror'
      preLoaderRoute: typeof ProxyMirrorIndexImport
      parentRoute: typeof ProxyImport
    }
    '/_proxy/$mirror/user/$id': {
      id: '/_proxy/$mirror/user/$id'
      path: '/$mirror/user/$id'
      fullPath: '/$mirror/user/$id'
      preLoaderRoute: typeof ProxyMirrorUserIdImport
      parentRoute: typeof ProxyImport
    }
    '/_proxy/$mirror/view/$id': {
      id: '/_proxy/$mirror/view/$id'
      path: '/$mirror/view/$id'
      fullPath: '/$mirror/view/$id'
      preLoaderRoute: typeof ProxyMirrorViewIdImport
      parentRoute: typeof ProxyImport
    }
  }
}

// Create and export the route tree

interface ProxyRouteChildren {
  ProxyMirrorIndexRoute: typeof ProxyMirrorIndexRoute
  ProxyMirrorUserIdRoute: typeof ProxyMirrorUserIdRoute
  ProxyMirrorViewIdRoute: typeof ProxyMirrorViewIdRoute
}

const ProxyRouteChildren: ProxyRouteChildren = {
  ProxyMirrorIndexRoute: ProxyMirrorIndexRoute,
  ProxyMirrorUserIdRoute: ProxyMirrorUserIdRoute,
  ProxyMirrorViewIdRoute: ProxyMirrorViewIdRoute,
}

const ProxyRouteWithChildren = ProxyRoute._addFileChildren(ProxyRouteChildren)

export interface FileRoutesByFullPath {
  '/': typeof IndexRoute
  '': typeof ProxyRouteWithChildren
  '/health': typeof HealthRoute
  '/$mirror': typeof ProxyMirrorIndexRoute
  '/$mirror/user/$id': typeof ProxyMirrorUserIdRoute
  '/$mirror/view/$id': typeof ProxyMirrorViewIdRoute
}

export interface FileRoutesByTo {
  '/': typeof IndexRoute
  '': typeof ProxyRouteWithChildren
  '/health': typeof HealthRoute
  '/$mirror': typeof ProxyMirrorIndexRoute
  '/$mirror/user/$id': typeof ProxyMirrorUserIdRoute
  '/$mirror/view/$id': typeof ProxyMirrorViewIdRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/': typeof IndexRoute
  '/_proxy': typeof ProxyRouteWithChildren
  '/health': typeof HealthRoute
  '/_proxy/$mirror/': typeof ProxyMirrorIndexRoute
  '/_proxy/$mirror/user/$id': typeof ProxyMirrorUserIdRoute
  '/_proxy/$mirror/view/$id': typeof ProxyMirrorViewIdRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths:
    | '/'
    | ''
    | '/health'
    | '/$mirror'
    | '/$mirror/user/$id'
    | '/$mirror/view/$id'
  fileRoutesByTo: FileRoutesByTo
  to:
    | '/'
    | ''
    | '/health'
    | '/$mirror'
    | '/$mirror/user/$id'
    | '/$mirror/view/$id'
  id:
    | '__root__'
    | '/'
    | '/_proxy'
    | '/health'
    | '/_proxy/$mirror/'
    | '/_proxy/$mirror/user/$id'
    | '/_proxy/$mirror/view/$id'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexRoute: typeof IndexRoute
  ProxyRoute: typeof ProxyRouteWithChildren
  HealthRoute: typeof HealthRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexRoute: IndexRoute,
  ProxyRoute: ProxyRouteWithChildren,
  HealthRoute: HealthRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/_proxy",
        "/health"
      ]
    },
    "/": {
      "filePath": "index.tsx"
    },
    "/_proxy": {
      "filePath": "_proxy.tsx",
      "children": [
        "/_proxy/$mirror/",
        "/_proxy/$mirror/user/$id",
        "/_proxy/$mirror/view/$id"
      ]
    },
    "/health": {
      "filePath": "health.tsx"
    },
    "/_proxy/$mirror/": {
      "filePath": "_proxy/$mirror/index.tsx",
      "parent": "/_proxy"
    },
    "/_proxy/$mirror/user/$id": {
      "filePath": "_proxy/$mirror/user.$id.tsx",
      "parent": "/_proxy"
    },
    "/_proxy/$mirror/view/$id": {
      "filePath": "_proxy/$mirror/view.$id.tsx",
      "parent": "/_proxy"
    }
  }
}
ROUTE_MANIFEST_END */
