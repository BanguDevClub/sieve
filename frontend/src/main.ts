import './app.css';
import App from './App.svelte';
import { mount } from 'svelte';

const appElement = document.getElementById('app');

if (!appElement) {
  throw new Error('Failed to find the #app element');
}

const app = mount(App, {
  target: appElement,
});

export default app;
