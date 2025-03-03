import { writable } from 'svelte/store';

export const backgroundImage = writable(null);
export const averageColor = writable(null);

export function loadState() {
  try {
    const serializedState = localStorage.getItem('whosin-state');
    if (serializedState === null) {
      return undefined;
    }
    return JSON.parse(serializedState);
  } catch (err) {
    return undefined;
  }
}

export function saveState(state) {
  try {
    const serializedState = JSON.stringify(state);
    localStorage.setItem('whosin-state', serializedState);
  } catch {
    // ignore write errors
  }
}

export function setBackgroundImage(imageUrl) {
  console.log('set background image', imageUrl);
    saveState({ 
      ...loadState(),
        backgroundImage: imageUrl,
    });
    backgroundImage.set(imageUrl);
}

export function getBackgroundImage() {
    const state = loadState();
    if (state && state.backgroundImage) {
        backgroundImage.set(state.backgroundImage);
    }
    return state?.backgroundImage || null;
}

export function setAverageColor(color) {
  saveState({ 
    ...loadState(),
      averageColor: color,
  });
  averageColor.set(color);
}

export function getAverageColor() {
  const state = loadState();
  if (state && state.averageColor) {
      averageColor.set(state.averageColor);
  }
  return state?.averageColor || null;
}