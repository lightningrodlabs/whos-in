import { writable } from 'svelte/store';

export const view = writable("home");
export const viewHash = writable(new Uint8Array([]));
export const notifications = writable([]);

export function navigate(location, hash) {
    view.update(v => location);
    viewHash.update(h => hash)
}

export function notifications_update(new_notifications) {
    notifications.update(v => new_notifications)
}