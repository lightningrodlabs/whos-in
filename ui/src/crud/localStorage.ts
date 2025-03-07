import { writable } from 'svelte/store';
import { navigate } from '../store';

export const backgroundImage = writable(null);
export const averageColor = writable(null);
export const colorPalette = writable(null);
// export const storedView = writable(null);

export function loadState() {
  try {
    const serializedState = localStorage.getItem('whosin-state');
    if (serializedState === null) {
      return undefined;
    }
    // get background image and average color from state
    const state = JSON.parse(serializedState);
    backgroundImage.set(state?.backgroundImage || null);
    averageColor.set(state?.averageColor || null);
    colorPalette.set(state?.colorPalette || null);
    console.log("found stored view", state?.storedView);
    navigate((state?.storedView?.view || null), (state?.storedView?.hash || null));
    // storedView.set(state?.storedView || null);
    return JSON.parse(serializedState);
  } catch (err) {
    return undefined;
  }
}

export function returnState() {
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
      ...returnState(),
        backgroundImage: imageUrl,
    });
    backgroundImage.set(imageUrl);
}

export function setAverageColor(color) {
  saveState({ 
    ...returnState(),
      averageColor: color,
  });
  averageColor.set(color);
}

export function setCurrentView(view) {
  saveState({ 
    ...returnState(),
      storedView: view,
  });
  // storedView.set(view);
}

export function setColorPalette(palette) {
  saveState({ 
    ...returnState(),
      colorPalette: palette,
  });
  colorPalette.set(palette);
}