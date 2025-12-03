import { Routes } from '@angular/router';

export const routes: Routes = [
  {
    path: '',
    loadComponent: () => import('./pages/home/home.component').then(m => m.HomeComponent),
    title: 'NanoPDF - High-Performance PDF Library'
  },
  {
    path: 'docs',
    children: [
      {
        path: '',
        loadComponent: () => import('./pages/docs/docs.component').then(m => m.DocsComponent),
        title: 'Documentation - NanoPDF'
      },
      {
        path: 'getting-started',
        loadComponent: () => import('./pages/docs/getting-started/getting-started.component').then(m => m.GettingStartedComponent),
        title: 'Getting Started - NanoPDF'
      },
      {
        path: 'rust',
        loadComponent: () => import('./pages/docs/rust/rust.component').then(m => m.RustComponent),
        title: 'Rust Documentation - NanoPDF'
      },
      {
        path: 'javascript',
        loadComponent: () => import('./pages/docs/javascript/javascript.component').then(m => m.JavascriptComponent),
        title: 'JavaScript Documentation - NanoPDF'
      },
      {
        path: 'go',
        loadComponent: () => import('./pages/docs/go/go.component').then(m => m.GoComponent),
        title: 'Go Documentation - NanoPDF'
      }
    ]
  },
  {
    path: 'api',
    loadComponent: () => import('./pages/api/api.component').then(m => m.ApiComponent),
    title: 'API Reference - NanoPDF'
  },
  {
    path: 'benchmarks',
    loadComponent: () => import('./pages/benchmarks/benchmarks.component').then(m => m.BenchmarksComponent),
    title: 'Benchmarks - NanoPDF'
  },
  {
    path: '**',
    redirectTo: ''
  }
];
